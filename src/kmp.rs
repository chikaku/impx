//! KMP 算法
//!
//! - [OI Wiki - 前缀函数与 KMP 算法](https://oi-wiki.org/string/kmp/)

/// 前缀函数
///
/// 前缀函数计算对于一个给定长度为 n 的字符串 s 计算出一个长度为 n 的数组 p  
/// 其中 `p[i]` 表示子串 `s[0..i]` 具有的相同的真前缀和真后缀的长度的最大值  
///
/// 如对于字符串 `abcabcd`  
/// 当 i 为 3 时子串为 `abca` 长度相等的真前缀和真后缀有 `(a, a)` 则其最大值为 1  
/// 当 i 为 4 时子串为 `abcab` 长度相等的真前缀和真后缀有 `(a, a)` 和 `(aa, aa)`则其最大值为 2  
///
/// 计算前缀函数的方式
///
/// - 朴素算法: 从 1~n 遍历 i 接着在字串内从 i~1 遍历 j 如果找到相同的前后缀即可退出此次 j 遍历
/// - 优化算法1: `p[i+1] <= p[i]+1` 很显然因为从 `p[i]` 到 `p[i+1]` 只在最后面增加了 s[i+1] 这一个字符最好的情况下只 +1
/// - 优化算法2: 如果新增的字符 `s[i+1] != s[p[i]]`
///     - 那 `p[i+1]` 所表示的前后缀肯定是比 `p[i]` 要短如果按照优化算法1的思路可以从 `p[i]-1` 向下遍历
///     - 上面说过 `p[i]` 表示的是 `s[0..i]` 内相同真前后缀的最大长度
///     - 假设 `s[i]` 内的真前后缀的长度有 1 3 5 则 `p[i] = 5`
///     - 回到 `s[i+1] != s[p[i]]` 我们可以不必从 `p[i]-1` 向下遍历, 只需要找到一个比 `p[i]` 低的那个相同前后缀的长度
///     - 假设这个只比 `p[i]` 低的相同前后缀的长度为 j 那么只需要检查是否 `s[i+1]` 和 `s[j]` 是否相等
///     - 如果 `s[i+1] == s[j]` 那么最大前后缀长度就能在 j 的基础上+1即 `p[i+1] == j+1` 不需要再进行字符串比较
///     - 因为这里 j 已经是一个相同前后缀的长度即 `s[..j] == s[(i-j+1)..]` 对于 s[i+1] 只需要多比较一位即可
///     - 如果 `s[i+1] != s[j]` 则继续找到更下一个较低的前后缀长度进行相同的比较即可
///     
/// 如何寻找在 `s[..i]` 内仅次于 `p[i]` 的 j 呢?
///
/// - 假设存在这个长度 j 那么由于 j 是仅次于 `p[i]` 的相同前后缀的长度
/// - 这个长度为 j 的真前缀一定包含在长度为 `p[i]` 的真前缀内
/// - 这个长度为 j 的真后缀一定包含在长度为 `p[i]` 的真后缀内
/// - 这个时候把 `s[0..p[i]]` 视作一个子串 `sp` 长度为 `p[i]`
/// - 那 j 一定是这个子串 `sp` 内的相同的真前缀和真后缀的长度的最大值
/// - 即可得到 `j = p[len(sp)-1] = p[len(p)-1]`
/// - 由此可以得到一个状态转移方程对于长度第 n 大的相同前后缀 `j(n) = p[j(n-1)-1] (j(n-1) > 0)`
///
/// ```
/// use impx::kmp::prefix_n;
///
/// let p = prefix_n("abcab");
/// assert_eq!(2, p[4]); // 最大相同前后缀为 ab
/// ```
pub fn prefix_n(s: &str) -> Vec<usize> {
    let n = s.len();

    // 朴素算法
    let mut p = vec![0; n];
    for i in 1..n {
        // 字串为 S[..i] 长度为 i+1
        for j in (0..(i + 1)).rev() {
            // 在字串中分别从大到小取 j 个测试是否真前后缀相同
            if s[..j] == s[(i + 1 - j)..(i + 1)] {
                p[i] = j;
                break;
            }
        }
    }

    // 优化算法1
    let mut p = vec![0; n];
    for i in 1..n {
        // 字串为 S[..i] 长度为 i+1
        for j in (0..=(p[i - 1] + 1)).rev() {
            // p[i] <= p[i-1]+1 直接从 i-1 开始遍历
            if s[..j] == s[(i + 1 - j)..(i + 1)] {
                p[i] = j;
                break;
            }
        }
    }

    // 优化算法2
    let chars = s.as_bytes();
    let mut p = vec![0; n];
    for i in 1..n {
        // 先找到上一个子串的最大相同前后缀长度
        let mut j = p[i - 1];

        // 如果 s[i] != s[j] 则需要找到前一个子串的次级最大相同前后缀长度 j
        // 然后继续比较直到 j 为 0
        // 或者当前子串的最后一个字符 chars[i] 与最大前缀的后一个字符 chars[j] 相等
        while j > 0 && chars[i] != chars[j] {
            j = p[j - 1];
        }

        // 这里需要判断下是什么原因退出的循环
        // 如果 chars[i] == chars[j] 直接将 j += 1 即可
        // 即使此时 j == 0 也不影响
        // 如果 chars[i] != chars[j] 则是因为 j == 0 退出的循环
        if chars[i] == chars[j] {
            j += 1;
        }

        p[i] = j;
    }

    p
}

