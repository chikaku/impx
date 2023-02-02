//! 二叉搜索树

use std::cmp::Ordering;

/// 二叉搜索树节点
pub struct BinarySearchNode<T> {
    pub value: (T, usize),
    pub left: Option<Box<BinarySearchNode<T>>>,
    pub right: Option<Box<BinarySearchNode<T>>>,
}

/// 二叉搜索树
pub struct BinarySearchTree<T> {
    pub root: Option<BinarySearchNode<T>>,
}

impl<T: Ord> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    /// 插入元素
    pub fn insert(&mut self, value: T) {
        match &mut self.root {
            None => self.root = Some(BinarySearchNode::new(value)),
            Some(node) => node.insert(value),
        }
    }

    /// 查找元素是否存在
    pub fn find(&self, value: &T) -> bool {
        self.root
            .as_ref()
            .map(|node| node.find(value))
            .unwrap_or_default()
    }

    /// 树中最大元素
    pub fn max(&self) -> Option<&T> {
        self.root.as_ref().map(|node| node.max())
    }

    /// 树中最小元素
    pub fn min(&self) -> Option<&T> {
        self.root.as_ref().map(|node| node.min())
    }
}

impl<T: Ord> BinarySearchNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: (value, 1),
            left: None,
            right: None,
        }
    }

    pub fn find(&self, v: &T) -> bool {
        match self.value.0.cmp(v) {
            Ordering::Equal => true,
            Ordering::Less => self
                .right
                .as_ref()
                .map(|node| node.find(v))
                .unwrap_or_default(),
            Ordering::Greater => self
                .left
                .as_ref()
                .map(|node| node.find(v))
                .unwrap_or_default(),
        }
    }

    pub fn insert(&mut self, v: T) {
        match self.value.0.cmp(&v) {
            Ordering::Equal => {
                self.value.1 += 1;
            }
            Ordering::Less => match &mut self.right {
                None => {
                    self.right = Some(Box::new(Self::new(v)));
                }
                Some(node) => {
                    node.insert(v);
                }
            },
            Ordering::Greater => match &mut self.left {
                None => {
                    self.left = Some(Box::new(Self::new(v)));
                }
                Some(node) => {
                    node.insert(v);
                }
            },
        }
    }

    pub fn min(&self) -> &T {
        let mut node = self;
        while let Some(child) = &node.left {
            node = child.as_ref();
        }

        &node.value.0
    }

    pub fn max(&self) -> &T {
        let mut node = self;
        while let Some(child) = &node.right {
            node = child.as_ref();
        }

        &node.value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_tree() {
        let mut t = BinarySearchTree::new();
        t.insert(1);
        t.insert(9);
        t.insert(2);
        t.insert(8);

        assert!(t.find(&2));
        assert!(t.find(&9));
        assert!(!t.find(&7));

        assert_eq!(t.max(), Some(&9));
        assert_eq!(t.min(), Some(&1));
    }
}
