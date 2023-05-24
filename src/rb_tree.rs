//! 红黑树
//!
//! - [教你透彻了解红黑树](https://github.com/julycoding/The-Art-Of-Programming-By-July-2nd/blob/master/ebook/zh/03.01.md)
//! - [visualization RedBlackTree](https://www.cs.usfca.edu/~galles/visualization/RedBlack.html)
//! - [wikipedia - Red–black_tree](https://en.wikipedia.org/wiki/Red%E2%80%93black_tree)
//!
//! 红黑树的性质:(和传统的叶子结点定义不同, 这里把空节点当作叶子结点)
//!
//! - 规则1: 每个节点的颜色要么是红色要么是黑色
//! - 规则2: 空结点是黑色
//! - 规则3: 红色节点的子节点都是黑色的
//! - 规则4: 任意节点到叶结点上所有路径上黑色节点数量相等

use std::{fmt::Debug, ptr::NonNull};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Left,
    Right,
}

pub struct Node<T> {
    color: Color,
    value: T,
    left: Option<NonNull<Node<T>>>,
    right: Option<NonNull<Node<T>>>,
    parent: Option<(NonNull<Node<T>>, Dir)>,
}

type NodePtr<T> = NonNull<Node<T>>;

impl<T> Node<T> {
    fn new(value: T, color: Color) -> Self {
        Self {
            value,
            color,
            left: None,
            right: None,
            parent: None,
        }
    }
}

impl<T: Debug> Node<T> {
    pub fn depth(&self) -> usize {
        let left_depth = self
            .left
            .map(|mut node| unsafe { node.as_mut() }.depth())
            .unwrap_or(1);

        let right_depth = self
            .right
            .map(|mut node| unsafe { node.as_mut() }.depth())
            .unwrap_or(1);

        debug_assert_eq!(left_depth, right_depth, "{:#?}", self);
        if self.color == Color::Black {
            left_depth + 1
        } else {
            left_depth
        }
    }
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("color", &self.color)
            .field("value", &self.value)
            .field("left", &self.left.map(|node| unsafe { node.as_ref() }))
            .field("right", &self.right.map(|node| unsafe { node.as_ref() }))
            .field("parent", &self.parent)
            .finish()
    }
}

fn colorof<T>(node: Option<NodePtr<T>>) -> Color {
    node.map(|mut ptr| unsafe { ptr.as_mut() }.color)
        .unwrap_or(Color::Black)
}

fn other(dir: &Dir) -> Dir {
    match dir {
        Dir::Left => Dir::Right,
        Dir::Right => Dir::Left,
    }
}

fn childof<T>(node: &Node<T>, dir: Dir) -> &Option<NonNull<Node<T>>> {
    match dir {
        Dir::Left => &node.left,
        Dir::Right => &node.right,
    }
}

type SiblingPair<T> = (Option<NodePtr<T>>, Option<NodePtr<T>>, Option<NodePtr<T>>);

fn sibling<T>(node: &Node<T>) -> SiblingPair<T> {
    let (parent_ptr, dir) = node.parent.expect("parent node must exist");
    let parent = unsafe { parent_ptr.as_ref() };

    match other(&dir) {
        Dir::Left => parent
            .left
            .map(|ptr| {
                let node = unsafe { ptr.as_ref() };
                (Some(ptr), node.right, node.left)
            })
            .unwrap_or_default(),
        Dir::Right => parent
            .right
            .map(|ptr| {
                let node = unsafe { ptr.as_ref() };
                (Some(ptr), node.left, node.right)
            })
            .unwrap_or_default(),
    }
}

