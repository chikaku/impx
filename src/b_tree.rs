//! B-Tree
//!
//! - [B树](https://zh.wikipedia.org/zh-hans/B%E6%A0%91)
//! - [B-Tree 可视化](https://www.cs.usfca.edu/~galles/visualization/BTree.html)
//! - <https://github.com/linw1995/bt>
//!
//! M 阶 `B-Tree` 规则:
//!
//! - 每个节点最多有 M 个子节点
//! - 内部节点最少有 (M-1)/2 个 value 节点
//! - 如果根节点不是叶子节点, 则至少有两个子节点
//! - 有 k 个子节点的非叶子节点有 k-1 个键
//! - 所有的叶子节点在同一层
//!
//! 具体实现详情见代码内注释

use std::{fmt::Debug, ptr::NonNull};

type Entry<K, V> = (K, V);

/// B-Tree 节点
pub struct BTreeNode<K, V> {
    order: usize,
    values: Vec<Entry<K, V>>,
    children: Vec<NonNull<BTreeNode<K, V>>>,
}

/// B-Tree
pub struct BTree<K, V> {
    root: NonNull<BTreeNode<K, V>>,
    length: usize,
}

impl<K: Ord, V> BTree<K, V> {
    /// 创建一个 M 阶 B-Tree
    pub fn new(order: usize) -> Self {
        let root = BTreeNode::new(order);
        let root = unsafe { root.into_raw_ptr() };

        Self { root, length: 0 }
    }

    /// 返回 B-Tree 节点个数
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// 返回 B-Tree 中 key 对应的节点
    pub fn get(&self, key: &K) -> Option<&Entry<K, V>> {
        unsafe { self.root.as_ref().get(key) }
    }

    /// 返回 B-Tree 中最小 Entry
    pub fn min(&self) -> Option<&Entry<K, V>> {
        let root = unsafe { self.root.as_ref() };
        root.min()
    }

    /// 返回 B-Tree 中最大 Entry
    pub fn max(&self) -> Option<&Entry<K, V>> {
        let root = unsafe { self.root.as_ref() };
        root.max()
    }

    /// 向 B-Tree 中插入 Entry 如果对应 key 已经存在则将旧值换出
    pub fn insert(&mut self, entry: Entry<K, V>) -> Option<Entry<K, V>> {
        let key = &entry.0;

        // 收集下降过程中的父节点
        let mut parents = vec![];
        let mut curr_node = self.root;

        // 从上至下进行查找, 找到相等的 key 直接替换
        // 否则一直下降到叶子节点上进行新增
        loop {
            let node = unsafe { curr_node.as_mut() };
            match node.values.binary_search_by(|e| e.0.cmp(key)) {
                Ok(idx) => {
                    let old = &mut node.values[idx];
                    return Some(std::mem::replace(old, entry));
                }
                Err(idx) => {
                    if node.is_leaf() {
                        node.values.insert(idx, entry);
                        self.length += 1;
                        break;
                    }

                    assert!(idx <= node.values.len());
                    parents.push(curr_node);
                    curr_node = node.children[idx];
                }
            }
        }

        // 从下至上对每个满节点进行分裂
        loop {
            let node = unsafe { curr_node.as_mut() };
            if node.values.len() < node.order {
                return None;
            }

            // 节点数量最大应该等于阶数
            assert!(node.values.len() <= node.order);

            // 叶节点满之后要进行节点分裂
            // 取中间节点上升到上一级, 左右两边分裂作为新的子节点
            // 取出后半部分的值放到新的节点上
            // 对于非叶子节点同理取出后半部分的 children 放到新的节点上
            let mid_idx = (node.order / 2) + 1;
            let mut new_node = BTreeNode::new(node.order);
            new_node.values = node.values.drain(mid_idx..).collect();
            if !node.is_leaf() {
                new_node.children = node.children.drain(mid_idx..).collect();
            }

            // 从前半部分节点尾部取出中间节点(提升到上一级)
            let mid_entry = node.values.pop().expect("child have at least one node");

            let left_node = curr_node;
            let right_node = unsafe { new_node.into_raw_ptr() };

            match parents.pop() {
                None => {
                    // 如果父节点是空的则 curr_node 是根节点
                    let mut new_root = BTreeNode::new(node.order);
                    new_root.values.push(mid_entry);
                    new_root.children.push(left_node);
                    new_root.children.push(right_node);
                    self.root = unsafe { new_root.into_raw_ptr() };
                    return None;
                }
                Some(mut parent) => {
                    // 原本父节点的一个子节点分裂成左右两部分和一个中间节点
                    // 其中中间节点添加到父节点的 values 内
                    // 左右部分添加到 children 对应的位置上
                    let key = &mid_entry.0;
                    let par_node = unsafe { parent.as_mut() };
                    match par_node.values.binary_search_by(|e| e.0.cmp(key)) {
                        Ok(_) => panic!("child key should't appear at parent node"),
                        Err(idx) => {
                            par_node.values.insert(idx, mid_entry);

                            // 原本在 idx 上的子节点产生了分裂所以要先把 idx 移除掉再插入左节点
                            // par_node.children.remove(idx);
                            // par_node.children.insert(idx, left_node);
                            par_node.children[idx] = left_node;
                            par_node.children.insert(idx + 1, right_node);

                            // 父节点变成当前节点继续检查是否需要分裂
                            curr_node = parent;
                        }
                    }
                }
            };
        }
    }

