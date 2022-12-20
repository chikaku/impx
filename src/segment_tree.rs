//! 线段树
//!
//! - [OI Wiki - 线段树](https://oi-wiki.org/ds/seg/)
//!
//! 线段树是一种用来维护区间信息的数据结构, 可以做到在 O(logn) 的时间复杂度内区间求和和区间最值查询  
//! 对于一个长度为 n 的数组 a 可以根据数组构建出一棵完全二叉树
//!
//! - 其中根节点管辖此数组 [0, n-1] 内的最大值这个值暂时没把法直接计算出
//! - 先计算出区间的中间值 mid
//! - 递归构建左子树: 使用左节点管辖此数组 [0, mid] 内的最大值
//! - 递归构建右子树: 使用右节点管辖此数组 [mid+1, n-1] 内的最大值
//! - 对于一个区间 [s, t] 如果 `s == t` 则此区间内的最大值就是 `a[s]`
//! - 左右子树构建完成后可以计算出根节点的 sum 值为左右子树的 sum 之和
//!
//! 如果需要查询区间 `[s, t]` 之和可以将此区间分割成树中保存了的区间然后相加即可
//!
//! - 原始的最大区间是 `[0, n-1]` 这个范围肯定是大于等于 `[s, t]` 的
//! - 先计算出区间内的中间点 mid 然后如果 `s <= mid` 那么 `[s, t]` 肯定有一段在 `[0, mid]` 内
//! - 同理如果 `mid+1 <= t` 那么 `[s, t]` 肯定有一段在 `[mid+1, n-1]` 内
//! - 然后递归子节点在 `[0, mid]` 内查询 `[s, mid]` 的和
//! - 同理递归子节点在 `[mid+1, n-1]` 内查询 `[mid+1, n-1]` 的和
//! - 如果有一个区间 `[l, h]` 在 `[s, t]` 内部则直接把 `[l, h]` 的 sum 返回
//! - 递归所有子区间求和即可
//!
//! 高效更新单个节点或者更新整个区间比如现在要在区间 `[s, t]` 内全部全部加上 `v`  
//! 当然可以简单的更新 `[s, t]` 的所有节点, 但是实际上对于 `[s, t]` 区间的求和子节点是不被读取的  
//! 所以可以直接更新 `[s, t]` 区间所在节点的 sum 值然后添加一个懒标记  
//! 直到需要计算 `[s, t]` 区间内的子节点时才有更新子节点的 sum 值

/// 线段树节点
#[derive(Default, Clone, Copy)]
pub struct Node {
    pub low: usize,       // 节点管辖左区间
    pub high: usize,      // 节点管辖的右区间
    pub index: usize,     // 节点在线段树中的索引
    pub sum: isize,       // 节点所管辖区间内元素和
    pub lazy_mark: isize, // 懒标记表示此区间内有数据修改但是还没有更新到下方到子区间内
}

/// 线段树
pub struct SegmentTree {
    tree: Vec<Node>,
}

/// 构建线段树
///
/// 类似于最大/最小堆从 `[0, n-1]` 开始建立根节点, 然后每次取中点分别建立左右子节点  
/// 最后更新根节点的 sum 为左右子节点 sum 之和
pub fn build(index: usize, low: usize, high: usize, v: &[isize], t: &mut [Node]) {
    let mut node = t[index];
    node.low = low;
    node.high = high;
    node.index = index;

    if low == high {
        node.sum = v[low];
        t[index] = node;
        return;
    }

    // 从中间切开构建左右子树
    let mid = low + ((high - low) >> 1);
    build(index * 2 + 1, low, mid, v, t);
    build(index * 2 + 2, mid + 1, high, v, t);

    // 根节点的 sum 是左右两子节点的 sum 的和
    node.sum = t[index * 2 + 1].sum + t[index * 2 + 2].sum;
    t[index] = node;
}

