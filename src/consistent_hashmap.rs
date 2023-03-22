//! 一致哈希
//!
//! 参考: <https://github.com/buraksezer/consistent>

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// 虚拟节点
struct VNode<T> {
    hash: u64,   // 虚拟节点散列值
    node: T,     // 指向的真实节点
    load: usize, // 负载
}

pub struct ConsistentHashMap<T> {
    partition_count: usize,           // 分区数量
    virtual_replication_count: usize, // 每个节点的虚拟节点副本数量

    vnodes: Vec<VNode<T>>, // 虚拟节点列表
    partitions: Vec<T>,    // 分区列表
}

fn hasher<T: Hash>(v: &T) -> u64 {
    let mut state = DefaultHasher::new();
    v.hash(&mut state);
    state.finish()
}

impl<T> ConsistentHashMap<T>
where
    T: Copy + Clone + Ord + Hash,
{
    pub fn new(partition_count: usize, virtual_replication_count: usize) -> Self {
        Self {
            partition_count,
            virtual_replication_count,
            vnodes: vec![],
            partitions: vec![],
        }
    }

    fn balance_load(&mut self) {
        if self.vnodes.is_empty() {
            return;
        }

        // 重置负载
        self.vnodes.iter_mut().for_each(|v| v.load = 0);

        // 每个虚拟节点最大负载
        let max_load = (self.partition_count / self.vnodes.len()) + 1;
        let last = self.vnodes.len();

        self.partitions.clear();
        for partiton_id in 0..self.partition_count {
            let hash = hasher(&partiton_id.to_string());

            // 根据分区 ID 的哈希值找到对应的虚拟节点
            let mut index = match self
                .vnodes
                .binary_search_by(|v| v.hash.cmp(&hash))
                .map(|index| index + 1)
                .unwrap_or_else(|pos| pos)
            {
                v if v == last => 0,
                index => index,
            };

            // 找到一个未达到最大负载的虚拟节点
            let mut find_count = 0;
            let mut vnode = &mut self.vnodes[index];
            while vnode.load == max_load {
                index += 1;
                if index == last {
                    index = 0;
                }
                vnode = &mut self.vnodes[index];
                find_count += 1;
                if find_count == last {
                    panic!("cann't find available node")
                }
            }

            // 添加分区对应的真实节点和对应虚拟节点负载
            vnode.load += 1;
            self.partitions.push(vnode.node);
        }
    }

    pub fn add(&mut self, node: T) {
        // 创建虚拟节点
        for i in 0..self.virtual_replication_count {
            let mut state = DefaultHasher::new();
            i.to_string().hash(&mut state);
            node.hash(&mut state);
            let hash = state.finish();

            // 二分查找插入位置保证 vnodes 有序
            self.vnodes.insert(
                self.vnodes
                    .binary_search_by(|p| p.hash.cmp(&hash))
                    .unwrap_or_else(|pos| pos),
                VNode {
                    hash,
                    node,
                    load: 0,
                },
            )
        }

        // 重新平衡节点负载
        self.balance_load();
    }

    pub fn remove(&mut self, node: T) {
        self.vnodes.retain_mut(|v| v.node != node);
        self.balance_load();
    }

    pub fn locate<K: Hash>(&self, key: &K) -> T {
        let mut state = DefaultHasher::new();
        key.hash(&mut state);
        let hash = state.finish() as usize;

        // 定位到对应的分区
        let index = hash % self.partition_count;
        self.partitions[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_add() {
        let mut h = ConsistentHashMap::new(360, 8);

        for i in 0..360 {
            h.add(i);
        }

        for i in 0..360 {
            h.remove(i);
        }
    }

    fn test_add_relocated(partition_count: usize, virtual_replica: usize) {
        const TEST_NODE_COUNT: usize = 8;
        const TEST_KEY_COUNT: usize = 102400;

        let mut h = ConsistentHashMap::new(partition_count, virtual_replica);
        for i in 0..TEST_NODE_COUNT {
            h.add(i);
        }

        let mut vs = vec![];
        for i in 0..TEST_KEY_COUNT {
            let node = h.locate(&i);
            vs.push(node);
        }

        h.add(TEST_NODE_COUNT);
        let mut chagned = 0;
        for i in 0..TEST_KEY_COUNT {
            let node = h.locate(&i);
            if vs[i] != node {
                chagned += 1;
            }
        }

        println!(
            "{}x{}:\t{:.1}%\trelocated",
            partition_count,
            virtual_replica,
            (chagned as f64 / TEST_KEY_COUNT as f64) * 100.0
        );
    }

    #[test]
    fn test_consistent_hashmap() {
        test_add_relocated(80, 20);
        test_add_relocated(80, 25);
        test_add_relocated(80, 30);
        test_add_relocated(80, 40);
        test_add_relocated(80, 50);
        test_add_relocated(80, 60);
        test_add_relocated(80, 70);

        test_add_relocated(360, 40);
        test_add_relocated(360, 60);
        test_add_relocated(360, 80);
        test_add_relocated(360, 100);
        test_add_relocated(360, 120);
        test_add_relocated(360, 180);
        test_add_relocated(360, 300);

        // 80x20:   13.7%   relocated
        // 80x25:   18.6%   relocated
        // 80x30:   17.4%   relocated
        // 80x40:   15.0%   relocated
        // 80x50:   12.5%   relocated
        // 80x60:   16.3%   relocated
        // 80x70:   13.8%   relocated
        // 360x40:  24.4%   relocated
        // 360x60:  31.1%   relocated
        // 360x80:  25.6%   relocated
        // 360x100: 22.8%   relocated
        // 360x120: 18.6%   relocated
        // 360x180: 12.5%   relocated
        // 360x300: 12.0%   relocated
    }
}