    /// 在 B-Tree 中删除 key 如果存在则返回被删除的 Entry
    pub fn delete(&mut self, key: &K) -> Option<Entry<K, V>> {
        let mut curr_node = self.root;
        let mut parents = vec![];

        let val_idx = loop {
            let node = unsafe { curr_node.as_ref() };
            match node.values.binary_search_by(|e| e.0.cmp(key)) {
                Ok(idx) => break idx,
                Err(idx) => {
                    if node.is_leaf() {
                        return None;
                    }

                    // 否则寻找下一级的节点
                    parents.push((curr_node, idx));
                    curr_node = node.children[idx];
                    continue;
                }
            }
        };

        let mut key_node = curr_node;
        let key_node = unsafe { key_node.as_mut() };
        let old_val = if key_node.is_leaf() {
            key_node.values.remove(val_idx)
        } else {
            // 如果是中间节点, 需要继续向下找到左子树的最大节点
            parents.push((curr_node, val_idx));
            curr_node = key_node.children[val_idx];
            let mut node = unsafe { curr_node.as_mut() };
            while !node.is_leaf() {
                let idx = node.children.len() - 1;
                parents.push((curr_node, idx));
                curr_node = node.children[idx];
                node = unsafe { curr_node.as_mut() };
            }

            // 得到左子树的最大节点后替换掉中间节点上的值
            let entry = node.pop_max().expect("non-left max child must be exists");
            std::mem::replace(&mut key_node.values[val_idx], entry)
        };

        self.length -= 1;

        // 如果叶子节点上的值数量仍然大于等于阶数的一半则无需重新平衡
        let mut node = unsafe { curr_node.as_mut() };
        if node.values.len() >= node.order / 2 {
            return Some(old_val);
        }

        let order = node.order;
        let limit = (order - 1) / 2;
        // 限制: 每个节点最少有 limit 个 value
        // 平衡流程, 如果当前节点元素 n0 < limit
        // 1. 如果右兄弟有多余元素(n1 > limit), 用其最小值交换间隔父节点, 把父节点交换下来
        // 2. 如果左兄弟有多余元素(n2 > limit), 用其最大值交换间隔父节点, 把父节点交换下来
        // 3. 如果左右兄弟都没有多余元素即 n1 <= limit && n2 <= limit
        //    则可以将当前节点与某个兄弟加上间隔的父节点合并, 比如合并右节点
        //    n0 + n2 + 1< 2*limit + 1 = order
        //    合并完的节点数量依然是合法的

        while let Some((mut parent, index)) = parents.pop() {
            let par_node = unsafe { parent.as_mut() };

            // 检查左节点是否存在; 数量是否有多余
            // 这里检查左边放在检查右边前面, 因为左边是取最大值右边是取最小值
            // 取最大值直接用 pop O(1) 取最小值用 remove(0) 是 O(N) 的
            // 对于 children 节点的插入顺序是相反的, 但是因为有一次叶子节点的判断所以还是左节点放前面好
            if index > 0 {
                let par_mid_val = &mut par_node.values[index - 1];
                let mut sib_left = par_node.children[index - 1];
                let sib_left = unsafe { sib_left.as_mut() };
                if sib_left.values.len() > limit {
                    let sib_left_max = sib_left.values.pop().expect("");
                    let par_mid_val = std::mem::replace(par_mid_val, sib_left_max);
                    node.values.push(par_mid_val);
                    if !node.is_leaf() {
                        node.children.insert(0, sib_left.children.pop().expect(""));
                    }

                    node = par_node;
                    if node.values.len() >= limit {
                        return Some(old_val);
                    }

                    continue;
                }
            }

            // 检查右节点是否存在; 数量是否有多余
            if index + 1 < par_node.children.len() {
                let par_mid_val = &mut par_node.values[index];
                let mut sib_right = par_node.children[index + 1];
                let sib_right = unsafe { sib_right.as_mut() };
                if sib_right.values.len() > limit {
                    let sib_right_min = sib_right.values.remove(0);
                    let par_mid_val = std::mem::replace(par_mid_val, sib_right_min);
                    node.values.push(par_mid_val);
                    if !node.is_leaf() {
                        node.children.push(sib_right.children.remove(0));
                    }

                    node = par_node;
                    if node.values.len() >= limit {
                        return Some(old_val);
                    }

                    continue; // 递归向上
                }
            }

            // 左右节点都不满足开始执行合并
            let (mid_val, mut left, mut right, merge_idx) = if index > 0 {
                // 合并左节点移除 children 中的 index 替换 index-1
                let par_mid_val = par_node.values.remove(index - 1);
                let left = par_node.children[index - 1];
                let right = par_node.children[index];
                (par_mid_val, left, right, index - 1)
            } else {
                // 合并右节点移除 children 中的 index+1 替换 index
                let par_mid_val = par_node.values.remove(index);
                let left = par_node.children[index];
                let right = par_node.children[index + 1];
                (par_mid_val, left, right, index)
            };

            par_node.children.remove(merge_idx + 1);
            let left_node = unsafe { left.as_mut() };
            let right_node = unsafe { right.as_mut() };

            // 把原先间隔的父节点 value 和右边节点上的所有数据挪到左边节点上
            left_node.values.push(mid_val);
            left_node.values.append(&mut right_node.values);
            left_node.children.append(&mut right_node.children);

            // 如果当前父节点是根节点且是空节点则直接替换根节点
            if par_node.values.is_empty() && parents.is_empty() {
                self.root = left;
                return Some(old_val);
            }

            par_node.children[merge_idx] = left;
            node = par_node;
            if node.values.len() >= limit {
                return Some(old_val);
            }
        }

        Some(old_val)
    }
}

