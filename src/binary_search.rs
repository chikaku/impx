//! 二分搜索
//!
//! <https://oi-wiki.org/basic/binary/>
//!
//! 时间复杂度: 最优 O(1) 最坏 O(logn)
//!
//! 二分搜索用于在**有序**序列中查找目标值, 这里有序的概念可以参考 <https://oi-wiki.org/basic/binary/#最大值最小化>
//!
//! 在编写二分搜索时需要注意上下界条件以及何时停止  
//! 一种写法是下界使用索引, 上界使用最右边的索引+1 即 [0, n) 然后终止条件是 `low < high`  
//! 但是这种写法的 `mid` 值不容易计算, 在大于或者小于的情况下一个要加 1 一个不用剪去 1 比较容易弄混  
//! 另一般比较容易理解的方式是, 上下界都使用具体的索引 [0, n-1] 终止条件变成是 `low <= high`  
//! 这种写法的好处是在大于或者小于的情况下 `low = mid+1` `high = mid - 1` 都有一个加减一的逻辑且 `mid` 比较好计算  
//! 具体的细节可以参考下面给出的使用二分搜索的算法实现源码

/// 二分查找目标值所在索引
pub fn binary_search<T: Ord>(v: &[T], target: &T) -> Option<usize> {
    let (mut low, mut high) = (0, v.len() - 1);
    while low <= high {
        let mid = low + ((high - low) >> 1);
        match v[mid].cmp(target) {
            std::cmp::Ordering::Less => {
                low = mid + 1;
            }
            std::cmp::Ordering::Equal => {
                return Some(mid);
            }
            std::cmp::Ordering::Greater => {
                if mid == 0 {
                    // avoid subtract with overflow
                    return None;
                }
                high = mid - 1;
            }
        }
    }

    None
}

/// 二分查找符合搜索函数的最小索引
///
/// 这里对搜索函数有一个限制, 搜索函数的方向必须和有序的方向是一致的  
/// 比如对于一个递增数组, 搜索函数一定是大于等于某个数而不能是小于或等于某个数  
/// 因为搜索比 x 小的元素的最小索引要么是 0 要么不存在  
/// 同理如果用此函数搜索某个具体元素是否存在时要使用 `x >= 1` 这种方式
///
/// ```
/// use impx::binary_search::binary_search_first;
///
/// assert_eq!(binary_search_first(&[1, 2, 2, 3], |&x| x >= 2), Some(1));
/// ```
pub fn binary_search_first<T, F>(v: &[T], cmp: F) -> Option<usize>
where
    T: Ord,
    F: Fn(&T) -> bool,
{
    let mut index = None;
    let (mut low, mut high) = (0, v.len() - 1);
    while low <= high {
        let mid = low + ((high - low) >> 1);
        if cmp(&v[mid]) {
            index = Some(mid);
            if mid == 0 {
                // avoid subtract with overflow
                return index;
            }
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    index
}

/// 二分查找符合搜索函数的最大索引
///
/// 这里对搜索函数有一个限制, 搜索函数的方向必须和有序的方向是相反的  
/// 比如对于一个递增数组, 搜索函数一定是小于等于某个数而不能是大于等于某个数  
/// 因为搜索比 x 大的元素的最大索引要么是最后一个元素要么不存在  
/// 同理如果用此函数搜索某个具体元素是否存在时要使用 `x <= 1` 这种方式
///
/// ```
/// use impx::binary_search::binary_search_last;
///
/// assert_eq!(binary_search_last(&[1, 2, 2, 3], |&x| x <= 2), Some(2));
/// ```
pub fn binary_search_last<T, F>(v: &[T], cmp: F) -> Option<usize>
where
    T: Ord,
    F: Fn(&T) -> bool,
{
    let mut index = None;
    let (mut low, mut high) = (0, v.len() - 1);
    while low <= high {
        let mid = low + ((high - low) >> 1);
        if cmp(&v[mid]) {
            index = Some(mid);
            low = mid + 1;
        } else {
            if mid == 0 {
                // avoid subtract with overflow
                return index;
            }
            high = mid - 1;
        }
    }

    index
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_binary_search() {
        use super::binary_search as search;

        assert!(search(&[1, 3, 5, 6, 7, 9], &0).is_none());
        assert!(search(&[1, 3, 5, 6, 7, 9], &2).is_none());
        assert!(search(&[1, 2, 2, 5, 5, 7, 8, 8, 9], &2).is_some());
        assert!(search(&[1, 2, 2, 5, 5, 7, 8, 8, 9], &8).is_some());

        assert_eq!(search(&[1, 3, 5, 6, 7, 9], &1), Some(0));
        assert_eq!(search(&[1, 3, 5, 6, 7, 9], &9), Some(5));
        assert_eq!(search(&[1, 2, 2, 5, 5, 7, 8, 8, 9], &7), Some(5));
    }

    #[test]
    fn test_binary_search_first() {
        use super::binary_search_first as search;

        assert_eq!(search(&[1, 1, 2, 2, 3, 3], |&x| x > 0), Some(0));
        assert_eq!(search(&[1, 1, 2, 2, 3, 3], |&x| x >= 2), Some(2));
        assert_eq!(search(&[1, 1, 2, 2, 3, 3, 4], |&x| x > 4), None);
        assert_eq!(search(&[6, 5, 4, 3, 2, 1], |&x| x <= 4), Some(2));
    }

    #[test]
    fn test_binary_search_last() {
        use super::binary_search_last as search;

        assert_eq!(search(&[1, 1, 2, 2, 3, 3], |&x| x < 3), Some(3));
        assert_eq!(search(&[1, 1, 2, 2, 3, 3], |&x| x <= 2), Some(3));
        assert_eq!(search(&[1, 1, 2, 2, 3, 3, 4], |&x| x < 1), None);
        assert_eq!(search(&[6, 5, 4, 3, 2, 1], |&x| x >= 4), Some(2));
    }
}
