//! 布隆过滤器
//!
//! 参考:
//!
//! - [bloom](https://github.com/bits-and-blooms/bloom)

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::bitset::BitSet;

/// 参考 go-bloom 的实现, 这里偷懒直接把标准库哈希重新哈希了四次
fn hash<T: Hash + ?Sized>(key: &T) -> [u64; 4] {
    let mut h = DefaultHasher::new();
    key.hash(&mut h);
    let v0 = h.finish();

    (v0 & 0xFFFF).hash(&mut h);
    let v1 = h.finish();

    ((v0 >> 16) & 0xFFFF).hash(&mut h);
    let v2 = h.finish();

    ((v0 >> 32) & 0xFFFF).hash(&mut h);
    let v3 = h.finish();

    ((v0 >> 48) & 0xFFFF).hash(&mut h);
    let v4 = h.finish();

    [v1, v2, v3, v4]
}

pub struct BloomFilter {
    k: usize,
    bits: BitSet,
}

impl BloomFilter {
    pub fn new(m: usize, k: usize) -> Self {
        Self {
            k,
            bits: BitSet::new(m),
        }
    }

    fn location(&self, hbase: &[u64; 4], i: usize) -> usize {
        let p1 = hbase[((i + i % 2) % 4) / 2 + 2];
        let (p0, _) = hbase[i % 2].overflowing_add((i as u64) * p1);
        (p0 as usize) % self.bits.len()
    }

    pub fn add<T: Hash + ?Sized>(&mut self, key: &T) {
        let hbase = hash(key);
        for i in 0..self.k {
            let pos = self.location(&hbase, i);
            self.bits.set(pos);
        }
    }

    pub fn test<T: Hash + ?Sized>(&self, key: &T) -> bool {
        let hbase = hash(key);
        for i in 0..self.k {
            let pos = self.location(&hbase, i);
            if !self.bits.test(pos) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_filter() {
        let mut b = BloomFilter::new(1024, 2);

        b.add(&1);
        b.add(&2);
        b.add(&3);
        b.add(&4);

        assert!(b.test(&1));
        assert!(b.test(&2));
        assert!(b.test(&3));
        assert!(b.test(&4));
    }
}
