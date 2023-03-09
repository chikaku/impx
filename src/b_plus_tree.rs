//! B+ Tree
//!
//! 参考:
//!
//! - [visualization BPlusTree](https://www.cs.usfca.edu/~galles/visualization/BPlusTree.html)
//! - [b-plus-trees](https://www.scaler.com/topics/data-structures/b-plus-trees/)
//!
//! 对于阶数为 M 的 B+ 树有以下约束:
//!
//! - 每个节点上 key 的数目最多为 M-1
//! - 每个节点上 children 的数目最多为 M
//! - 每个节点上 key 的数目最少为 [(M+1)/2]-1
//! - 每个节点上 children 的数目最少为 [(M+1)/2]
//! - 实际上对于中间节点 children 的数目总是 key 数目 +1

use std::{fmt::Debug, iter::Zip, ptr::NonNull, slice::Iter};

pub struct BPlusTree<K, V> {
    order: usize,
    length: usize,
    root: Option<NonNull<Node<K, V>>>,
}

pub struct Node<K, V> {
    is_leaf: bool,                      // 是否叶子节点
    keys: Vec<K>,                       // 当前节点保存的键
    children: Vec<NonNull<Node<K, V>>>, // 当前(中间)节点的子节点列表
    values: Vec<V>,                     // 叶子节点保存的值
    next: Option<NonNull<Node<K, V>>>,  // 指向下一个叶子节点
}

