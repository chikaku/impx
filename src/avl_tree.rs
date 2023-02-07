//! AVL 树
//!
//! - [OI Wiki - AVL 树](https://oi-wiki.org/ds/avl/)
//! - [详解 AVL](https://zhuanlan.zhihu.com/p/34899732)
//! - [AVL Tree](https://www.javatpoint.com/avl-tree)
//! - [Insertion in an AVL Tree](https://www.geeksforgeeks.org/insertion-in-an-avl-tree/)
//! - [AVL Tree 可视化](https://www.cs.usfca.edu/~galles/visualization/AVLtree.html)
//!
//! AVL 树的性质很简单: 左右子树的高度差不超过 1 因此在插入和删除的过程中需要重新平衡
//!
//! 插入: 按照二叉搜索树的查找顺序, 找到合适的节点直接插入接着向上递归重新平衡  
//! 删除: 找到对应节点, 从节点右子树中找到最小值替换到当前被删掉的节点重新平衡子树  
//!
//! 定义平衡因子factor: 左子树高度减去右子树高度, 则有:
//!
//! - root.factor > 0: 左子树比右子树高
//! - root.factor < 0: 右子树比左子树高
//!
//! root.factor > 1: 左子树比右子树高两层需要平衡:
//!
//! - root.left.factor > 0: 左子树的左子树更高 (LL)
//! - root.left.factor <= 0: 左子树的右子树更高 (LR)
//!
//! root.factor < -1: 右子树比左子树高两层需要平衡:
//!
//! - root.right.factor < 0: 右子树的右子树更高 (RR)
//! - root.right.factor >= 0: 右子树的左子树更高 (RL)
//!
//! 各种情况的平衡方式:
//!
//! - 对于 LL 的情况右旋一次即可; 对于 RR 的情况左旋一次即可
//! - 对于 LR 的情况(下图1)对左子树执行一次左旋变成 LL 再右旋一次即可
//! - 对于 RL 的情况(下图2)对右子树执行一次右旋变成 RR 再左旋一次即可
//!
//! ```text
//!      z                               z                           x
//!     / \                            /   \                        /  \
//!    y   T4  Left Rotate (y)        x    T4  Right Rotate(z)    y      z
//!   / \      - - - - - - - - ->    /  \      - - - - - - - ->  / \    / \
//! T1   x                          y    T3                    T1  T2 T3  T4
//!     / \                        / \
//!   T2   T3                    T1   T2
//!
//!    z                            z                            x
//!   / \                          / \                          /  \
//! T1   y   Right Rotate (y)    T1   x      Left Rotate(z)   z      y
//!     / \  - - - - - - - - ->     /  \   - - - - - - - ->  / \    / \
//!    x   T4                      T2   y                  T1  T2  T3  T4
//!   / \                              /  \
//! T2   T3                           T3   T4
//! ```
//!
//! TODO: 在插入删除过程中来来回回有很多 `Box` 的 wrap 考虑怎么处理
use std::{cmp::Ordering, fmt::Debug};

/// AVL 树
pub struct AVLTree<T> {
    root: Option<AVLNode<T>>,
}

/// AVL 树节点
pub struct AVLNode<T> {
    value: T,
    height: usize,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>,
}

impl<T: Ord> AVLNode<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            height: 0,
            left: None,
            right: None,
        }
    }

    /// 平衡因子
    fn balance_factor(&self) -> isize {
        let hl = self.left.as_ref().map(|t| t.height).unwrap_or_default() as isize;
        let hr = self.right.as_ref().map(|t| t.height).unwrap_or_default() as isize;

        hl - hr
    }

    /// 重置树高度
    fn reset_height(&mut self) {
        let hl = self.left.as_ref().map(|t| t.height).unwrap_or_default();
        let hr = self.right.as_ref().map(|t| t.height).unwrap_or_default();
        self.height = hl.max(hr) + 1;
    }
}

/// AVL 树中插入值
pub fn insert<T: Ord>(mut root: AVLNode<T>, value: T) -> AVLNode<T> {
    match root.value.cmp(&value) {
        Ordering::Equal => return root,
        Ordering::Greater => match root.left.take() {
            None => {
                root.left = Some(Box::new(AVLNode::new(value)));
            }
            Some(node) => {
                root.left = Some(Box::new(insert(*node, value)));
            }
        },
        Ordering::Less => match root.right.take() {
            None => {
                root.right = Some(Box::new(AVLNode::new(value)));
            }
            Some(node) => {
                root.right = Some(Box::new(insert(*node, value)));
            }
        },
    };

    root = rebalance(root);
    if let Some(left) = &mut root.left {
        left.reset_height();
    }
    if let Some(right) = &mut root.right {
        right.reset_height();
    }
    root.reset_height();

    root
}