/// Knuth-Morris-Pratt 算法
///
/// 这里的 kmp 用来在字符串 s 中寻找第一次出现的子串 t  
/// kmp 利用前缀函数的信息来减少重复的比较
///
/// 假设有一个主串 `ababxxx...` 和一个子串 `ababcxxx`  
/// 从 i= 0 比较前四个字符 abab 都匹配第五个 c 和 x 不匹配  
/// 按照正常的办法这里需要将 i+1 再重新进行比较  
/// 但是非常直观的我们可以看出这里已经匹配的子串 abab 其实有一个相同的前后缀 ab  
/// 重新比较时只需要将子串中的第一个 ab 和主串中的 第二个 ab 对齐即可  
/// 也可以说是将已经匹配的子串部分的后缀变成新一轮匹配的已配对前缀即可  
/// 这样做的好处是不需要回滚 i 只需要更新当前在子串上已匹配的个数 k
///
/// ```text
///     i        主串匹配索引 i  
///     ↓       
/// ababxxx      当前产生了不匹配  
/// ababcxxx     第一轮匹配结束, 此处已经配对的是 [abab] 具有相同的前后缀 [ab]  
///   ababcxxx   将已配对的后缀 [ab] 变成新一轮匹配的前缀当前已经配对的是 [ab] 不需要回滚主串上的索引 i
/// ```
///
/// 即使用 KMP 算法寻找子串流程如下:
///
/// - 先计算子串 t 的前缀函数 P
/// - 遍历主串 s 并保存当前已经匹配的数量 k
/// - 如果 `s[i] == p[k]` 那么将已经匹配的数量 +1
/// - 如果 `s[i] == p[k]` 而且之前已经有匹配的部分即 `k > 0`
///     - 通过子串的前缀函数 P[k-1] 可以得到跟当前已经匹配部分的前缀相同的最大后缀长度
///     - 将这个最大后缀替换为当前匹配的前缀即可, 即将已经匹配的长度 k 修改为最大后缀长度
pub fn kmp(s: &str, t: &str) -> Option<usize> {
    let p = prefix_n(t);
    let t = t.as_bytes();
    let n = t.len();

    let mut k = 0; // 已经匹配的数量
    for (i, &v) in s.as_bytes().iter().enumerate() {
        // 已经有部分匹配, 但是下一个不匹配
        // 这个时候需要根据前缀函数将相同后缀作为新一轮匹配的前缀
        if k > 0 && v != t[k] {
            k = p[k - 1]
        }

        if v == t[k] {
            k += 1;
        }

        if k == n {
            return Some(i + 1 - k);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_prefix_n() {
        use super::*;

        let p = prefix_n("abcabcd");
        assert_eq!(&p, &[0, 0, 0, 1, 2, 3, 0]);

        let s = "abaabbbaabbaaaabaab";
        let p = prefix_n(s);
        for (i, &v) in p.iter().enumerate() {
            assert_eq!(s[..v], s[(i + 1 - v)..(i + 1)])
        }
    }

    #[test]
    fn test_kmp() {
        use super::*;

        assert_eq!(Some(0), kmp("aaaaa", "aaaa"));
        assert_eq!(Some(2), kmp("abababxxc", "ababx"));
        assert_eq!(Some(3), kmp("abababxxc", "babx"));
        assert_eq!(Some(5), kmp("klslapwosldkal", "pwo"));
        assert_eq!(Some(2), kmp("pqpsapspsp", "ps"));
        assert_eq!(Some(6), kmp("bacbadababacamcaddababaca", "ababaca"),)
    }
}