/// 旋转操作
/// 由于每个节点上保存了父节点指针, 需要修复节点的父指针
pub fn rotate<T>(mut node_ptr: NonNull<Node<T>>, dir: &Dir) -> NonNull<Node<T>> {
    let node = unsafe { node_ptr.as_mut() };

    match dir {
        Dir::Left => {
            let mut right_ptr = node.right.expect("rorate left: right child must exist");
            let right = unsafe { right_ptr.as_mut() };

            // 根节点左旋, 修改其父节点为右子节点, 并拿到之前父节点
            let old_parent = node.parent.replace((right_ptr, Dir::Left));
            if let Some((mut parent_ptr, dir)) = old_parent {
                // 如果之前的父节点不为空需要根据指向替换其子节点
                let parent = unsafe { parent_ptr.as_mut() };
                match dir {
                    Dir::Left => parent.left = Some(right_ptr),
                    Dir::Right => parent.right = Some(right_ptr),
                }
            }
            // 右子节点晋升为根节点后, 修改自己的父节点
            right.parent = old_parent;

            // 把旧的右子节点的左子树赋值给旧的根节点
            // 此子树的父节点也需要跟着修改为旧的根节点
            let right_left = right.left;
            if let Some(mut right_left_ptr) = right_left {
                let right_left = unsafe { right_left_ptr.as_mut() };
                right_left.parent = Some((node_ptr, Dir::Right));
            }
            node.right = right_left;

            // 把旧根节点设置成新的根节点的左子节点
            right.left = Some(node_ptr);
            right_ptr
        }
        Dir::Right => {
            let mut left_ptr = node.left.expect("rorate right: left child must exist");
            let left = unsafe { left_ptr.as_mut() };

            let old_parent = node.parent.replace((left_ptr, Dir::Right));
            if let Some((mut parent_ptr, dir)) = old_parent {
                let parent = unsafe { parent_ptr.as_mut() };
                match dir {
                    Dir::Left => parent.left = Some(left_ptr),
                    Dir::Right => parent.right = Some(left_ptr),
                }
            }
            left.parent = old_parent;

            let left_right = left.right;
            if let Some(mut left_right_ptr) = left_right {
                let left_right = unsafe { left_right_ptr.as_mut() };
                left_right.parent = Some((node_ptr, Dir::Left));
            }

            node.left = left_right;
            left.right = Some(node_ptr);
            left_ptr
        }
    }
}

pub struct RBTree<T> {
    root: Option<NonNull<Node<T>>>,
}

