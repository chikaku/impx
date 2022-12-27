//! 数组相关算法

/// Boyer–Moore 多数投票算法
///
/// 多数投票算法用于解决寻找数组中的多数元素, 这里多数指的是数量占总数一半以上流程如下:
///
/// - 设置一个计数器 count 和当前多数元素 e
/// - 遍历元素如果当前 count 为 0 则将多数元素设置为当前元素
/// - 否则如果当前元素等于 e 则 count+1 否则 count-1
pub fn majority_vote<T: std::cmp::Eq>(arr: &[T]) -> &T {
    let (mut count, mut e) = (0, &arr[0]);
    for v in arr {
        if count == 0 {
            (e, count) = (v, 1);
            continue;
        }

        if v == e {
            count += 1;
        } else {
            count -= 1;
        }
    }

    e
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_vote() {
        assert_eq!(majority_vote(&[1, 2, 1]), &1);
        assert_eq!(majority_vote(&[1, 1, 1]), &1);
        assert_eq!(majority_vote(&[0, 1, 1]), &1);
        assert_eq!(majority_vote(&[1, 2, 2, 3]), &2);
        assert_eq!(majority_vote(&[1, 3, 1, 2, 3, 3, 3, 4, 3]), &3);
    }
}
