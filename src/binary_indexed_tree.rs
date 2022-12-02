//! 树状数组
//!
//! - [OI Wiki - 树状数组](https://oi-wiki.org/ds/fenwick/)
//! - [BV1ce411u7qP](https://www.bilibili.com/video/BV1ce411u7qP)
//! - [BV1pE41197Qj](https://www.bilibili.com/video/BV1pE41197Qj)
//!
//! 树状数组一般用于计算一个可修改的数组的前 n 个元素的和  
//! 做法是提前将每两个元素 a1, a2 的和 b1 组织成他们的父节点, 把 a3, a4 的和 b2 也组织成父节点以此类推  
//! 接着再把 b1 和 b2 的和 c1 组织成他们的父节点以此类推, 直到一层节点上只有一个元素  
//! 这样整个数组就组织称了一颗树, 但是在这颗树中有一些元素是已经不需要了的, 比如 a2, a4  
//! 不只是第一层, 第二层 b2 也是不需要的, 需要计算 a1-a3 的时候只需要计算 b1+a3 计算 a1-a4 的时候只需要 c1  
//! 再往上会发现每层的偶数个数元素都是不需要的  
//! 最终剩下来的元素刚好又组织成了一个跟源数组长度相等的数组  
//!
//! 那么在计算前 n 个元素之和的时候就可以找到前 n 个元素的各个区间和即可  
//! 比如计算 `a1~a7` 时可以用 `c1(a1-a4) + b3(a5-a6) + a7`
//!
//! 那接下来如何找到前 n 个元素对应的区间有哪些呢?  
//! 观察我们刚才构造树的过程, 在第一层我们把真正的第偶数个元素都移除了(替换成了上级节点)  
//! 也就是说此时数组中第 x 个元素(从0开始算), 如果 x 的最后一位是 0 那么这个元素就是原本的元素, 否则他就是上级的元素  
//! 同理如果 x 的倒数第二位是 0 那么这个元素就是原本的第二层的元素, 否则他就是更上级的元素  
//! 此时还能得到另外一条结论, 如果 x 的最低位 1 的位置所代表的值是 1 即其值为 `_____1` 那么他就只能代表他自己  
//! 如果 x 的最低位 1 的位置所代表的值是 4 即其值为 `___100` 那么说明他处于第三层, 代表的是其下面第一层 4 个元素的和
//! 同理 `__1000` 是第四层代表下面 8 个元素的和  
//!
//! 我们把求一个数的最低位 1 所表示的数值的函数叫做 `lowbit`  我们先组织好一颗树状数组 arr  
//! 那么在计算 arr 的前 k 个元素之和时只需要先计算 `lowbit(k-1)` 表示 `arr[k-1]` 这个值现在能代表的第一层元素的个数  
//! 接着要计算前面还剩下的没有被代表的元素的和, 此时只需要 `m = k-lowbit(k-1)` 就是剩下的前面没被代表的元素的个数  
//! 再利用相同的方法求出前 m 个元素的和加上已经算出来的 `arr[k-1]` 就是完整的前 k 个数的和了

/// 计算一个数的最低位 1 所代表的数
///
/// 比如 1100 最低位 1 表示的数是 100
///
/// ```
/// use impx::binary_indexed_tree::lowbit;
///
/// assert_eq!(lowbit(0b1010), 0b10);
/// ```
pub fn lowbit(n: usize) -> usize {
    let n = n as isize;
    (n & -n) as usize
}

/// 初始化树状数组
///
/// 首先确定自己的值, 然后把自己的值加给自己的上层父节点, 这样传递下去就能够得到完整的正确的结果  
/// 另外之前计算 lowbit 的时候都是从 1 开始索引的所以使用时要用 `lowbit(i+1)`
pub fn init(arr: &[isize]) -> Vec<isize> {
    let n = arr.len();
    let mut t = vec![0; n];
    for i in 0..n {
        t[i] += arr[i];
        let j = i + lowbit(i + 1);
        if j < n {
            t[j] += t[i]
        }
    }

    t
}

/// 计算前 n 项之和
///
/// 按照之前描述过的方式先计算 `t[n - 1]`  
/// 然后再把 `n -= lowbit(n)` 表示剩下没被代表的剩余元素个数迭代求和即可
pub fn sum(t: &[isize], mut n: usize) -> isize {
    let mut res = 0;
    while n > 0 {
        res += t[n - 1];
        n -= lowbit(n);
    }

    res
}

/// 修改原数组元素值
///
/// `arr[i] = arr[i]+v` 这里 arr 指的是源数组但是参数传进来的需要是求和后的树状数组  
/// 只需要修改对应 `i` 索引处的值并更新上层的值即可, 求 `i` 对应的父节点用 `i+lowbit(i+1)` 即可
pub fn update(t: &mut [isize], mut i: usize, v: isize) {
    let n = t.len();
    while i < n {
        t[i] += v;
        i += lowbit(i + 1);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_binary_indexed_tree() {
        use super::*;

        let a = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut t = init(&a);
        assert_eq!(&t, &[1, 3, 3, 10, 5, 11, 7, 36]);

        assert_eq!(sum(&t, 2), 3);
        assert_eq!(sum(&t, 4), 10);
        assert_eq!(sum(&t, 7), 28);
        assert_eq!(sum(&t, 8), 36);

        update(&mut t, 2, -3);
        update(&mut t, 4, -5);

        // vec![1, 2, 0, 4, 0, 6, 7, 8];
        assert_eq!(sum(&t, 2), 3);
        assert_eq!(sum(&t, 3), 3);
        assert_eq!(sum(&t, 5), 7);
        assert_eq!(sum(&t, 8), 28);
    }

    fn rand_slice(n: i32) -> Vec<isize> {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        (0..n).map(|_| rng.gen_range(0..1000)).collect()
    }

    #[test]
    fn test_binary_indexed_tree_rand() {
        use super::*;

        let a = rand_slice(128);
        let mut t = init(&a);

        assert_eq!(sum(&t, 15), a[..15].iter().sum());
        assert_eq!(sum(&t, 66), a[..66].iter().sum());
        assert_eq!(sum(&t, 97), a[..97].iter().sum());
        assert_eq!(sum(&t, 111), a[..111].iter().sum());

        update(&mut t, 32, -100);
        update(&mut t, 80, -100);
        update(&mut t, 120, -100);

        assert_eq!(sum(&t, 15), a[..15].iter().sum());
        assert_eq!(sum(&t, 66), a[..66].iter().sum::<isize>() - 100);
        assert_eq!(sum(&t, 90), a[..90].iter().sum::<isize>() - 200);
        assert_eq!(sum(&t, 121), a[..121].iter().sum::<isize>() - 300);
    }
}
