//! CRC32 算法
//!
//! - [CRC校验原理及实现](https://zhuanlan.zhihu.com/p/256487370)
//! - [CRC检验算法原理及其Java实现](https://www.klavor.com/dev/20190618-552.html)
//! - [CRC查表法运算原理](https://blog.csdn.net/zhaojia92/article/details/116886307)
//!
//! CRC32 是循环冗余校验(Cyclic Redundancy Check)算法的一种, 主要用于校验数据的正确性  
//! CRC 算法的原理是: 将源数据看作一个二进制被除数, 与一个预定义的二进制初始值相除取余数得到的  
//! 注意这里减法是不借位的 0-0=0 0-1=1 1-0=1 1-1=0 与异或运算相同(加法也是同理)  
//!
//! 对于除法比如 1000001 除以 101
//!
//! ```text
//! // 第一次运算(101往左移动 4 位)得到的结果不够除以直接再往后移一位
//! 1000001
//! 101  
//!   10
//!   100
//!
//! // 第二次运算(101往左移动 2 位)还是不够再往后移一位
//!   10001
//!   101
//!     101
//!
//! // 第三次运算(101往左移动 0 位)最后的商是 10101
//!     101
//!     101
//!       0
//! ```
//!
//! 具体的 CRC 算法具有以下参数模型:
//!
//! - WIDTH: 生成的 CRC 数据位宽
//! - POLY: 多项式除数, 在使用时忽略最高位的 1
//! - INIT: 使用的 CRC 初始值
//! - REFIN: 计算前源数据是否左右翻转
//! - REFOUT: 计算后的结果是否左右翻转
//! - XOROUT: 计算后的结果与此值进行异或
//!
//! 对于一种比较具体的算法如 CRC32 在进行计算的时候可以通过查表的方式减少运算次数  
//! 比如将源数据的 8bit 为一组, 实际上每组运算的数据只是跟除数除去 8 次即取余 8 次的结果
//!
//! ```text
//!           |11111111|
//!    1111111|1       |   第一次运算
//!     111111|11      |  第二次运算
//!      11111|111     |   第三次运算
//!       1111|1111    |   第四次运算
//!      ...           ...
//! ```
//!
//! 由于除数是预定义的, 我们可以提前计算好这个 8bit 的所有情况下的异或值(减法)  
//! 在实际计算过程中 8bit 为一组直接从表中取需要进行异或的值即可

static TABLE_CRC32: [u32; 256] = make_crc32_table();

pub fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFFFFFF;
    for &v in data {
        let index: u32 = (crc ^ (v as u32)) & 0xFF;
        crc = (crc >> 8) ^ TABLE_CRC32[index as usize];
    }

    crc ^ 0xFFFFFFFF
}

// 计算 CRC32 表
const fn make_crc32_table() -> [u32; 256] {
    let poly = reverse_u32(0x04C11DB7);

    let mut table = [0; 256];
    let mut i: usize = 0;

    while i < 256 {
        let mut v: u32 = i as u32;
        let mut j = 0;
        while j < 8 {
            if v & 0x01 == 1 {
                v = (v >> 1) ^ poly;
            } else {
                v >>= 1;
            }
            j += 1;
        }

        table[i] = v;
        i += 1;
    }

    table
}

// 左右翻转 32 位数据
const fn reverse_u32(u: u32) -> u32 {
    let mut v = 0;
    let mut i = 0;
    while i < 32 {
        v <<= 1;
        v |= (u >> i) & 1;
        i += 1;
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_u32() {
        let a = 0b00110101000101010100100101100101;
        let b = 0b10100110100100101010100010101100;
        assert_eq!(reverse_u32(a), b);
    }

    #[test]
    fn test_crc32() {
        assert_eq!(crc32("impx".as_bytes()), 0x5684BF1E);
        assert_eq!(crc32("crc32".as_bytes()), 0xAFABD35E);
        assert_eq!(crc32("aaa123".as_bytes()), 0x22AB0907);
        assert_eq!(crc32("00000000".as_bytes()), 0xC0088D03);
        assert_eq!(crc32("10011001".as_bytes()), 0xFE79F3DE);
    }
}