impl SegmentTree {
    /// 根据输入数组建立线段树
    pub fn new(v: &[isize]) -> Self {
        let n = v.len();
        let mut tree = vec![Node::default(); n * 4];
        build(0, 0, n - 1, v, &mut tree);

        Self { tree }
    }

    /// 线段树修改区间数据, 区间内的每个元素的增加值为 diff
    pub fn update(&mut self, low: usize, high: usize, diff: isize) {
        self.update_node(0, low, high, diff);
    }

    fn update_node(&mut self, index: usize, low: usize, high: usize, diff: isize) {
        let mut node = self.tree[index];
        if low <= node.low && node.high <= high {
            node.sum += ((node.high - node.low + 1) as isize) * diff;
            node.lazy_mark += diff;
            self.tree[node.index] = node;
            return;
        }

        // 如有未更新的标记, 先更新到下一层
        if node.low != node.high && node.lazy_mark > 0 {
            let mut left = &mut self.tree[node.index * 2 + 1];
            left.sum += ((left.high - left.low + 1) as isize) * node.lazy_mark;
            left.lazy_mark += node.lazy_mark;

            let mut right = &mut self.tree[node.index * 2 + 2];
            right.sum += ((right.high - right.low + 1) as isize) * node.lazy_mark;
            right.lazy_mark += node.lazy_mark;

            node.lazy_mark = 0;
        }

        let mid = node.low + ((node.high - node.low) >> 1);
        let left_index = node.index * 2 + 1;
        let right_index = node.index * 2 + 2;

        // 如果左节点在区间内
        if low <= mid {
            self.update_node(left_index, low, mid, diff);
        }

        // 如果右节点在区间内
        if mid < high {
            self.update_node(right_index, mid, high, diff);
        }

        node.sum = self.tree[left_index].sum + self.tree[right_index].sum;
        self.tree[node.index] = node;
    }

    /// 线段树获取区间 `[low, high]` 内元素之和
    pub fn sum(&mut self, low: usize, high: usize) -> isize {
        self.sum_node(0, low, high)
    }

    fn sum_node(&mut self, index: usize, low: usize, high: usize) -> isize {
        let mut node = self.tree[index];
        if low <= node.low && node.high <= high {
            return node.sum;
        }

        // 如有未更新的标记, 先更新到下一层
        if node.low != node.high && node.lazy_mark > 0 {
            let mut left = &mut self.tree[node.index * 2 + 1];
            left.sum += ((left.high - left.low + 1) as isize) * node.lazy_mark;
            left.lazy_mark += node.lazy_mark;

            let mut right = &mut self.tree[node.index * 2 + 2];
            right.sum += ((right.high - right.low + 1) as isize) * node.lazy_mark;
            right.lazy_mark += node.lazy_mark;

            node.lazy_mark = 0;
        }

        let mut sum = 0;
        let mid = node.low + ((node.high - node.low) >> 1);

        // 如果左节点在区间内
        if low <= mid {
            sum += self.sum_node(index * 2 + 1, low, high);
        }

        // 如果右节点在区间内
        if mid < high {
            sum += self.sum_node(index * 2 + 2, low, high);
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_segment_tree() {
        use super::*;

        let v = [1, 2, 3, 4, 5, 6];
        let mut t = SegmentTree::new(&v);
        assert_eq!(5, t.sum(1, 2));
        assert_eq!(6, t.sum(5, 5));

        // [1, 2, 0, 4, 5, 6];
        t.update(2, 2, -3);
        assert_eq!(2, t.sum(1, 2));
        assert_eq!(11, t.sum(4, 5));

        // [1, 3, 1, 5, 6, 6];
        t.update(1, 4, 1);
        assert_eq!(5, t.sum(0, 2));
        assert_eq!(21, t.sum(1, 5));

        // [1, 3, 1, 5, 6, 6];
        t.update(0, 5, 0);
        assert_eq!(5, t.sum(0, 2));
        assert_eq!(21, t.sum(1, 5));
    }
}