impl<T: Ord> RBTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    fn rotate2(&mut self, node_ptr: NodePtr<T>, dir: &Dir) {
        let is_root = unsafe { node_ptr.as_ref() }.parent.is_none();
        let new_ptr = rotate(node_ptr, dir);
        if is_root {
            self.root = Some(new_ptr)
        }
    }

    pub fn insert(&mut self, value: T) -> Option<T> {
        let mut parent_ptr = match self.root {
            None => {
                let node = Box::new(Node::new(value, Color::Black));
                let ptr = unsafe { NonNull::new_unchecked(Box::into_raw(node)) };
                self.root = Some(ptr);
                return None;
            }
            Some(node) => node,
        };

        // 先执行二叉搜索树插入流程
        let new_node_ptr = loop {
            let parent = unsafe { parent_ptr.as_mut() };
            match value.cmp(&parent.value) {
                std::cmp::Ordering::Equal => {
                    return Some(std::mem::replace(&mut parent.value, value));
                }
                std::cmp::Ordering::Less => match parent.left {
                    Some(left) => parent_ptr = left,
                    None => {
                        // 准备新的红色节点
                        let mut new_node = Box::new(Node::new(value, Color::Red));
                        new_node.parent = Some((parent_ptr, Dir::Left));
                        let new_node_raw = Box::into_raw(new_node);
                        let new_node_ptr = unsafe { NonNull::new_unchecked(new_node_raw) };

                        // 设置新节点的位置
                        parent.left = Some(new_node_ptr);
                        break new_node_ptr;
                    }
                },
                std::cmp::Ordering::Greater => match parent.right {
                    Some(right) => parent_ptr = right,
                    None => {
                        // 准备新的红色节点
                        let mut new_node = Box::new(Node::new(value, Color::Red));
                        new_node.parent = Some((parent_ptr, Dir::Right));
                        let new_node_raw = Box::into_raw(new_node);
                        let new_node_ptr = unsafe { NonNull::new_unchecked(new_node_raw) };

                        // 设置新节点的位置
                        parent.right = Some(new_node_ptr);
                        break new_node_ptr;
                    }
                },
            }
        };

        self.balance(new_node_ptr);
        None
    }

    /// 在红色节点上插入一个红色节点后的平衡
    fn balance(&mut self, mut node_ptr: NonNull<Node<T>>) {
        // N 表示当前节点
        // P 表示 N 的父节点
        // G 表示 P 的父节点即 N 的祖父节点
        // U 表示 G 的另一个子节点即 P 的兄弟节点 N 的叔父节点

        let mut node = unsafe { node_ptr.as_mut() };
        while let Some((mut parent_ptr, node_dir)) = &node.parent {
            let mut parent = unsafe { parent_ptr.as_mut() };

            match parent.color {
                // P 是黑色, 直接退出
                Color::Black => return,
                // P 是红色时, 继续观察 G
                Color::Red => match &parent.parent {
                    // 如果 G 为空表示 P 是根节点
                    // 由于 N 是红色, 根据规则3需要将 P 改为黑色
                    None => {
                        parent.color = Color::Black;
                        return;
                    }
                    // 如果 G 存在, 由于 P 是红色则则 G 一定是黑色
                    Some((mut grand_ptr, parent_dir)) => {
                        let grand = unsafe { grand_ptr.as_mut() };
                        debug_assert_eq!(grand.color, Color::Black);

                        // U 是红色, 则将 P 和 U 点改成黑色
                        // G 改成红色即可然后将 N = G 继续向上修复
                        //     G(⚫)           G(🔴)
                        //      /  \            / \
                        //  P(🔴) U(🔴) -->  P(⚫) U(⚫)
                        //    /                /
                        // N(🔴)            N(🔴)
                        if let Some(mut uncle_ptr) = childof(grand, other(parent_dir)) {
                            let uncle = unsafe { uncle_ptr.as_mut() };
                            if uncle.color == Color::Red {
                                uncle.color = Color::Black;
                                parent.color = Color::Black;
                                grand.color = Color::Red;
                                node = grand;
                                continue;
                            }
                        }

                        // 叔父节点是黑色(可能存在, 也可能不存在)
                        // 如果 N 和 P 的方向不同, 则需要将 P 左旋或右旋至相同的方向
                        // 然后将 N 和 P 交换
                        //     G(⚫)        G(⚫)
                        //      /             /
                        //  P(🔴)    -->  N(🔴)
                        //     \            /
                        //   N(🔴)       P(🔴)
                        if node_dir != parent_dir {
                            self.rotate2(parent_ptr, parent_dir);
                            parent = node;
                        }

                        // 如果 N 和 P 都是左子节点: 将 G 右旋
                        // 如果 N 和 P 都是右子节点: 将 G 左旋
                        // 接着修改 P 和 G 的颜色即可 G 有可能是根节点旋转完后要重置
                        //      G(⚫)          P(🔴)           P(⚫)
                        //       / \            /  \            /   \
                        //   P(🔴) U(⚫) -> N(🔴) G(⚫) -> N(🔴) G(🔴)
                        //     /                      \               \
                        //  N(🔴)                   U(⚫)            U(⚫)
                        self.rotate2(grand_ptr, &other(parent_dir));
                        parent.color = Color::Black;
                        grand.color = Color::Red;
                        return;
                    }
                },
            }
        }
    }

    /// 将一个新节点替换到指定节点的位置
    fn replace_child(&mut self, mut node: NodePtr<T>, new: Option<NodePtr<T>>) {
        match (unsafe { node.as_mut() }).parent {
            None => {
                self.root = new;
                if let Some(mut new_ptr) = new {
                    let new_node = unsafe { new_ptr.as_mut() };
                    new_node.parent = None;
                }
            }
            Some((mut parent, dir)) => {
                let parent_node = unsafe { parent.as_mut() };
                match dir {
                    Dir::Left => parent_node.left = new,
                    Dir::Right => parent_node.right = new,
                }

                if let Some(mut new_ptr) = new {
                    let new_node = unsafe { new_ptr.as_mut() };
                    new_node.parent = Some((parent, dir));
                }
            }
        }
    }

    /// 删除节点
    pub fn delete(&mut self, value: &T) -> Option<T> {
        // 先在二叉搜索树上找到需要删除的节点
        let mut curr_ptr = self.root;
        while let Some(mut node_ptr) = curr_ptr {
            let node = unsafe { node_ptr.as_mut() };
            match value.cmp(&node.value) {
                std::cmp::Ordering::Less => curr_ptr = node.left,
                std::cmp::Ordering::Greater => curr_ptr = node.right,
                std::cmp::Ordering::Equal => break,
            }
        }

        // 如果对应节点不存在直接返回
        let node = match curr_ptr {
            None => return None,
            Some(mut ptr) => unsafe { ptr.as_mut() },
        };

        // 经过和中间子节点的替换, 得到一个待删除的叶子节点
        let leaf_node = match (node.left, node.right) {
            // 对于叶子节点直接返回
            (None, None) => node,

            // 只有一个左子节点, 此子节点一定是红色的, 如果是黑色, 左右子树高度肯定会不一致
            // 因此当前节点也只能是黑色的, 则直接替换成子节点然后修改颜色即可(路径上总的黑色保持不变)
            //    N(⚫)           cl(⚫)
            //    /   \       ->   /  \
            // cl(🔴) nil(⚫)    ..   ..
            (Some(mut left_ptr), None) => {
                let left_node = unsafe { left_ptr.as_mut() };
                debug_assert_eq!(node.color, Color::Black);
                debug_assert_eq!(left_node.color, Color::Red);

                left_node.color = Color::Black;
                self.replace_child(node.into(), Some(left_ptr));
                return Some(unsafe { Box::from_raw(node) }.value);
            }

            // 只有一个右子节点此子节点一定是红色的, 如果是黑色, 左右子树高度肯定会不一致
            // 因此当前节点也只能是黑色的, 则直接替换成子节点然后修改颜色即可(路径上总的黑色保持不变)
            //     N(⚫)           cr(⚫)
            //    /    \       ->   /  \
            // nil(⚫) cr(🔴)     ..   ..
            (None, Some(mut right_ptr)) => {
                let right_node = unsafe { right_ptr.as_mut() };
                debug_assert_eq!(node.color, Color::Black);
                debug_assert_eq!(right_node.color, Color::Red);

                right_node.color = Color::Black;
                self.replace_child(node.into(), Some(right_ptr));
                return Some(unsafe { Box::from_raw(node) }.value);
            }

            // 左右子节点都非空, 找到右子树的最小节点(最左节点)进行替换(可以直接替换数据)
            // 此时整颗数在颜色上是平衡的
            // 最左节点不可能有左子树, 则要么是叶子节点, 要么是只有一个红色节点(原因同上)
            // 判断如果右子节点不为空按照以上的步骤处理
            // 如果右子节点为空则直接返回这个叶子节点
            //     N()
            //    /    \
            //  ..     ..
            //        /
            //      l(🔴)
            //      /    \
            //   nil(⚫)  ...
            (Some(_), Some(right_ptr)) => {
                let mut succ_ptr = right_ptr;
                while let Some(left_ptr) = unsafe { succ_ptr.as_mut() }.left {
                    succ_ptr = left_ptr;
                }

                let succ = unsafe { succ_ptr.as_mut() };
                std::mem::swap(&mut node.value, &mut succ.value);
                debug_assert!(succ.left.is_none());

                match succ.right {
                    None => succ,
                    Some(mut right_ptr) => {
                        let right_node = unsafe { right_ptr.as_mut() };
                        debug_assert_eq!(succ.color, Color::Black);
                        debug_assert_eq!(right_node.color, Color::Red);

                        right_node.color = Color::Black;
                        self.replace_child(succ_ptr, Some(right_ptr));
                        return Some(unsafe { Box::from_raw(succ) }.value);
                    }
                }
            }
        };

        // 红色叶子节点或者是根节点直接删除即可
        if leaf_node.color == Color::Red || leaf_node.parent.is_none() {
            self.replace_child(leaf_node.into(), None);
            return Some(unsafe { Box::from_raw(leaf_node) }.value);
        }

        self.delete_black_leaf(leaf_node.into())
    }

    /// 删除黑色叶子节点
    fn delete_black_leaf(&mut self, mut delete_node: NodePtr<T>) -> Option<T> {
        // 移除这个节点
        // N 表示当前节点
        // P 表示当前节点的父节点
        // S 表示当前节点的兄弟节点
        // C 表示 S 的子节点中靠近 N 的那个
        // D 表示 S 的子节点中远离 N 的那个
        //   P          P
        //  / \        / \
        // N   S      S   N
        //    / \    / \
        //   C   D  D  C

        let mut n = unsafe { delete_node.as_mut() };
        while let Some((mut p, dir)) = n.parent {
            // N 在传入时是一个黑色叶子节点, 在循环内部 N 有可能会被替换, 但是被替换的也是黑色节点
            let (mut s, mut c, mut d) = sibling(n);

            if colorof(s) == Color::Red {
                // 对应 wiki 中的 Case_D3
                // 如果 S 是红色, 且已知 N 是非空黑色节点
                // 为了保证路径上黑色节点数量一致 C 和 D 肯定是非空的黑色节点
                // 同时由于 S 是红色则 P 也只能是黑色
                //   P(⚫)
                //   /  \
                // N(⚫) S(🔴)
                //        / \
                //    C(⚫) D(⚫)
                debug_assert_eq!(colorof(c), Color::Black);
                debug_assert_eq!(colorof(d), Color::Black);
                debug_assert_eq!(colorof(p.into()), Color::Black);

                // 在 N 的方向上进行一次旋转并交换 S 和 P 颜色
                //      S(⚫)
                //       /  \
                //   P(🔴)  D(⚫)
                //    /  \
                // N(⚫) C(⚫)
                // 此时 N 的兄弟节点变成 C
                self.rotate2(p, &dir);
                unsafe { p.as_mut() }.color = Color::Red;
                unsafe { s.unwrap().as_mut() }.color = Color::Black;
                s = c;

                // 更新 C D 节点
                let snode = unsafe { s.unwrap().as_mut() };
                (c, d) = match dir {
                    Dir::Left => (snode.left, snode.right),
                    Dir::Right => (snode.right, snode.left),
                };
            }

            if colorof(c) == Color::Red {
                // 对应 wiki 中的 Case_D5
                //  P(B/R)
                //   /  \
                // N(⚫) S(⚫)
                //        / \
                //     C(🔴) D

                // 先把 S 向 dir 的反方向旋转再修改 C 和 S 的颜色
                //  P(B/R)
                //   /  \
                // N(⚫) C(⚫)
                //        / \
                //      D  S(🔴)
                self.rotate2(s.unwrap(), &other(&dir));
                unsafe { s.unwrap().as_mut() }.color = Color::Red;
                unsafe { c.unwrap().as_mut() }.color = Color::Black;
                d = s;
                s = c;
            }

            if colorof(d) == Color::Red {
                // 对应 wiki 中的 Case_D6
                //  P(B/R)
                //   /  \
                // N(⚫) S(⚫)
                //        /  \
                //      ..  D(🔴)

                // 将 P 向 N 的方向旋转
                //     S(⚫)
                //     /   \
                //  P(B/R) D(🔴)
                //   /      / \
                // N(⚫)

                // 将 S 的颜色修改为 P 的颜色 P 和 D 修改为黑色
                self.rotate2(p, &dir);
                unsafe { s.unwrap().as_mut() }.color = colorof(p.into());
                unsafe { d.unwrap().as_mut() }.color = Color::Black;
                unsafe { p.as_mut() }.color = Color::Black;
                break;
            }

            if colorof(p.into()) == Color::Red {
                // 对应 wiki 中的 Case_D4
                // C 和 D 都是黑色, 直接替换 P 和 S 的颜色即可
                //   P(🔴)            P(S)
                //   /  \            /    \
                // N(S) S(⚫)   ->  N(S) S(🔴)
                //       / \              /   \
                //   C(⚫) D(⚫)       C(⚫)  D(⚫)
                unsafe { p.as_mut() }.color = Color::Black;
                unsafe { s.unwrap().as_mut() }.color = Color::Red;
                break;
            }

            // 对应 wiki 中的 Case_D2
            // P S C D 全都是黑色则把 S 改成红色
            // 然后用 P 替换 N 递归向上
            //    P(⚫)
            //     /  \
            // N(⚫) S(⚫)
            //       /   \
            //    C(⚫) D(⚫)
            unsafe { s.unwrap().as_mut() }.color = Color::Red;
            n = unsafe { p.as_mut() };
        }

        self.replace_child(delete_node, None);
        let delete_node = unsafe { delete_node.as_mut() };
        Some(unsafe { Box::from_raw(delete_node) }.value)
    }
}

