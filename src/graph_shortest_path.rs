//! 图最短路径

use std::collections::BinaryHeap;

/// Floyd 算法
pub fn floyd(
    n: usize,                        // 节点个数 编号为 1..N
    edges: &[(usize, usize, usize)], // (u, v, w) u->v 权重为 w
    src: usize,                      // 源节点
    dst: usize,                      // 目标节点
) -> Option<usize> {
    // f[k][x][y] 表示在子图 1..k 内从 x 到 y 的最短路径
    let mut f = vec![vec![vec![None; n + 1]; n + 1]; n + 1];

    // 如果两节点有直接连接则设置对应路径权重
    for &(u, v, w) in edges {
        f[0][u][v] = Some(w);
    }

    // 每个节点和自己的连接路径权重为 0
    for u in 1..=n {
        f[0][u][u] = Some(0);
    }

    for k in 1..=n {
        for x in 1..=n {
            for y in 1..=n {
                let path_thk = f[k - 1][x][k].and_then(|w1| f[k - 1][k][y].map(|w2| w1 + w2));
                f[k][x][y] = min_option_usize(f[k - 1][x][y], path_thk);
            }
        }
    }

    f[n][src][dst]
}

#[derive(PartialEq, Eq, Debug)]
struct NodeDistance(usize, usize);

impl Ord for NodeDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for NodeDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Dijkstra 算法
pub fn dijkstra(
    n: usize,                        // 节点个数 编号为 1..N
    edges: &[(usize, usize, usize)], // (u, v, w) u->v 权重为 w
    src: usize,                      // 源节点
    dst: usize,                      // 目标节点
) -> Option<usize> {
    let mut g = vec![vec![]; n + 1];
    for &edge in edges {
        let (u, v, w) = edge;
        g[u].push((v, w));
    }

    // distance 记录 src 到各个节点的最短路径
    let mut distance = vec![None; n + 1];

    // 保存目前已知的路径
    let mut h = BinaryHeap::new();
    h.push(NodeDistance(src, 0));

    // 在当前已知路径中取最短的那一条 src -> u
    // 则这一条路径必定是最短路径
    while let Some(NodeDistance(u, dis)) = h.pop() {
        if distance[u].is_none() {
            distance[u] = Some(dis);
            for &(v, w) in &g[u] {
                // 把当前节点连通的节点加入堆
                if distance[v].is_none() {
                    h.push(NodeDistance(v, dis + w));
                }
            }
        }
    }

    distance[dst]
}

fn min_option_usize<T: std::cmp::Ord>(a: Option<T>, b: Option<T>) -> Option<T> {
    match (a, b) {
        (None, None) => None,
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (Some(a), Some(b)) => Some(a.min(b)),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn node_distance_heap() {
        let mut h = BinaryHeap::new();
        h.push(NodeDistance(2, 10));
        h.push(NodeDistance(6, 3));
        h.push(NodeDistance(4, 9));
        h.push(NodeDistance(2, 5));

        assert_eq!(h.pop(), Some(NodeDistance(6, 3)));
        assert_eq!(h.pop(), Some(NodeDistance(2, 5)));
        assert_eq!(h.pop(), Some(NodeDistance(4, 9)));
        assert_eq!(h.pop(), Some(NodeDistance(2, 10)));
    }

    fn assert_shortest_path<F>(f: F)
    where
        F: Fn(usize, &[(usize, usize, usize)], usize, usize) -> Option<usize>,
    {
        let g1 = [(1, 2, 2), (2, 3, 2), (3, 4, 1), (1, 3, 1)];
        assert_eq!(f(4, &g1, 1, 4), Some(2));

        let g2 = [
            (1, 2, 10),
            (1, 6, 3),
            (2, 3, 7),
            (2, 4, 5),
            (4, 1, 3),
            (4, 3, 4),
            (4, 5, 7),
            (6, 2, 2),
            (6, 4, 6),
            (6, 5, 1),
        ];

        assert_eq!(f(6, &g2, 1, 2), Some(5));
        assert_eq!(f(6, &g2, 1, 3), Some(12));
        assert_eq!(f(6, &g2, 1, 4), Some(9));
        assert_eq!(f(6, &g2, 1, 5), Some(4));
        assert_eq!(f(6, &g2, 1, 6), Some(3));
    }

    #[test]
    fn test_floyd() {
        assert_shortest_path(floyd);
    }

    #[test]
    fn test_dijkstra() {
        assert_shortest_path(dijkstra);
    }
}
