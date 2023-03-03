//! 位图
//!
//! 参考:
//!
//! - [bitset](https://github.com/bits-and-blooms/bitset)

pub struct BitSet {
    length: usize,
    bits: Vec<u64>,
}

const ALIGN: usize = 6;
const WORD_SIZE: usize = 64;

fn alignof6(n: usize) -> usize {
    if n & (WORD_SIZE - 1) == 0 {
        return n >> ALIGN;
    }

    (n >> ALIGN) + 1
}

fn word_index(n: usize) -> usize {
    n & (WORD_SIZE - 1)
}

impl BitSet {
    pub fn new(cap: usize) -> Self {
        let size = alignof6(cap);

        Self {
            length: size * WORD_SIZE,
            bits: vec![0; size],
        }
    }

    // 返回当前位图大小
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.length
    }

    // 重置位图
    pub fn reset(&mut self) {
        self.bits.fill(0);
    }

    fn extend(&mut self, length: usize) {
        let new_size = alignof6(length - self.length);
        let new_bits = vec![0; new_size];
        self.bits.extend(new_bits.iter());
        self.length += new_size * WORD_SIZE;
    }

    // 将第 pos 位(从 0 开始)设置为 1
    pub fn set(&mut self, pos: usize) {
        if pos >= self.length {
            self.extend(pos + 1);
        }

        self.bits[pos >> ALIGN] |= 1 << word_index(pos)
    }

    // 返回第 pos 位(从 0 开始)是否为 1
    pub fn test(&self, pos: usize) -> bool {
        if pos >= self.length {
            return false;
        }

        self.bits[pos >> ALIGN] & 1 << word_index(pos) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align() {
        assert_eq!(alignof6(32), 1);
        assert_eq!(alignof6(64), 1);
        assert_eq!(alignof6(65), 2);
    }

    #[test]
    fn test_bitset() {
        let mut bs = BitSet::new(32);

        bs.set(0);
        bs.set(10);
        bs.set(31);

        assert!(bs.test(0));
        assert!(bs.test(10));
        assert!(bs.test(31));

        assert!(!bs.test(129));
        bs.set(129);
        assert!(bs.test(129));
    }
}