impl<K: Ord, V> BTreeNode<K, V> {
    pub fn new(order: usize) -> Self {
        Self {
            order,
            values: Vec::with_capacity(order),
            children: Vec::with_capacity(order),
        }
    }

    /// 是否叶子节点
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// 根据 key 查找是否存在
    pub fn get(&self, key: &K) -> Option<&Entry<K, V>> {
        match self.values.binary_search_by(|e| e.0.cmp(key)) {
            Ok(idx) => Some(&self.values[idx]),
            Err(idx) => self
                .children
                .get(idx)
                .and_then(|node| unsafe { node.as_ref().get(key) }),
        }
    }

    fn min(&self) -> Option<&Entry<K, V>> {
        let mut node = self;
        loop {
            if node.is_leaf() {
                return node.values.first();
            }

            let child = node.children[0];
            node = unsafe { child.as_ref() };
        }
    }

    fn max(&self) -> Option<&Entry<K, V>> {
        let mut node = self;
        loop {
            if node.is_leaf() {
                return node.values.last();
            }

            let child = node.children[node.children.len() - 1];
            node = unsafe { child.as_ref() };
        }
    }

    fn pop_max(&mut self) -> Option<Entry<K, V>> {
        if self.is_leaf() {
            return self.values.pop();
        }

        let mut max_child = self.children[self.children.len() - 1];
        let max_child = unsafe { max_child.as_mut() };
        max_child.pop_max()
    }

    unsafe fn into_raw_ptr(self) -> NonNull<Self> {
        NonNull::new_unchecked(Box::into_raw(Box::new(self)))
    }
}

impl<K: Debug, V: Debug> Debug for BTree<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut q = vec![&self.root];

        while !q.is_empty() {
            let s = q.drain(0..).collect::<Vec<_>>();
            for node in s {
                let node = unsafe { node.as_ref() };
                write!(
                    f,
                    "{:?}",
                    node.values.iter().map(|e| &e.0).collect::<Vec<_>>()
                )?;
                for child in &node.children {
                    q.push(child);
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btree() {
        let mut t = BTree::new(3);

        t.insert((1, ()));
        t.insert((2, ()));
        t.insert((3, ()));
        t.insert((4, ()));
        t.insert((5, ()));
        t.insert((6, ()));
        t.insert((7, ()));
        t.insert((8, ()));
        t.insert((9, ()));
        t.insert((10, ()));
        t.insert((11, ()));
        t.insert((12, ()));
        println!("{:?}", t);

        assert_eq!(t.len(), 12);
        assert_eq!(t.min(), Some(&(1, ())));
        assert_eq!(t.max(), Some(&(12, ())));

        assert!(t.delete(&6).is_some());
        assert!(t.delete(&1).is_some());
        assert!(t.delete(&12).is_some());
        assert!(t.delete(&9).is_some());
        assert!(t.delete(&6).is_none());
        assert!(t.delete(&8).is_some());
        println!("{:?}", t);

        assert_eq!(t.len(), 7);
        assert_eq!(t.min(), Some(&(2, ())));
        assert_eq!(t.max(), Some(&(11, ())));

        t.delete(&10);
        t.delete(&2);
        t.delete(&4);
        t.delete(&3);
        t.delete(&11);
        t.delete(&7);
        t.delete(&5);
        println!("{:?}", t);

        assert!(t.is_empty());
    }
}