/// AVL 树中删除值
pub fn delete<T: Ord>(mut root: AVLNode<T>, value: &T) -> Option<Box<AVLNode<T>>> {
    match root.value.cmp(value) {
        Ordering::Equal => {
            if let Some(right) = root.right {
                // 如果右子树存在, 从右子树中找到一个最小值替换到当前节点
                let (value, right) = take_min(*right);
                root.value = value;
                root.right = right;
                root = rebalance(root);
                root.reset_height();
                Some(Box::new(root))
            } else {
                // 否则直接返回左节点即可
                root.left
            }
        }
        Ordering::Less => {
            if let Some(right) = root.right {
                root.right = delete(*right, value);
                root.reset_height();
            }

            Some(Box::new(root))
        }
        Ordering::Greater => {
            if let Some(left) = root.left {
                root.left = delete(*left, value);
                root.reset_height();
            }

            Some(Box::new(root))
        }
    }
}

/// AVL 树重新平衡
pub fn rebalance<T: Ord>(mut root: AVLNode<T>) -> AVLNode<T> {
    let factor = root.balance_factor();

    if factor > 1 {
        let left_factor = root
            .left
            .as_ref()
            .expect("left subtree must exist when factor > 1")
            .balance_factor();

        if left_factor > 0 {
            // LL
            root = rotate_right(root);
        } else {
            // LR
            root.left = root.left.map(|node| Box::new(rotate_left(*node)));
            root = rotate_right(root);
        }

        return root;
    }

    if factor < -1 {
        let right_factor = root
            .right
            .as_ref()
            .expect("right subtree must exist when factor < -1")
            .balance_factor();

        if right_factor < 0 {
            // RR
            root = rotate_left(root);
        } else {
            // RL
            root.right = root.right.map(|node| Box::new(rotate_right(*node)));
            root = rotate_left(root);
        }

        return root;
    }

    root
}

fn take_min<T: Ord>(mut root: AVLNode<T>) -> (T, Option<Box<AVLNode<T>>>) {
    if let Some(left) = root.left {
        let (value, right) = take_min(*left);
        root.left = right;
        root = rebalance(root);
        root.reset_height();
        (value, Some(Box::new(root)))
    } else {
        (root.value, root.right.take())
    }
}

/// 左旋
///
/// ```text
///   z                                y
///  /  \                            /   \
/// T1   y     Left Rotate(z)       z      x
///     /  \   - - - - - - - ->    / \    / \
///    T2   x                     T1  T2 T3  T4
///        / \
///      T3  T4
/// ```
pub fn rotate_left<T>(mut node: AVLNode<T>) -> AVLNode<T> {
    let right = node.right.take();
    let mut right = *right.unwrap();

    node.right = right.left.take();
    right.left = Some(Box::new(node));

    right
}

/// 右旋
///
/// ```text
///        z                                      y
///       / \                                   /   \
///      y   T4      Right Rotate (z)          x      z
///     / \          - - - - - - - - ->      /  \    /  \
///    x   T3                               T1  T2  T3  T4
///   / \
/// T1   T2
/// ```
pub fn rotate_right<T>(mut node: AVLNode<T>) -> AVLNode<T> {
    let left = node.left.take();
    let mut left = *left.unwrap();

    node.left = left.right.take();
    left.right = Some(Box::new(node));

    left
}

impl<T: Debug> AVLNode<T> {
    pub fn show(&self, level: usize) -> String {
        let mut res = format!("{:?}\n", self.value);
        if let Some(left) = &self.left {
            res.push_str(&"  ".repeat(level));
            res.push_str("L: ");
            res.push_str(&left.show(level + 1));
        }
        if let Some(right) = &self.right {
            res.push_str(&"  ".repeat(level));
            res.push_str("R: ");
            res.push_str(&right.show(level + 1));
        }

        res
    }
}

impl<T: Ord + Debug> AVLTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn insert(&mut self, value: T) {
        match self.root.take() {
            None => {
                self.root = Some(AVLNode::new(value));
            }
            Some(node) => {
                let root = insert(node, value);
                self.root = Some(root);
            }
        }
    }

    pub fn delete(&mut self, value: &T) {
        if let Some(node) = self.root.take() {
            self.root = delete(node, value).map(|node| *node);
        }
    }
}

impl<T: Ord + Debug> Default for AVLTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> Debug for AVLTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.root {
            None => writeln!(f, "None"),
            Some(root) => writeln!(f, "{}", root.show(0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avl_tree() {
        let mut t = AVLTree::new();

        t.insert(1);
        t.insert(8);
        t.insert(2);
        t.insert(6);
        t.insert(3);
        t.insert(4);
        t.insert(9);
        t.insert(7);

        assert_eq!(t.root.as_ref().map(|node| node.value), Some(3));
        println!("{:?}", t);

        t.delete(&8);
        t.delete(&2);
        t.delete(&3);

        assert_eq!(t.root.as_ref().map(|node| node.value), Some(6));
        println!("{:?}", t);
    }
}
