//! 树堆
//!
//! 参考:
//!
//! - [Treap - OI wiki](https://oi-wiki.org/ds/treap)
//! - [数据结构-Treap树/替罪羊树](https://yanglei253.github.io/2020/07/03/dataStructure/dataStructure-treaptree/)
//!
//! 树堆是一种弱平衡二叉搜索树，结合了二叉搜索树和堆的性质  
//! 每个节点上有两个值:
//!
//! - 树上的值: 节点本身的值，维护二叉搜索树的性质
//! - 堆上的值: 随机生成出来的，维护堆的性质
//!
//! Treap 有旋转和无旋两种实现方式，这里只写有旋的实现  
//! 旋转 Treap 的实现方式如下：
//!
//! - 插入时，首先按照正常 BST 的方式找到对应的位置
//! - 创建新节点时随机生成一个值 priority
//! - 比较新节点及其父节点的 priority 如果新节点的 priority 比父节点小则通过旋转更新父节点
//!
//! - 删除时，首先按照正常 BST 的方式找到需要删除的值
//! - 通过旋转的方式，将待删除的值旋转到叶子节点
//! - 直接删除叶子节点即可
//!
//! 实际上树堆依赖了 BST 的一个性质即左旋和右旋任意节点后仍会是一棵合法的 BST  
//! 利用此性质可以很方便的执行堆化(堆化也就是节点上浮和下沉两种操作对应旋转)

use rand::Rng;

use std::ptr::NonNull;

/// Treap 节点
pub struct Node<T> {
    value: T,      // 树上值
    priority: u64, // 优先级即堆上值(随机获取)
    left: Option<NonNull<Node<T>>>,
    right: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, priority: u64) -> Self {
        Self {
            value,
            priority,
            left: None,
            right: None,
        }
    }

    pub fn depth(&self) -> usize {
        let depth_left = self
            .left
            .map_or(0, |left_ptr| unsafe { left_ptr.as_ref() }.depth());

        let depth_right = self
            .right
            .map_or(0, |right_ptr| unsafe { right_ptr.as_ref() }.depth());

        1.max(depth_left + 1).max(depth_right + 1)
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

enum Dir {
    Left,
    Right,
}

/// Treap 树堆
#[derive(Default)]
pub struct Treap<T> {
    rng: rand::rngs::ThreadRng,
    root: Option<NonNull<Node<T>>>,
}

impl<T: Ord + Eq> Treap<T> {
    /// 创建新的树堆
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            root: None,
        }
    }

    /// 返回树深度(高度)
    pub fn depth(&self) -> usize {
        self.root
            .map_or(0, |root_ptr| unsafe { root_ptr.as_ref() }.depth())
    }

    fn new_node(&mut self, value: T) -> NonNull<Node<T>> {
        let priority = self.rng.gen();
        let new_node = Box::new(Node::new(value, priority));
        unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) }
    }

    /// Treap 插入值
    pub fn insert(&mut self, value: T) {
        let mut root_ptr = match self.root {
            None => {
                let new_node_ptr = self.new_node(value);
                self.root = Some(new_node_ptr);
                return;
            }
            Some(root_ptr) => root_ptr,
        };

        let mut parents = vec![];
        let mut curr_ptr = root_ptr;
        let mut curr_node = unsafe { root_ptr.as_mut() };

        loop {
            match curr_node.value.cmp(&value) {
                // 树上已经有重复值
                std::cmp::Ordering::Equal => return,
                std::cmp::Ordering::Less => match curr_node.right {
                    None => {
                        let new_node_ptr = self.new_node(value);
                        curr_node.right = Some(new_node_ptr);
                        parents.push((curr_ptr, Dir::Right));
                        break;
                    }
                    Some(right_ptr) => {
                        parents.push((curr_ptr, Dir::Right));
                        curr_ptr = right_ptr;
                        curr_node = unsafe { curr_ptr.as_mut() };
                    }
                },
                std::cmp::Ordering::Greater => match curr_node.left {
                    None => {
                        let new_node_ptr = self.new_node(value);
                        curr_node.left = Some(new_node_ptr);
                        parents.push((curr_ptr, Dir::Left));
                        break;
                    }
                    Some(left_ptr) => {
                        parents.push((curr_ptr, Dir::Left));
                        curr_ptr = left_ptr;
                        curr_node = unsafe { curr_ptr.as_mut() }
                    }
                },
            }
        }

        let (mut parent_ptr, mut dir) = parents.pop().unwrap();
        let mut parent_node = unsafe { parent_ptr.as_ref() };

        loop {
            let new_node_ptr = match dir {
                Dir::Left => {
                    let child_ptr = parent_node.left.unwrap();
                    let child_node = unsafe { child_ptr.as_ref() };
                    if child_node.priority >= parent_node.priority {
                        return;
                    } else {
                        rotate_right(parent_ptr)
                    }
                }
                Dir::Right => {
                    let child_ptr = parent_node.right.unwrap();
                    let child_node = unsafe { child_ptr.as_ref() };
                    if child_node.priority >= parent_node.priority {
                        return;
                    } else {
                        rotate_left(parent_ptr)
                    }
                }
            };

            match parents.pop() {
                None => {
                    self.root = Some(new_node_ptr);
                    return;
                }
                Some((mut pp_ptr, pp_dir)) => {
                    let pp_node = unsafe { pp_ptr.as_mut() };
                    match pp_dir {
                        Dir::Left => pp_node.left = Some(new_node_ptr),
                        Dir::Right => pp_node.right = Some(new_node_ptr),
                    }

                    (parent_ptr, parent_node, dir) = (pp_ptr, pp_node, pp_dir);
                }
            }
        }
    }

    /// Treap 删除值
    pub fn delete(&mut self, value: &T) -> Option<T> {
        let root_ptr = match self.root {
            None => return None,
            Some(root_ptr) => root_ptr,
        };

        let mut parent = None;
        let mut curr_ptr = root_ptr;
        let mut curr_node = unsafe { curr_ptr.as_mut() };

        loop {
            match curr_node.value.cmp(value) {
                std::cmp::Ordering::Less => match curr_node.right {
                    None => return None,
                    Some(right_ptr) => {
                        parent = Some((curr_ptr, Dir::Right));
                        curr_ptr = right_ptr;
                        curr_node = unsafe { curr_ptr.as_mut() };
                    }
                },
                std::cmp::Ordering::Greater => match curr_node.left {
                    None => return None,
                    Some(left_ptr) => {
                        parent = Some((curr_ptr, Dir::Left));
                        curr_ptr = left_ptr;
                        curr_node = unsafe { curr_ptr.as_mut() };
                    }
                },
                std::cmp::Ordering::Equal => {
                    loop {
                        let single_child = if curr_node.left.is_none() {
                            Some(curr_node.right.take())
                        } else if curr_node.right.is_none() {
                            Some(curr_node.left.take())
                        } else {
                            None
                        };

                        if let Some(child) = single_child {
                            match parent {
                                None => {
                                    self.root = child;
                                }
                                Some((mut p_ptr, dir)) => {
                                    let parent_node = unsafe { p_ptr.as_mut() };
                                    match dir {
                                        Dir::Left => parent_node.left = child,
                                        Dir::Right => parent_node.right = child,
                                    }
                                }
                            }

                            let node = unsafe { Box::from_raw(curr_node) };
                            return Some(node.value);
                        }

                        // 右旋到下一层更新父节点
                        let new_parent = rotate_right(curr_ptr);
                        curr_ptr = unsafe { new_parent.as_ref() }.right.unwrap();
                        curr_node = unsafe { curr_ptr.as_mut() };
                        parent = Some((new_parent, Dir::Right));
                    }
                }
            }
        }
    }
}

