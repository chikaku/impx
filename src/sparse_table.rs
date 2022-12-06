//! 稀疏表
//!
//! - [OI Wiki - ST 表](https://oi-wiki.org/ds/sparse-table/)
//! - [OI Wiki - 倍增](https://oi-wiki.org/basic/binary-lifting/)
//! - [【白话系列】倍增算法](https://blog.csdn.net/jarjingx/article/details/8180560)
//! - [算法学习笔记(12): ST表](https://zhuanlan.zhihu.com/p/105439034)
//!
//! 稀疏表使用了倍增思想主要用来解决[可重复贡献问题](https://oi-wiki.org/ds/sparse-table/定义)比如区间的最小最大值问题  
//! 稀疏表需要以 `O(nlogn)` 的时间复杂度对输入数据进行预处理, 然后使查询的时间复杂度降低到 `O(1)`
//!
//! 正常情况下如果我们需要求某区间 `[i, j]` 的最大值有两种方式:
//!
//! - 每次从 i 遍历到 j 取最大值, 这样时间复杂度很高
//! - 预先计算好任意 `[i, j]` 的最大值则查询时就是 `O(1)` 了
//!
//! 对于第二种方式, 如果一个一个去比较所有可能区间最终构造的时间复杂度是 `O(n^2)` 有没有可能减少这个时间复杂度  
//! 可以考虑用倍增的思想, 对于每个起点 i 我们不是计算所有的 `[i, n]` 的最大值而是计算倍增步长  
//! 比如对于任意的 `[1, n]` 我们只需要存储 `[1,1], [1,2], [1,4], [1,8]` 这些范围内的最大值  
//! 我们给每个起点都计算这些步长的步数最终能覆盖所有区间范围, 然后我们就可以通过最多一次计算找到某个区间的最大值  
//! 比如 `1,3` 是没有直接存储的的的但是可以通过 `max([1,2], [2,3])` 得到  
//! 这样我们就以 `O(nlogn)` 的时间复杂度求出了覆盖所有范围的最大值  
//! 同时空间复杂度也可以下降到 `O(nlogn)` 因为第二维存的不是索引 `1,2,4,8` 而是步长 `0,1,2,3` 这样就减少了空间的使用
//!
//! 在在进行预处理的时候时通过动态规划做计算的, 如果把每个起点坐标作为遍历的第一层是没把法计算的  
//! 比如我们计算起点为 1 的数据 `[1,1], [1,2]` 都好算但是 `[1,4]` 是算不出来的
//!
//! 所以在预处理的过程中就像是上面我们在构造 `[1,8]` 的最大值的过程, 递增步长
//!
//! - 第一次步长为 1 计算
//! - 第二次步长为 2 计算
//! - 第三次步长为 4 计算
//! - ...
//!
//! 最终我们得到了预处理过的数据结构 `f[i][j]` 表示 `[i, i+2^j-1]` 区间内的最大值 `j` 也就相当于倍增的步长
//!
//! 那如何通过这个数据结构进行查询呢? 根据 `max[a, c] = max(max[a, b], max[b, c])`  
//! 对于区间 `l` 到 `r` 范围内的最大值我们只需要找到一个 `s1, s2` 使得 `l <= s1, s2 <= r`  
//! 然后再计算 `max[l, r] = max(max[l, s2], max[s1, r])` 即可  
//! 而且这里要求 `s1 = l + 2^n-1` 和 `r = s2 + 2^n -1` 才能够使用我们预处理的数据结构 `f`  
//! 对于 s1 我们肯定是想让他尽量靠近 r 避免产生 `s1 < s2` 的情况  
//! 所以直接找到一个最大的 `n = log(r-l+1)` 得到 `s1 = l+2^n-1`  
//! 同时这个步长也能被 s2 用到即 `r = s2 + 2^n-1` 那 s2 也可以算出来 `s2 = r - 2^n -1`  
//! 通过步长最终可以得到结果 `max[l,r] = max(f[l][n], f[s2][n])`

/// 初始化稀疏表
pub fn init(v: &[isize]) -> Vec<Vec<isize>> {
    let n = v.len();
    // 先计算最大步长
    let max_step = (n as f32).log2().ceil() as usize;
    let mut f = vec![vec![0; max_step + 1]; n];

    // 对于 0 步长最大值都是自己
    for i in 0..n {
        f[i][0] = v[i];
    }

    // 外层步长从 1 开始遍历到多一倍步长
    for step in 1..=max_step {
        // 内层起点从 0 开始
        for start in 0..n {
            // 如果范围在步长以内
            // 把步长切半, 比较两个小范围的最大值
            if start + (1 << step) - 1 < n {
                f[start][step] = f[start][step - 1].max(f[start + (1 << (step - 1))][step - 1]);
            }
        }
    }

    f
}

/// 计算稀疏表 f 所表示范围区间 `[l, r]` 内的最大值
pub fn max(f: &[Vec<isize>], l: usize, r: usize) -> isize {
    let n = ((r - l + 1) as f32).log2().floor() as usize;
    f[l][n].max(f[r + 1 - (1 << n)][n])
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sparse_table_max() {
        use super::*;

        let a = vec![3, 5, 7, 2, 1, 9];
        let f = init(&a);

        assert_eq!(max(&f, 0, 3), 7);
        assert_eq!(max(&f, 3, 4), 2);
        assert_eq!(max(&f, 4, 4), 1);
        assert_eq!(max(&f, 0, 5), 9);
    }

    fn rand_slice(n: i32) -> Vec<isize> {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        (0..n).map(|_| rng.gen_range(0..1000)).collect()
    }

    #[test]
    fn test_sparse_table_rand() {
        use super::*;

        let a = rand_slice(64);
        let f = init(&a);

        assert_eq!(&max(&f, 20, 60), a[20..61].iter().max().unwrap());
        assert_eq!(&max(&f, 10, 60), a[10..61].iter().max().unwrap());
        assert_eq!(&max(&f, 0, 64), a[0..].iter().max().unwrap());
        assert_eq!(&max(&f, 60, 64), a[60..].iter().max().unwrap());
    }
}
