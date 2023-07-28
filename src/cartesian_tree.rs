//! 笛卡尔树
//!
//! 参考:
//!
//! - [OI Wiki - 笛卡尔树](https://oi-wiki.org/ds/cartesian-tree/)
//!
//! 笛卡尔树是一种二叉树，每个节点是一个元组 `(k, w)`  
//! 其中 k 保证节点二叉搜索树的性质 w 保证节点堆的性质  
//! (很多情况下都是直接将数列的索引作为 k 了)
//!
//! 以最小堆为例考虑构建笛卡尔树的过程:
//!
//! 先考虑两个节点 `node1(k1, w1)` 和 `node2(k2, w2)`  
//! 如果 k1 < k2 则根据二叉搜索树性质 k1 应该在 k2 左边，即 k1 是 k2 的左子树，或者 k2 是 k1 的右子树  
//! 再通过最小堆性质，如果 w1 小于 w2 则 node1 是 node2 的父节点，如果 w1 大于 w2 则 node2 应该是 node1 的父节点  
//! 这样就可以确定两个节点的位置关系
//!
//! 考虑三个按 k 递增的节点 (1, w) (2, w) (3, w) 则三个节点的位置关系是确定的一定是 1 在 2 左边，而 2 在 3 左边  
//! 可能构成 1 <- 2 <- 3 或者 1 -> 2 -> 3 或者 1 <- 2 -> 3 左右顺序是固定的  
//! 只要再根据他们的 w 值确定节点父子节点关系即可
//!
//! 现在考虑任意序列，可以先将序列按照 k 排序后续按照以上规则，每次加入到树的节点肯定是最右边的  
//! 所以在构建树时可以维护一个右链(从根节点往一直往右下构成的链), 每个节点从下往上找到在右链上的位置即可
//!
//! ```text
//! // 找到一个小于自己的节点作为父节点
//! while (n.w > x.w) { n = parentof(n) }
//!
//! // 修改子树
//! x.left = n.right
//! n.right = x
//! ```

use std::ptr::NonNull;

/// 笛卡尔树节点
pub struct Node<K, W> {
    pub k: K,
    pub w: W,
    pub left: Option<Box<Node<K, W>>>,
    pub right: Option<Box<Node<K, W>>>,
}

/// 笛卡尔树
pub struct CartesianTree<K, W> {
    pub root: Option<Box<Node<K, W>>>,
}

/// 构建笛卡尔树
pub fn build_cartesian_tree<K, W>(xs: &mut [(K, W)]) -> CartesianTree<K, W>
where
    K: std::cmp::Ord + Clone + Copy,
    W: std::cmp::Ord + Clone + Copy,
{
    // 先按照 k 排好序
    xs.sort_by(|(k0, _), (k1, _)| k0.cmp(k1));

    // 迭代插入节点
    let root = xs.iter().fold(None, |root, &x| insert_into_right(root, x));

    CartesianTree { root }
}

/// 将节点插入到树中，流程很简单直接在右链中找到位置插入即可  
fn insert_into_right<K, W>(root: Option<Box<Node<K, W>>>, x: (K, W)) -> Option<Box<Node<K, W>>>
where
    K: std::cmp::Ord + Clone + Copy,
    W: std::cmp::Ord + Clone + Copy,
{
    match root {
        None => Some(new_node(x)),
        Some(root) => {
            if root.w > x.1 {
                // 比根节点还要小直接替换成根节点
                let mut node = new_node(x);
                node.left = Some(root);
                return Some(node);
            }

            // 因为所有权和可变性的一些问题，需要用 unsafe 处理
            let root_raw = Box::into_raw(root);
            let mut curr_ptr = unsafe { NonNull::new_unchecked(root_raw) };
            let mut curr_mut = unsafe { curr_ptr.as_mut() };

            loop {
                match curr_mut.right.take() {
                    Some(right) => {
                        let right_raw = Box::into_raw(right);
                        let mut right_ptr = unsafe { NonNull::new_unchecked(right_raw) };
                        let right_mut = unsafe { right_ptr.as_mut() };

                        if right_mut.w < x.1 {
                            // 如果右节点还比较小, 再赋值回去往下递归...
                            let old_right = unsafe { Box::from_raw(right_raw) };
                            curr_mut.right = Some(old_right);
                            curr_mut = right_mut;
                        } else {
                            // 否则直接替换此处的右节点, 把旧的右子树替换成新节点的左子树
                            let old_right = unsafe { Box::from_raw(right_raw) };
                            let mut node = new_node(x);
                            node.left = Some(old_right);
                            curr_mut.right = Some(node);

                            return Some(unsafe { Box::from_raw(root_raw) });
                        }
                    }
                    None => {
                        // 查到底了，直接添加一个空节点
                        curr_mut.right = Some(new_node(x));
                        return Some(unsafe { Box::from_raw(root_raw) });
                    }
                }
            }
        }
    }
}

fn new_node<K, W>(x: (K, W)) -> Box<Node<K, W>> {
    Box::new(Node {
        k: x.0,
        w: x.1,
        left: None,
        right: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartesian_tree_build() {
        let mut xs = vec![9, 3, 7, 1, 8, 12, 10, 20, 15, 18, 5]
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i, v))
            .collect::<Vec<_>>();

        let root = build_cartesian_tree(&mut xs).root.unwrap();

        assert_eq!(root.w, 1);

        let (l1, r1) = (root.left.unwrap(), root.right.unwrap());
        assert_eq!(l1.w, 3);
        assert_eq!(r1.w, 5);

        let l2 = r1.left.unwrap();
        assert_eq!(l2.w, 8);

        let l3 = l2.right.unwrap();
        assert_eq!(l3.w, 10);

        let l4 = l3.right.unwrap();
        assert_eq!(l4.w, 15);

        let l5 = l4.right.unwrap();
        assert_eq!(l5.w, 18);
    }
}