impl<K: Ord + Copy + Debug, V> BPlusTree<K, V> {
    pub fn new(order: usize) -> Self {
        Self {
            order,
            length: 0,
            root: None,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn find(&self, key: &K) -> Option<(&K, &V)> {
        let mut node_ptr = match self.root {
            None => return None,
            Some(node) => node,
        };

        // 非叶子节点先查找对对应的叶子节点
        let mut node_ref = unsafe { node_ptr.as_ref() };
        while !node_ref.is_leaf {
            node_ptr = match node_ref.keys.binary_search(key) {
                Ok(index) => node_ref.children[index + 1],
                Err(index) => node_ref.children[index],
            };

            node_ref = unsafe { node_ptr.as_ref() };
        }

        // 在叶子节点上搜索值
        node_ref
            .keys
            .binary_search(key)
            .ok()
            .map(|index| (&node_ref.keys[index], &node_ref.values[index]))
    }

    pub fn insert(&mut self, entry: (K, V)) -> Option<(K, V)> {
        // 如果当前根节点为空先创建根节点
        let mut node_ptr = match self.root {
            Some(node) => node,
            None => {
                let mut node = Node::new(true);
                node.keys.push(entry.0);
                node.values.push(entry.1);

                let node_raw = Box::into_raw(Box::new(node));
                let node_ptr = unsafe { NonNull::new_unchecked(node_raw) };

                self.root = Some(node_ptr);
                self.length = 1;
                return None;
            }
        };

        let mut parents = vec![];
        let mut node_mut = unsafe { node_ptr.as_mut() };

        // 非叶子节点先查找对对应的叶子节点
        let key = &entry.0;
        while !node_mut.is_leaf {
            parents.push(node_ptr);
            node_ptr = match node_mut.keys.binary_search(key) {
                Ok(index) => node_mut.children[index + 1],
                Err(index) => node_mut.children[index],
            };

            node_mut = unsafe { node_ptr.as_mut() };
        }

        // 在叶子节点上先查找 key 是否已经存在
        // 如果存在直接替换出来, 否则按位置插入
        let old_entry = match node_mut.keys.binary_search(key) {
            Ok(index) => {
                let key = std::mem::replace(&mut node_mut.keys[index], entry.0);
                let val = std::mem::replace(&mut node_mut.values[index], entry.1);
                Some((key, val))
            }
            Err(pos) => {
                node_mut.keys.insert(pos, entry.0);
                node_mut.values.insert(pos, entry.1);
                self.length += 1;
                None
            }
        };

        // 如果叶子节点未满直接返回
        if node_mut.keys.len() < self.order {
            return old_entry;
        }

        debug_assert_eq!(node_mut.keys.len(), self.order);

        // 准备分裂出的叶子节点的数据
        let mid = node_mut.keys.len() / 2;
        let mut new_leaf = Node::new(true);
        new_leaf.keys = node_mut.keys.drain(mid..).collect();
        new_leaf.values = node_mut.values.drain(mid..).collect();
        new_leaf.next = node_mut.next.take();

        // 需要插入到上层的 key
        let mut new_key = new_leaf.keys[0];

        // 重置叶子节点 next 指针
        let raw = Box::into_raw(Box::new(new_leaf));
        let mut new_node_ptr = unsafe { NonNull::new_unchecked(raw) };
        node_mut.next = Some(new_node_ptr);

        // 旧节点(被分裂的节点)
        let mut old_node_ptr = node_ptr;

        // 根据 parents 自底向上插入新的节点
        // 如果中间节点也满了则继续分裂, 重置 old_node, new_node, new_key
        while let Some(mut parent_ptr) = parents.pop() {
            let parent_mut = unsafe { parent_ptr.as_mut() };
            let pos = parent_mut
                .keys
                .binary_search(&new_key)
                .unwrap_or_else(|i| i);

            parent_mut.children.remove(pos);
            parent_mut.keys.insert(pos, new_key);
            parent_mut.children.insert(pos, old_node_ptr);
            parent_mut.children.insert(pos + 1, new_node_ptr);
            if parent_mut.keys.len() < self.order {
                return old_entry;
            }

            debug_assert_eq!(parent_mut.keys.len(), self.order);

            // 分裂节点
            // key 分裂成三份, 左右各一份, 还有中间一个 key 提升到上一层
            // children 分裂为两份即可, 左右各保留一份
            let mid = parent_mut.keys.len() / 2;
            let mut new_inter_node = Node::new(false);
            new_inter_node.keys = parent_mut.keys.drain((mid + 1)..).collect();
            new_inter_node.children = parent_mut.children.drain((mid + 1)..).collect();

            // 把前半部分的最后一个分裂出来
            new_key = parent_mut.keys.pop().unwrap();
            old_node_ptr = parent_ptr;
            let raw = Box::into_raw(Box::new(new_inter_node));
            new_node_ptr = unsafe { NonNull::new_unchecked(raw) };
        }

        // 当所有中间节点满的情况下走到这里
        // 需要在根节点上分裂 old_node 即原本的根节点
        let mut new_root = Node::new(false);
        new_root.keys = vec![new_key];
        new_root.children = vec![old_node_ptr, new_node_ptr];

        let new_root_raw = Box::into_raw(Box::new(new_root));
        let new_root_ptr = unsafe { NonNull::new_unchecked(new_root_raw) };

        self.root.replace(new_root_ptr);
        old_entry
    }

    pub fn delete(&mut self, key: &K) -> Option<(K, V)> {
        let mut node_ptr = match self.root {
            None => return None,
            Some(node) => node,
        };

        // 先向下找到对应的叶子节点
        let mut parents = vec![];
        let mut node_mut = unsafe { node_ptr.as_mut() };
        while !node_mut.is_leaf {
            let pos = match node_mut.keys.binary_search(key) {
                Err(pos) => pos,
                Ok(index) => index + 1,
            };

            parents.push((node_ptr, pos));
            node_ptr = node_mut.children[pos];
            node_mut = unsafe { node_ptr.as_mut() };
        }

        // 叶子节点中搜索 key
        let index = match node_mut.keys.binary_search(key) {
            Err(_) => return None,
            Ok(index) => index,
        };

        // 在叶子节点中移除对应的 key-value
        let entry = (node_mut.keys.remove(index), node_mut.values.remove(index));
        self.length -= 1;

        // 如果节点数量满足
        let min_count = (self.order + 1) / 2 - 1;
        if node_mut.keys.len() >= min_count || parents.is_empty() {
            return Some(entry);
        }

        // 否则开始进行借取和合并
        // 1. 如果左节点有多出, 则可以借取
        // 2. 如果左节点有多出, 则可以借取
        // 3. 否则左右节点肯定都是刚好 min_count 个
        //    当前节点只有 min_count-1
        //    加起来一共 2*min_count-1
        //
        // 借取结束后需要更新父节点对应索引的 key 值
        // 合并结束后需要移除父节点上对应的 key 值
        // 父节点上 key 的数量可能会不满足约束
        // 所以合并的流程需要递归向上进行
        // 对于叶子节点需要借取 key value
        // 对于中间节点需要借取 key child

        while let Some((mut parent_ptr, index)) = parents.pop() {
            let parent_mut = unsafe { parent_ptr.as_mut() };

            // 尝试左兄弟节点
            if index > 0 {
                let mut left_sibling_ptr = parent_mut.children[index - 1];
                let left_sibling_mut = unsafe { left_sibling_ptr.as_mut() };
                if left_sibling_mut.keys.len() > min_count {
                    let left_last_key = left_sibling_mut.keys.pop().unwrap();
                    node_mut.keys.insert(0, left_last_key);
                    if let Some(left_last_value) = left_sibling_mut.values.pop() {
                        node_mut.values.insert(0, left_last_value);
                    }
                    if let Some(left_last_child) = left_sibling_mut.children.pop() {
                        node_mut.children.insert(0, left_last_child);
                    }

                    // 修改父节点对应索引 key
                    parent_mut.keys[index - 1] = node_mut.keys[0];
                    return Some(entry);
                }
            }

            // 尝试右兄弟节点
            if index + 1 < parent_mut.children.len() {
                let mut right_sibling_ptr = parent_mut.children[index + 1];
                let right_sibling_mut = unsafe { right_sibling_ptr.as_mut() };
                if right_sibling_mut.keys.len() > min_count {
                    let right_first_key = right_sibling_mut.keys.remove(0);
                    node_mut.keys.push(right_first_key);
                    if !right_sibling_mut.values.is_empty() {
                        let right_first_value = right_sibling_mut.values.remove(0);
                        node_mut.values.push(right_first_value);
                    }
                    if !right_sibling_mut.children.is_empty() {
                        let right_first_child = right_sibling_mut.children.remove(0);
                        node_mut.children.push(right_first_child);
                    }

                    // 修改父节点对应索引 key
                    parent_mut.keys[index] = right_sibling_mut.keys[0];
                    return Some(entry);
                }
            }

            // 合并到左兄弟节点
            if index > 0 {
                let mut left_sibling_ptr = parent_mut.children[index - 1];
                let left_sibling_mut = unsafe { left_sibling_ptr.as_mut() };

                // 先从父节点上将被合并节点的 key 和 child 删除
                let mid_key = parent_mut.keys.remove(index - 1);
                parent_mut.children.remove(index);
                if !left_sibling_mut.is_leaf {
                    // 如果是中间节点, 需要把上一级的 key 拿下来
                    // 作为两个子节点的中间 key
                    left_sibling_mut.keys.push(mid_key);
                }

                // 合并数据
                left_sibling_mut.keys.append(&mut node_mut.keys);
                left_sibling_mut.values.append(&mut node_mut.values);
                left_sibling_mut.children.append(&mut node_mut.children);
                left_sibling_mut.next = node_mut.next.take();

                // 把被合并节点删除
                let _drop_node = unsafe { Box::from_raw(node_mut) };

                // 如果父节点 key 数量满足约束则可以返回
                if parent_mut.keys.len() >= min_count {
                    return Some(entry);
                }

                // 否则递归向上继续
                node_mut = parent_mut;
                continue;
            }

            // 把右兄弟节点合并过来
            if index + 1 < parent_mut.children.len() {
                let mut right_sibling_ptr = parent_mut.children[index + 1];
                let right_sibling_mut = unsafe { right_sibling_ptr.as_mut() };

                // 先从父节点上被合并节点的 key 和 child 删除
                let mid_key = parent_mut.keys.remove(index);
                parent_mut.children.remove(index + 1);
                if !node_mut.is_leaf {
                    node_mut.keys.push(mid_key);
                }

                // 合并数据
                node_mut.keys.append(&mut right_sibling_mut.keys);
                node_mut.values.append(&mut right_sibling_mut.values);
                node_mut.children.append(&mut right_sibling_mut.children);
                node_mut.next = right_sibling_mut.next.take();

                // 把被合并节点删除
                let _drop_node = unsafe { Box::from_raw(right_sibling_mut) };

                // 如果父节点 key 数量满足约束则可以返回
                if parent_mut.keys.len() >= min_count {
                    return Some(entry);
                }

                // 否则递归向上继续
                node_mut = parent_mut;
                continue;
            }
        }

        // 一直合并到根节点, 则重置根节点
        if node_mut.keys.is_empty() {
            let new_root = node_mut.children.pop().unwrap();
            self.root = Some(new_root);

            // 把原节点删除
            let _drop_node = unsafe { Box::from_raw(node_mut) };
        }

        Some(entry)
    }

    pub fn iter(&self) -> TreeIter<'_, K, V> {
        let mut node = match &self.root {
            Some(node) => node,
            None => return TreeIter::new(None),
        };

        let mut node_ref = unsafe { node.as_ref() };
        while !node_ref.is_leaf {
            node = node_ref.children.first().unwrap();
            node_ref = unsafe { node.as_ref() };
        }

        TreeIter::new(Some(node))
    }
}

type NodeRef<'a, K, V> = &'a NonNull<Node<K, V>>;

type NodeIter<'a, K, V> = Zip<Iter<'a, K>, Iter<'a, V>>;

pub struct TreeIter<'a, K, V> {
    node_iter: Option<(NodeRef<'a, K, V>, NodeIter<'a, K, V>)>,
}

