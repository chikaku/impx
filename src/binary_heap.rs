//! 二叉堆

pub struct BinaryHeap<T> {
    nodes: Vec<T>,
}

/// 构建二叉堆
pub fn build_heap<T: Copy + PartialOrd>(vs: &[T]) -> BinaryHeap<T> {
    let mut nodes = vs.to_vec();
    for i in (0..=nodes.len() / 2).rev() {
        down(&mut nodes, i);
    }

    BinaryHeap { nodes }
}

impl<T: Copy + PartialOrd> BinaryHeap<T> {
    /// 推入元素
    pub fn push(&mut self, v: T) {
        self.nodes.push(v);
        let idx = self.nodes.len() - 1;
        up(&mut self.nodes, idx);
    }

    /// 弹出当前最大元素
    pub fn pop(&mut self) -> Option<T> {
        if self.nodes.is_empty() {
            return None;
        }

        let last = self.nodes.len() - 1;
        self.nodes.swap(0, last);
        let value = self.nodes.pop();
        down(&mut self.nodes, 0);

        value
    }
}

fn up<T: PartialOrd>(v: &mut [T], mut root: usize) {
    while root > 0 && v[root] > v[(root - 1) / 2] {
        v.swap(root, (root - 1) / 2);
        root = (root - 1) / 2;
    }
}

fn down<T: PartialOrd>(v: &mut [T], root: usize) {
    let n = v.len();
    let mut max_idx = root;
    let left = root * 2 + 1;
    let right = root * 2 + 2;

    if left < n && v[left] > v[max_idx] {
        max_idx = left;
    }

    if right < n && v[right] > v[max_idx] {
        max_idx = right;
    }

    if max_idx != root {
        v.swap(max_idx, root);
        down(v, max_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_heap() {
        let vs = [1, 5, 2, 9, 4, 7];
        let mut h = build_heap(&vs);

        assert_eq!(h.pop(), Some(9));
        assert_eq!(h.pop(), Some(7));
        assert_eq!(h.pop(), Some(5));
        assert_eq!(h.pop(), Some(4));

        // 1 2
        h.push(3);
        h.push(0);
        h.push(7);

        assert_eq!(h.pop(), Some(7));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), Some(2));
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.pop(), Some(0));
        assert_eq!(h.pop(), None);
    }
}