impl<T: Ord> Default for RBTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> RBTree<T> {
    pub fn depth(&self) -> usize {
        match self.root {
            None => 0,
            Some(node) => {
                let node = unsafe { node.as_ref() };
                node.depth()
            }
        }
    }
}

impl<T: Debug> Debug for RBTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut queue = match self.root {
            None => return writeln!(f, "NIL"),
            Some(root) => vec![root],
        };

        while !queue.is_empty() {
            let mut tmp = vec![];
            let line = queue
                .iter()
                .map(|node| {
                    let node = unsafe { node.as_ref() };
                    if let Some(x) = node.left {
                        tmp.push(x);
                    }
                    if let Some(x) = node.right {
                        tmp.push(x);
                    }

                    format!("{:?}({:?})", &node.value, node.color)
                })
                .collect::<Vec<String>>()
                .join(" -> ");
            writeln!(f, "{}", line)?;
            queue = tmp;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rb_tree_insert1() {
        let mut t = RBTree::new();
        assert!(t.insert(1).is_none());
        assert!(t.insert(2).is_none());
        assert!(t.insert(3).is_none());
        assert!(t.insert(4).is_none());
        assert!(t.insert(5).is_none());
        assert!(t.insert(6).is_none());
        assert!(t.insert(7).is_none());
        assert!(t.insert(8).is_none());
        println!("{:?}", t.depth());
        println!("{:?}", t);

        let mut t = RBTree::new();
        assert!(t.insert(5).is_none());
        assert!(t.insert(4).is_none());
        assert!(t.insert(3).is_none());
        assert!(t.insert(2).is_none());
        assert!(t.insert(1).is_none());
        println!("{:?}", t.depth());
        println!("{:?}", t);

        let mut t = RBTree::new();
        assert!(t.insert(1).is_none());
        assert!(t.insert(2).is_none());
        assert!(t.insert(3).is_none());
        assert!(t.insert(1).is_some());
        assert!(t.insert(2).is_some());
        assert!(t.insert(3).is_some());
        println!("{:?}", t.depth());
        println!("{:?}", t);
    }

    #[test]
    fn test_rb_tree_insert2() {
        let mut t = RBTree::new();
        for i in 1..=10000 {
            assert!(t.insert(i).is_none());
            t.depth();
        }

        let mut t = RBTree::new();
        for i in (1..1000).rev() {
            assert!(t.insert(i).is_none());
            t.depth();
        }

        use rand::Rng;
        let mut rng = rand::thread_rng();

        let mut t = RBTree::new();
        for _ in 1..1000 {
            t.insert(rng.gen_range(1..10000));
            t.depth();
        }
    }

    #[test]
    fn test_rb_tree_insert3() {
        let mut t = RBTree::new();
        t.insert(1468);
        t.insert(7127);
        t.insert(1810);
        t.insert(7151);
        t.insert(3101);
        t.insert(5739);
        t.insert(8951);
        t.insert(1545);
        t.insert(2848);
        t.insert(3339);

        t.depth();
    }

    #[test]
    fn test_rb_tree_delete1() {
        let mut t = RBTree::new();
        assert!(t.insert(1).is_none());
        assert!(t.insert(2).is_none());
        assert!(t.insert(3).is_none());
        assert!(t.insert(4).is_none());
        assert!(t.insert(5).is_none());
        assert!(t.insert(6).is_none());
        assert!(t.insert(7).is_none());
        assert!(t.insert(8).is_none());
        println!("{:?}", &t);

        assert_eq!(t.delete(&6), Some(6));
        assert_eq!(t.delete(&8), Some(8));
        assert_eq!(t.delete(&1), Some(1));
        println!("{:?}", &t);
    }

    #[test]
    fn test_rb_tree_delete2() {
        let mut t = RBTree::new();

        for i in 1..=1000 {
            assert!(t.insert(i).is_none());
            t.depth();
        }

        for i in (1..=1000).rev() {
            assert!(t.delete(&i).is_some());
            t.depth();
        }

        let mut t = RBTree::new();

        for i in 1..=10000 {
            t.insert(i);
            t.depth();
        }

        for i in 1..=500 {
            t.delete(&(i * 10));
            t.depth();
        }

        for i in (500..=1000).rev() {
            t.delete(&(i * 10));
            t.depth();
        }
    }

    #[test]
    fn test_rb_tree_depth() {
        for i in 2..15 {
            // 从上到下每一层最右节点分别是黑色/红色
            let mut t = RBTree::new();
            for i in 1..(1 << i) {
                assert!(t.insert(i).is_none());
            }

            assert_eq!(t.depth(), i);
        }

        for i in 2..15 {
            // 从上到下每一层最左节点分别是黑色/红色
            let mut t = RBTree::new();
            for i in (1..(1 << i)).rev() {
                assert!(t.insert(i).is_none());
            }

            assert_eq!(t.depth(), i);
        }
    }
}