impl<'a, K: Ord + Copy + Debug, V> TreeIter<'a, K, V> {
    fn new(node: Option<&'a NonNull<Node<K, V>>>) -> Self {
        Self {
            node_iter: node.map(|node| {
                let node_ref = unsafe { node.as_ref() };
                let iter = node_ref.keys.iter().zip(node_ref.values.iter());

                (node, iter)
            }),
        }
    }
}

impl<'a, K: Ord + Copy + Debug, V> Iterator for TreeIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.node_iter {
            None => None,
            Some((node, iter)) => iter.next().or_else(|| {
                let node_ref = unsafe { node.as_ref() };
                match &node_ref.next {
                    None => {
                        self.node_iter = None;
                        None
                    }
                    Some(node) => {
                        let node_ref = unsafe { node.as_ref() };
                        let iter = node_ref.keys.iter().zip(node_ref.values.iter());
                        self.node_iter = Some((node, iter));
                        self.next()
                    }
                }
            }),
        }
    }
}

impl<K: Ord + Copy, V> Node<K, V> {
    fn new(is_leaf: bool) -> Self {
        Self {
            is_leaf,
            keys: vec![],
            children: vec![],
            values: vec![],
            next: None,
        }
    }
}

impl<K: Debug, V: Debug> Debug for Node<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.keys
                .iter()
                .map(|k| format!("{:?}", k))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl<K: Debug, V: Debug> Debug for BPlusTree<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let root = match self.root {
            None => return write!(f, "None"),
            Some(node) => node,
        };

        let mut queue = vec![&root];
        while !queue.is_empty() {
            let mut next_level = vec![];
            for node_ptr in queue {
                let node_ref = unsafe { node_ptr.as_ref() };
                write!(f, "{:?}", node_ref)?;
                for child in &node_ref.children {
                    next_level.push(child);
                }
            }

            writeln!(f)?;
            queue = next_level;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bptree_insert() {
        let mut t = BPlusTree::new(3);
        for i in 1..=16 {
            t.insert((i, ()));
        }
        assert_eq!(t.len(), 16);

        assert_eq!(
            format!("\n{:?}", t),
            "
[5,9]
[3][7][11,13]
[2][4][6][8][10][12][14,15]
[1][2][3][4][5][6][7][8][9][10][11][12][13][14][15,16]
"
        );

        for i in 1..=16 {
            assert!(t.insert((i, ())).is_some());
        }
        assert_eq!(t.len(), 16);

        assert_eq!(
            format!("\n{:?}", t),
            "
[5,9]
[3][7][11,13]
[2][4][6][8][10][12][14,15]
[1][2][3][4][5][6][7][8][9][10][11][12][13][14][15,16]
"
        );
    }

    #[test]
    fn bptree_delete() {
        let mut t = BPlusTree::new(3);
        for i in 1..=16 {
            t.insert((i, ()));
        }

        assert!(t.delete(&2).is_some());
        assert!(t.delete(&4).is_some());
        assert!(t.delete(&6).is_some());
        assert!(t.delete(&8).is_some());
        assert!(t.delete(&10).is_some());
        assert!(t.delete(&12).is_some());
        assert!(t.delete(&14).is_some());
        assert!(t.delete(&16).is_some());

        assert!(t.delete(&2).is_none());
        assert!(t.delete(&16).is_none());

        assert!(t.delete(&13).is_some());
        assert!(t.delete(&11).is_some());
        assert!(t.delete(&9).is_some());
        assert!(t.delete(&7).is_some());
        assert!(t.delete(&5).is_some());
        assert!(t.delete(&3).is_some());

        assert!(t.delete(&1).is_some());
        assert!(t.delete(&15).is_some());
        assert!(t.is_empty());
    }

    #[test]
    fn bptree_iter() {
        let mut t = BPlusTree::new(10);
        for i in 1..1000 {
            t.insert((i, ()));
        }

        let mut index = 1;
        let mut it = t.iter();
        while let Some(item) = it.next() {
            assert_eq!(item.0, &index);
            index += 1;
        }
    }
}
