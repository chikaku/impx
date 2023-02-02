//! 约瑟夫问题
//!
//! - [OI Wiki - 约瑟夫问题]
//!
//! 编号从 [0, n-1] 的 n 个人站成一圈, 从编号 0 开始循环: 每次往后数第 k 个人踢出, 求最后剩下个一个人时此人的编号是多少
//!
//! 考虑在第一轮将编号为 k-1 的人踢出后, 将前面 [0, k-2] 移动到最后
//!
//! ```text
//! round0: 0 1 2 ... k-2 k-1 k ... n-1  
//! round1:                   k ... n-1 0 1 ... k-2  
//!                           0 ... ... ... ... ... n-2 // 相对坐标  
//!```
//!
//! 假设 `J(n, k)` 为 n 个人中踢出 k 个的最终解  
//! 则可以从上图观察到如果知道了 `t = J(n-1, k)` 的编号可以很容易算出这个编号在有 n 个人的时候的坐标即
//!
//! ```text
//! J(n, k) = (J(n-1, k) + k) % n
//! ```

/// 约瑟夫问题线性解法
pub fn josephus_linear(n: usize, k: usize) -> usize {
    let mut idx = 0;
    for i in 1..=n {
        idx = (idx + k) % i
    }

    idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_josephus() {
        // 测试用例来自:
        // https://www.andrew.cmu.edu/course/15-121/labs/HW-3%20Josephus/lab.html

        assert_eq!(josephus_linear(5, 2), 2);
        assert_eq!(josephus_linear(5, 1), 4);
        assert_eq!(josephus_linear(66, 100), 6);
        assert_eq!(josephus_linear(1000, 123), 1);
    }
}
