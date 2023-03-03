//! 布谷鸟过滤器
//!
//! 参考:
//!
//! - [布谷鸟过滤器：实际上优于布隆过滤器](http://www.linvon.cn/posts/cuckoo/)

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// 最大踢出次数
const MAX_KICK: usize = 32;

/// 桶数量
const BUCKET_COUNT: usize = 1024;

/// 每个桶指纹树
const BUCKET_SIZE: usize = 4;

/// 布谷鸟过滤器
pub struct CuckooFilter {
    // 简单的布谷鸟哈希桶列表
    buckets: Vec<Vec<u8>>,
}

impl CuckooFilter {
    pub fn new() -> Self {
        Self {
            buckets: vec![vec![]; BUCKET_COUNT],
        }
    }

    /// 传入的是 key 的哈希值计算 key 的指纹, 简写直接取低 8 位
    fn fingerprint(&self, hash: u64) -> u8 {
        hash as u8
    }

    /// 计算哈希值
    fn hash(&self, key: &[u8]) -> u64 {
        let mut h = DefaultHasher::new();
        key.hash(&mut h);
        h.finish()
    }

    /// 对指纹取哈希
    fn hash_fp(&self, fp: u8) -> u64 {
        self.hash(&[fp])
    }

    /// 哈希值转换到桶索引, 简写直接用高 32 位取模作为桶索引
    fn hash2index(&self, hash: u64) -> usize {
        ((hash >> 32) as usize) % self.buckets.len()
    }

    /// 插入元素
    pub fn insert(&mut self, key: &[u8]) -> bool {
        let hash1 = self.hash(key);
        let fp = self.fingerprint(hash1);
        let i1 = self.hash2index(hash1);

        let bucket = &mut self.buckets[i1];
        if bucket.len() < BUCKET_SIZE {
            bucket.push(fp);
            return true;
        }

        // hash2 通过 hash1 和指纹哈希取反得到
        // 便于后续通过一个哈希取得另外一个哈希
        // hash2 = hash1 ^ fp_hash
        // hash1 = hash2 ^ fp_hash
        let hash2 = hash1 ^ self.hash_fp(fp);
        let i2 = self.hash2index(hash2);
        let bucket = &mut self.buckets[i2];
        if bucket.len() < BUCKET_SIZE {
            bucket.push(fp);
            return true;
        }

        let mut fp = fp;
        let mut curr_hash = hash2;
        let mut curr_bucket = bucket;

        for _ in 0..MAX_KICK {
            // 随便踢出一个倒霉蛋
            let kicked = curr_bucket.pop().unwrap();
            curr_bucket.push(fp);

            // 取反找到另外一个 hash 在另外一个桶上插入指纹
            // 如果另外一个桶上也已经满了则继续在新桶上踢出
            let another_hash = curr_hash ^ self.hash_fp(kicked);
            let another_index = self.hash2index(another_hash);
            let another_bucket = &mut self.buckets[another_index];
            if another_bucket.len() < BUCKET_SIZE {
                another_bucket.push(kicked);
                return true;
            }

            fp = kicked;
            curr_hash = another_hash;
            curr_bucket = another_bucket;
        }

        false
    }

    /// 查找元素是否可能存在
    pub fn lookup(&self, key: &[u8]) -> bool {
        let hash1 = self.hash(key);
        let fp = self.fingerprint(hash1);
        let i1 = self.hash2index(hash1);
        let bucket = &self.buckets[i1];
        if bucket.contains(&fp) {
            return true;
        }

        let hash2 = hash1 ^ self.hash_fp(fp);
        let i2 = self.hash2index(hash2);
        let bucket = &self.buckets[i2];
        bucket.contains(&fp)
    }

    /// 删除元素
    /// 需要保证元素之前被插入过, 否则可能会因为假阳哈希碰撞导致误删除
    pub fn delete(&mut self, key: &[u8]) {
        let hash1 = self.hash(key);
        let fp = self.fingerprint(hash1);
        let i1 = self.hash2index(hash1);
        let bucket = &mut self.buckets[i1];
        for (index, &fp1) in bucket.iter().enumerate() {
            if fp == fp1 {
                bucket.remove(index);
                return;
            }
        }

        let hash2 = hash1 ^ self.hash_fp(fp);
        let i2 = self.hash2index(hash2);
        let bucket = &mut self.buckets[i2];
        for (index, &fp1) in bucket.iter().enumerate() {
            if fp == fp1 {
                bucket.remove(index);
                return;
            }
        }
    }
}

impl Default for CuckooFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cuckoo_filter() {
        let mut cf = CuckooFilter::new();

        let keys = ["😀", "🐱", "🤔", "😨", "😭", "😊"];
        for key in keys {
            let key = key.as_bytes();
            match cf.insert(key) {
                true => assert!(cf.lookup(key)),
                false => println!("insert {:?} failed", key),
            }
        }
    }
}
