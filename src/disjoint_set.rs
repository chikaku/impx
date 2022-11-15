//! 并查集
//!
//! <https://oi-wiki.org/ds/dsu/>  
//! <https://zh.wikipedia.org/wiki/并查集>
//!
//! 并查集是用于管理数据所属集合的数据结构  
//! 在并查集构造的过程中, 将具有相同特征的数据合并在一起连接成一棵树  
//! 每棵树上有一个唯一的根节点, 具有相同根节点的元素在同一棵树中, 同属一个集合  
//! 集合与集合之间也可以合并, 但一定要通过根节点合并, 将一棵树的根节点指向另外一棵树的根节点  
//! 在图中使用并查集, 可以将同一个连通分量中的所有节点视为一个集合, 表示为一棵树

use std::{collections::HashMap, hash::Hash};

/// 并查集
pub struct DisjointSet<T> {
    parent: HashMap<T, T>,
    size: HashMap<T, usize>,
}

impl<T: Eq + Hash + Copy> DisjointSet<T> {
    /// 初始化并查集  
    /// 将所有元素的父节点设置为自己
    pub fn new(total: &[T]) -> Self {
        let mut size = HashMap::new();
        let mut parent = HashMap::new();
        for &v in total {
            parent.insert(v, v);
            size.insert(v, 1);
        }

        Self { parent, size }
    }

    /// 合并节点, 将 a 节点和 b 节点合并到同一集合  
    /// 实际执行的是将 a 节点的根节点和 b 节点的根节点进行合并  
    /// 在初始化时已经预先将每个节点的根节点设置为自己  
    /// 合并相当于在树的根节点上多加了一个层级  
    /// 所以在合并时可以通过判断节点所在集合的大小, 将节点较小的节点合并到较大的集合
    pub fn union(&mut self, a: &T, b: &T) {
        let aroot = self.find(a);
        let broot = self.find(b);

        if aroot != broot {
            // 合并时选择将节点数较小的节点树合并到较大的那一颗
            let asize = self.size.get(&aroot).expect("root must have size");
            let bsize = self.size.get(&broot).expect("root must have size");

            if asize < bsize {
                self.parent.insert(aroot, broot);
                self.size.insert(broot, asize + bsize);
            } else {
                self.parent.insert(broot, aroot);
                self.size.insert(aroot, asize + bsize);
            }
        }
    }

    /// 查找节点对应的根节点, 期间可以执行路径压缩, 缩短当前节点和根结点之间的路径
    pub fn find(&mut self, x: &T) -> T {
        let parent = self.parent.get(x).expect("");
        if parent == x {
            // 如果自己的父节点等于自己
            // 那么自己就是根节点
            return parent.to_owned();
        }

        // 否则向上查询父节点的根节点
        let x = x.to_owned();
        let parent = parent.to_owned();
        let root = self.find(&parent);

        // 找到根节点后, 直接将当前节点的父节点指向根节点压缩到根节点的路径
        self.parent.insert(x, root);
        root
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_disjoint_set() {
        use super::DisjointSet;

        let mut set = DisjointSet::new(&[1, 2, 3, 4, 5, 6, 7]);
        set.union(&1, &2);
        set.union(&2, &3);

        set.union(&4, &5);
        set.union(&5, &6);
        set.union(&6, &7);

        assert_eq!(set.find(&1), set.find(&3));
        assert_eq!(set.find(&5), set.find(&6));
        assert_ne!(set.find(&2), set.find(&4));
        assert_ne!(set.find(&3), set.find(&7));
    }
}