/// 节点左旋
pub fn rotate_left<T>(mut old_root_ptr: NonNull<Node<T>>) -> NonNull<Node<T>> {
    let old_root = unsafe { old_root_ptr.as_mut() };
    let mut new_root_ptr = old_root
        .right
        .take()
        .expect("rotate_left: root.right must non-nil");

    let new_root = unsafe { new_root_ptr.as_mut() };
    old_root.right = new_root.left.take();
    new_root.left = Some(old_root_ptr);

    new_root_ptr
}

/// 节点右旋
pub fn rotate_right<T>(mut old_root_ptr: NonNull<Node<T>>) -> NonNull<Node<T>> {
    let old_root = unsafe { old_root_ptr.as_mut() };
    let mut new_root_ptr = old_root
        .left
        .take()
        .expect("rotate_right: root.left must non-nil");

    let new_root = unsafe { new_root_ptr.as_mut() };
    old_root.left = new_root.right.take();
    new_root.right = Some(old_root_ptr);

    new_root_ptr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_treap_insert() {
        let mut t = Treap::new();
        for i in 0..128 {
            t.insert(i);
        }

        println!("depth: {}", t.depth());
    }

    #[test]
    fn test_treap_delete() {
        const N: usize = 64;
        let mut t = Treap::new();
        for i in 0..N {
            t.insert(i);
        }

        for i in 0..N {
            assert!(t.delete(&i).is_some());
            println!("depth: {}", t.depth());
        }

        for i in 0..N {
            assert!(t.delete(&i).is_none());
        }

        assert_eq!(t.depth(), 0);
    }
}
