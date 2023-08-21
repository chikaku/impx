//! 最小生成树
//!
//! 参考：[OI wiki - 最小生成树#Kruskal 算法](https://oi-wiki.org/graph/mst/#kruskal-%E7%AE%97%E6%B3%95)
//!
//! 首先定义什么是最小生成树：给定一个无向连通图，构建一个连通所有节点的树，其边权之和最小的，即为这个图的最小生成树
//!
//! 举例：假设一个图 `[(A, B, 1), (B, C, 2), (C, A, 3)]` 三元组表示一个边的两节点及其边权值  
//! 则这个图的最小生成树就是以 B 为根节点连接 A 和 C 的树，所有边权之和最小为 3  
//! 当然以 A 为根节点连接 B 再连接 C 也是一棵最小生成树，所有边权之和最小为 3  
//!
//! Kruskal 算法是构建最小生成树的常用算法，原理是：
//!
//! - 首先将所有的边按照权值排序，准备一个并查集，表示所有已经加入树的节点的集合
//! - 遍历取权值最小的边，判断这个边的两节点是否在并查集中相连
//! - 如果相连，表示这两个节点都已经加入到同一颗树中，不做处理
//! - 如果不相连，表示这两个节点目前还处于不同的树中，将两节点使用并查集合并即可
//! - 遍历结束后，能够保证最终的权值是最小的(详细的归纳法证明见 oi-wiki 链接)

type Edge = (usize, usize, usize);

/// Kruskal 算法构建最小生成树
pub fn kruskal(mut g: Vec<Edge>, m: usize /* 表示节点数量 */) -> Vec<Edge> {
    // 按边权逆序 sort 方面后面从尾部 pop 最小值
    g.sort_by(|a, b| b.2.cmp(&a.2));

    let mut uf = UnionFind::new(m);

    let mut res = vec![];
    while let Some(edge) = g.pop() {
        let (a, b, _) = edge;
        if !uf.connected(a, b) {
            res.push(edge);
            uf.union(a, b);
        }
    }

    res
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        Self { parent }
    }

    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a != root_b {
            self.parent[root_a] = root_b;
        }
    }

    fn find(&self, mut a: usize) -> usize {
        let mut root = self.parent[a];
        while a != root {
            a = root;
            root = self.parent[a];
        }

        root
    }

    fn connected(&self, a: usize, b: usize) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);
        root_a == root_b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kruskal() {
        let g = vec![(0, 1, 1), (1, 2, 1), (2, 0, 3)];
        let t = kruskal(g, 3);
        let s = t.iter().fold(0, |acc, x| acc + x.2);
        assert_eq!(s, 2);

        let g = vec![
            (0, 1, 7),  // A -> B 7
            (0, 3, 5),  // A -> D 5
            (1, 2, 8),  // B -> C 8
            (1, 3, 9),  // B -> D 9
            (1, 4, 7),  // B -> E 7
            (2, 4, 5),  // C -> E 5
            (3, 4, 15), // D -> E 15
            (3, 5, 6),  // D -> F 6
            (4, 5, 8),  // E -> F 8
            (4, 6, 9),  // E -> G 9
            (5, 6, 11), // F -> G 11
        ];
        let t = kruskal(g, 7);
        let s = t.iter().fold(0, |acc, x| acc + x.2);
        assert_eq!(s, 39);
    }
}
