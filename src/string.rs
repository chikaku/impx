//! 字符串相关

/// 最小表示法
///
/// 对于一个字符串 s 选定一个索引 i 通过 `s[i..]+s[..i]` 组成的字符串 t 是 s 的循环同构  
/// 最小表示问题指的是对于一个字符串 s 找到一个字典序最小的循环同构 t
///
/// 考虑 s 的两个循环同构 t1 和 t2 他们的起始索引是在 s 的 i 和 j 处  
/// 假设 t1 和 t2 的前 k 个字符相同即 `s[i..i+k] == s[j..j+k]`  
/// 现在考虑 `a = s[i+k]` 和 `b = s[j+k]`  
///
/// - 如果 `a > b` 那么最小表示肯定不会在 `[i, i+k]` 因为在这之间的每个 `i+n` 总能找到一个 `j+n` 比其更小
/// - 同理 `a < b` 那么最小表示肯定不会在 `[j, j+k]` 因为在这之间的每个 `j+n` 总能找到一个 `i+n` 比其更小
pub fn minimal_string(s: &str) -> usize {
    let n = s.len();
    let s = s.as_bytes();
    let (mut i, mut j, mut k) = (0, 1, 0);

    while i < n && j < n && k < n {
        if s[(i + k) % n] == s[(j + k) % n] {
            k += 1;
            continue;
        }

        if s[(i + k) % n] > s[(j + k) % n] {
            i = i + k + 1;
        } else {
            j = j + k + 1;
        }

        k = 0;
        if i == j {
            j = i + 1;
        }
    }

    // 这里较大的那个可能会越界所以需要取最小值
    std::cmp::min(i, j)
}

/// manacher 算法
///
/// manacher 算法主要用来处理回文字符串  
/// 朴素算法在查找回文串的时候先找到一个中心点然后查询左右两边的字符是否相等, 如相等则可以继续向外扩展  
/// 这种方式找到所有回文串的时间复杂度是 O(n)
///
/// 现在先观察一个回文串的特征比如 `abacaba` 在从前往后的过程中可以发现
///
/// - 以第一个 b 为中心有一个回文串半径为 2
/// - 以第一个 c 为中心有一个回文串半径为 4
///
/// 由于回文串的性质, 其中心左右两臂的字符串是对称的, 假设有一个以 mid 为中心点回文串左右边界为 `[l, r]`  
/// 如果 mid 左边有一个点 mid-x 以 mid-x 为中心的半径为 m 根据对称性质  
/// 那么 mid 右边对称的点 mid+x 以 mid+x 为中心的半径一定也为 m !  
/// 等等！！我们是根据对称性质推导出这个结果的  
/// 但是如果左边 mid-x 为中心的回文串半径超过了 mid 即 `mid-x+m > mid` 那么超出的部分是不对称的  
/// 但是在对称的范围内 mid-x 和 mid+x 的半径还是相同的
///
/// 以上关于臂长和中心点都是按照奇数长度的回文串来说的比如 `abcba` 的臂长是 3 中心点是 `c`  
/// 但是对于偶数长度的回文串如 `abba` 处理方式可能不一样为了方便可以在给定字符串的字符之间插入特定字符  
/// 比如 `abcba` 插入 `#` 变成 `a#b#c#b#a` 所有回文串都是奇数长度回文串的性质不变  
/// 对于 `abba` 插入 `#` 变成 `a#b#b#a` 所有回文串都是奇数长度
///
/// 根据上面的思想, 我们可以做如下处理:
///
/// - 首先字符串插入特殊值
/// - 对字符串从前往后进行遍历, 依次根据朴素算法查找回文串
/// - 如果找到了一个回文串将其臂长 `d[i]` 记录下来, 并记录当前臂长最大的那个回文串的左右索引 `[l, r]`
/// - 对于最长回文串右臂内的一个索引 j 查找与其对称的左臂内的索引的回文串臂长 `d[l + r - j]`
/// - 根据对称性 j 可以从此臂长开始再向外扩展查找回文串
pub fn manacher(s: &str) -> (usize, usize) {
    let s = s.as_bytes();
    let mut t = vec![b'#'];
    for &c in s {
        t.push(c);
        t.push(b'#');
    }

    let n = t.len();

    // 当前最长回文串的左右边界
    let (mut l, mut r) = (0, 0);

    // d[i] 表示 以 i 为中心点的回文串臂长
    let mut d = vec![0; n];

    for i in 0..n {
        let mut k = 1;
        if i <= r {
            // 计算对称点的臂长, 但是不能超出当前最大臂长
            k = std::cmp::min(d[l + r - i], r - i + 1);
        }

        // 扩展臂长
        while k <= i && i + k < n && t[i - k] == t[i + k] {
            k += 1;
        }

        // 保存当前索引臂长
        k -= 1;
        d[i] = k;

        // 更新最长臂长
        if i + k > r {
            l = i - k;
            r = i + k;
        }
    }

    // 查找最长臂长及其对应的索引
    let (mut max_mid, mut max_r) = (0, 0);
    for (i, &v) in d.iter().enumerate() {
        if v > max_r {
            (max_mid, max_r) = (i, v);
        }
    }

    // 计算出原始字符串中的索引
    ((max_mid - max_r) / 2, (max_mid + max_r) / 2 - 1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_minimal_string() {
        use super::*;

        assert_eq!(minimal_string("cbda"), 3);
        assert_eq!(minimal_string("abadacc"), 0);
        assert_eq!(minimal_string("acabadaa"), 6);
    }

    #[test]
    fn test_manacher() {
        use super::*;

        assert_eq!(manacher("xabbac"), (1, 4));
        assert_eq!(manacher("xabcbac"), (1, 5));
        assert_eq!(manacher("aacaabbacabb"), (5, 11));
    }
}
