//! 矩阵快速幂
//!
//! - [矩阵快速幂](https://www.desgard.com/algo/docs/part2/ch01/3-matrix-quick-pow/)
//!
//! 观察斐波那契递推公式:
//!
//! ```text
//! f(n)   = f(n-1) + f(n-2)
//! f(n-1) = f(n-1)
//! ```
//!
//! 等价于一个线性方程组
//!
//! ```text
//! y1 = x1 + x2
//! y2 = x1
//! ```
//!
//! 然后将其改写成 `A·X = B` 的矩阵形式:
//!
//! ```text
//! | y1 | = | 1 1 | | x1 |
//! | y2 | = | 1 0 | | x2 |
//!
//! 等价于
//!
//! | f(n)   | = | 1 1 | | f(n-1) |
//! | f(n-1) | = | 1 0 | | f(n-2) |
//! ```
//!
//! 然后可以改写成函数形式 `F(n) = X·F(n-1)` 则 Fn 可以看作是一个等比数列可以写出其通项公式
//!
//! ```text
//! F(n) = X^(n-1)·F(1)
//! ```
//!
//! 则通过矩阵快速幂计算将斐波那契第 N 项的计算时间复杂度从 O(N) 降低至 O(logN)

use std::ops::{AddAssign, Index, IndexMut, Mul};

#[derive(Clone, Copy)]
pub struct Matrix<T, const M: usize, const N: usize>([[T; N]; M]);

impl<T, const M: usize, const N: usize> Index<usize> for Matrix<T, M, N> {
    type Output = [T; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const M: usize, const N: usize> IndexMut<usize> for Matrix<T, M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/// 实现矩阵乘法 (m * n)·(n * p) = (m * p)
impl<T, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, N, P>> for Matrix<T, M, N>
where
    T: Default + Copy + Clone + Mul<Output = T> + AddAssign,
{
    type Output = Matrix<T, M, P>;

    fn mul(self, rhs: Matrix<T, N, P>) -> Self::Output {
        let mut res = [[T::default(); P]; M];

        for i in 0..M {
            for j in 0..P {
                for k in 0..N {
                    res[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }

        Matrix(res)
    }
}

/// 计算正方矩阵的 n 次幂
pub fn matrix_pow<const M: usize>(mut x: Matrix<i32, M, M>, mut n: usize) -> Matrix<i32, M, M> {
    let mut res = Matrix([[0; M]; M]);
    for i in 0..M {
        res[i][i] = 1;
    }

    while n > 0 {
        if n & 1 == 1 {
            res = res * x;
        }
        x = x * x;
        n >>= 1;
    }

    res
}

/// 使用矩阵快速幂求斐波那契数
pub fn fib(n: usize) -> i32 {
    if n == 0 {
        return 0;
    }

    // base 矩阵即 [fib(1), fib(0)]
    let base = Matrix([[1], [0]]);

    // 乘数矩阵
    let x = Matrix([[1, 1], [1, 0]]);

    // 结果矩阵
    let res = matrix_pow(x, n - 1) * base;

    res[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(5), 5);
        assert_eq!(fib(7), 13);
        assert_eq!(fib(17), 1597);
        assert_eq!(fib(19), 4181);
    }
}
