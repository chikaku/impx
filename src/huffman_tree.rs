//! 霍夫曼树

use crate::binary_tree::BinaryTree;

type HuffmanTree<T> = BinaryTree<Option<T>>;

/// 构建霍夫曼树
pub fn build_huffman_tree<T>(ws: &[(T, usize)]) -> HuffmanTree<T>
where
    T: Copy,
{
    let mut nodes = ws
        .iter()
        .map(|x| (BinaryTree::new(Some(x.0)), x.1))
        .collect::<Vec<_>>();

    // 按权值倒序
    nodes.sort_by(|a, b| b.1.cmp(&a.1));

    while let Some((node0, weight0)) = nodes.pop() {
        match nodes.pop() {
            None => return node0,
            Some((node1, weight1)) => {
                // 取权值最小的两个节点组成一棵新的树插入到序列中
                let weight = weight0 + weight1;
                let mut parent = BinaryTree::new(None);
                parent.left = Some(Box::new(node0));
                parent.right = Some(Box::new(node1));

                let idx = nodes
                    .binary_search_by(|(_, probe)| weight.cmp(probe))
                    .unwrap_or_else(|e| e);

                nodes.insert(idx, (parent, weight));
                continue;
            }
        };
    }

    // when ws is empty
    BinaryTree::new(None)
}

/// 构建霍夫曼编码
pub fn build_huffman_coding<T>(ws: &[(T, usize)]) -> Vec<(T, usize)>
where
    T: Copy,
{
    let mut res = vec![];
    let root = build_huffman_tree(ws);
    tree_dfs(&root, 0, &mut res);

    res
}

fn tree_dfs<T>(root: &BinaryTree<Option<T>>, v: usize, res: &mut Vec<(T, usize)>)
where
    T: Copy,
{
    if let Some(node) = &root.left {
        tree_dfs(node.as_ref(), v << 1, res);
    }

    if let Some(node) = &root.right {
        tree_dfs(node.as_ref(), (v << 1) | 1, res);
    }

    if let Some(value) = root.value {
        res.push((value, v));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huffman_tree() {
        let words = [('A', 35), ('B', 25), ('C', 15), ('D', 15), ('E', 10)];
        let tree = build_huffman_tree(&words);

        let left = tree.left.unwrap();
        let right = tree.right.unwrap();

        assert_eq!(left.left.unwrap().value, Some('C'));
        assert_eq!(left.right.unwrap().value, Some('B'));
        assert_eq!(right.right.unwrap().value, Some('A'));

        let left = right.left.unwrap();
        assert_eq!(left.left.unwrap().value, Some('E'));
        assert_eq!(left.right.unwrap().value, Some('D'));
    }

    #[test]
    fn test_huffman_coding() {
        let words = [('A', 35), ('B', 25), ('C', 15), ('D', 15), ('E', 10)];
        let coding = build_huffman_coding(&words);
        assert_eq!(coding, [('C', 0), ('B', 1), ('E', 4), ('D', 5), ('A', 3)]);
    }
}
