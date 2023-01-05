//! 基数树
//!
//! 参考 [armon/go-radix](https://github.com/armon/go-radix) 实现的 Rust 版本的 Radix-Tree

/// 基数树节点之间相连的边
pub struct Edge<T> {
    label: char,
    node: Node<T>,
}

/// 基数树节点
pub struct Node<T> {
    value: Option<(String, T)>,
    prefix: String,
    edges: Vec<Edge<T>>,
}

impl<T> Node<T> {
    pub fn new() -> Self {
        Self {
            value: None,
            prefix: String::new(),
            edges: vec![],
        }
    }

    pub fn new_prefix(s: &str) -> Self {
        Self {
            value: None,
            prefix: String::from(s),
            edges: vec![],
        }
    }

    /// 二分查找以 target 作为首字符的子节点
    pub fn find(&self, target: &char) -> Option<&Node<T>> {
        self.edges
            .binary_search_by(|edge| edge.label.cmp(target))
            .ok()
            .map(|idx| &self.edges[idx].node)
    }

    /// 二分查找以 target 作为首字符的子节点所在边索引
    pub fn find_index(&self, target: &char) -> Result<usize, usize> {
        self.edges.binary_search_by(|edge| edge.label.cmp(target))
    }

    /// 返回以当前节点作为数据节点的值
    pub fn value(&self) -> Option<(&str, &T)> {
        self.value.as_ref().map(|(k, v)| (k.as_str(), v))
    }

    /// 节点新增一条边
    pub fn add_edge(&mut self, edge: Edge<T>) {
        match self.find_index(&edge.label) {
            Ok(_) => unreachable!("repeat label in edges"),
            Err(index) => self.edges.insert(index, edge),
        }
    }

    /// 合并子节点
    fn merge_child(&mut self) {
        if self.edges.len() == 1 {
            let child = self.edges.remove(0).node;
            self.prefix.push_str(&child.prefix);
            self.edges = child.edges;
            self.value = child.value;
        }
    }

    /// 删除所有子节点和自己
    fn delete(&mut self) -> usize {
        let mut count = 0;
        if self.value.take().is_some() {
            count += 1;
        }

        for edge in &mut self.edges {
            count += edge.node.delete();
        }

        count
    }
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 基数树
pub struct RadixTree<T> {
    root: Node<T>,
    size: usize,
}

impl<T> RadixTree<T> {
    pub fn new() -> Self {
        Self {
            root: Node::new(),
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// 查找 key 对应基数树中的 value
    pub fn find(&self, key: &str) -> Option<&T> {
        let mut node = &self.root;
        let mut search = key;

        while let Some(label) = search.chars().peekable().peek() {
            match node.find(label) {
                None => break,
                Some(child) => {
                    // 如果前缀与当前节点不匹配则结束搜索
                    if !search.starts_with(&child.prefix) {
                        return None;
                    }

                    // 去掉在当前节点上的前缀继续搜索
                    search = &search[child.prefix.len()..];
                    node = child;
                }
            }
        }

        node.value().map(|(_, v)| v)
    }

    /// 在树中插入 key-value 对如果树中已经存在对应的 key 则更新其值并将旧值返回出来
    pub fn insert(&mut self, key: &str, value: T) -> Option<T> {
        let mut node = &mut self.root;
        let mut search = key;

        loop {
            match search.chars().peekable().peek() {
                None => match &mut node.value {
                    None => {
                        // 如果当前找到的节点不是数据节点, 则直接写入数据
                        node.value = Some((String::from(key), value));
                        self.size += 1;
                        return None;
                    }
                    Some(v) => {
                        // 如果当前找到的节点是数据节点, 则将其值交换出来
                        return Some(std::mem::replace(&mut v.1, value));
                    }
                },
                Some(&label) => match node.find_index(&label) {
                    Err(_) => {
                        node.add_edge(Edge {
                            label,
                            node: Node {
                                value: Some((String::from(key), value)),
                                prefix: String::from(search),
                                edges: vec![],
                            },
                        });

                        self.size += 1;
                        return None;
                    }
                    Ok(index) => {
                        // 如果能找到对应字符的边首先查看此节点是不是 search 的前缀
                        // 如果是则在 search 上去掉前缀继续向下搜索即可
                        if search.starts_with(&node.edges[index].node.prefix) {
                            node = &mut node.edges[index].node;
                            search = &search[node.prefix.len()..];
                            continue;
                        }

                        // 如果不是前缀则需要分裂当前节点
                        // 先把当前子节点移出父节点
                        let mut child = node.edges.remove(index).node;

                        // 计算出 search 和当前节点的最长公共前缀
                        // 以这个公共前缀建立一个新的节点
                        let size = longest_commin_prefix(search, &child.prefix);
                        let mut new_parent = Node::new_prefix(&search[..size]);

                        // 子节点移除公共前缀, 添加到新节点的边上
                        child.prefix.drain(..size);
                        new_parent.add_edge(Edge {
                            label: first_char(&child.prefix),
                            node: child,
                        });

                        // 用 search 添加新的数据节点
                        // 也要先移除公共前缀
                        search = &search[size..];
                        if search.is_empty() {
                            // 如果 search 已经空了则直接把数据写到新的父节点
                            new_parent.value = Some((String::from(key), value));
                        } else {
                            // 否则将 search 写入到父节点的一个新的子节点
                            new_parent.add_edge(Edge {
                                label: first_char(search),
                                node: Node {
                                    value: Some((String::from(key), value)),
                                    prefix: String::from(search),
                                    edges: vec![],
                                },
                            });
                        }

                        // 把分裂出的新节点添加到原先父节点 node 的边上
                        // 父节点已经事先把旧节点移除了所以这里无需再关心
                        node.add_edge(Edge {
                            label,
                            node: new_parent,
                        });

                        self.size += 1;
                        return None;
                    }
                },
            }
        }
    }

    /// 在基数树中删除指定的 key
    ///
    /// 删除前要先进行查找, 如果找到了数据节点将其置空即可  
    /// 另外如果数据节点没有子节点则此节点可以在父节点上删除  
    /// 不像字典树, 父节点不可能只有这一个节点所以不需要递归删除父节点  
    ///
    /// 除此之外如果数据节点数据置空后只剩下一条边则可以考虑合并子节点
    ///
    /// 同时如果父节点删除掉数据节点之后只剩下一条边而且父节点不是数据节点  
    /// 则可以考虑将父节点和其子节点合并  
    ///
    /// 如基数树中有 `abc` 和 `acd` 两个数据则插入完成后应该是 root 节点有一个节点 a  
    /// 同时 a 有两个子节点 `bc` 和 `cd` 当 `abc` 被删除时 `a` 可以和 `cd` 合并成 `acd`
    pub fn delete(&mut self, key: &str) -> Option<(String, T)> {
        let mut node = &mut self.root;
        let mut search = key;
        let mut is_root = true;

        while let Some(label) = search.chars().peekable().peek() {
            match node.find_index(label) {
                // 没找到节点
                Err(_) => return None,
                Ok(index) => {
                    let child = &node.edges[index].node;
                    if !search.starts_with(&child.prefix) {
                        return None;
                    }

                    search = &search[child.prefix.len()..];
                    if !search.is_empty() {
                        node = &mut node.edges[index].node;
                        is_root = false;
                        continue;
                    }

                    // 取出节点值
                    let child = &mut node.edges[index].node;
                    let value = child.value.take();

                    // 如果节点的边数为 1 则合并其子节点
                    // 此时节点的数据可能重新被赋值
                    if child.edges.len() == 1 {
                        child.merge_child();
                    }

                    // 如果节点为空且非数据节点则可以删除
                    if child.edges.is_empty() && child.value.is_none() {
                        node.edges.remove(index);
                    }

                    // 如果父节点不是根节点且边数为 1 则也可以合并子节点
                    if !is_root && node.edges.len() == 1 && node.value.is_none() {
                        node.merge_child();
                    }

                    if value.is_some() {
                        self.size -= 1;
                    }

                    return value;
                }
            }
        }

        None
    }

    /// 删除指定前缀
    ///
    ///
    pub fn delete_prefix(&mut self, pre: &str) -> usize {
        let mut parent = &mut self.root;
        let mut child_index = None;
        let mut search = pre;
        let mut is_root = true;

        // 找到前缀节点对应的父节点和索引前缀节点在父节点中的索引 child_index
        while let Some(label) = search.chars().peekable().peek() {
            if let Some(index) = child_index {
                let edge: &mut Edge<T> = &mut parent.edges[index];
                parent = &mut edge.node;
                is_root = false;
            }

            match parent.find_index(label) {
                Err(_) => return 0,
                Ok(index) => {
                    let child = &parent.edges[index].node;

                    // 如果 search 和 child.prefix 都不是互相的前缀则不存在 pre 前缀的节点搜索结束
                    if !search.starts_with(&child.prefix) && !child.prefix.starts_with(search) {
                        return 0;
                    }

                    let size = std::cmp::max(child.prefix.len(), search.len());
                    search = &search[size..];
                    child_index = Some(index);
                }
            }
        }

        match child_index {
            None => {
                // 直接从根节点清理所有数据
                let size = self.size;
                self.root.edges.clear();
                self.size = 0;
                size
            }
            Some(index) => {
                let child = &mut parent.edges[index].node;
                let deleted = child.delete();
                parent.edges.remove(index);

                if !is_root && parent.edges.len() == 1 && parent.value.is_none() {
                    parent.merge_child();
                }

                deleted
            }
        }
    }

    /// 转换成引用迭代器
    pub fn iter(&self) -> Iter<'_, T> {
        let mut indexes = vec![];
        let mut node = &self.root;
        while !node.edges.is_empty() && node.value.is_none() {
            indexes.push(0);
            node = &node.edges[0].node;
        }

        let root = &self.root;
        Iter { root, indexes }
    }
}

impl<T> Default for RadixTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 引用迭代器
pub struct Iter<'a, T> {
    root: &'a Node<T>,
    indexes: Vec<usize>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (&'a str, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.indexes.is_empty() {
            return None;
        }

        let indexes = &self.indexes;
        let mut parent = self.root;
        let mut node = self.root;
        let mut curr_idx = 0;
        for &v in indexes {
            parent = node;
            node = &parent.edges[v].node;
            curr_idx = v;
        }

        let res = node.value();

        // 向下寻找子节点
        if !node.edges.is_empty() {
            self.indexes.push(0);
            node = &node.edges[0].node;
            while node.value.is_none() {
                self.indexes.push(0);
                node = &node.edges[0].node;
            }

            return res;
        }

        self.indexes.pop();

        // 寻找兄弟节点
        if let Some(edge) = parent.edges.get(curr_idx + 1) {
            self.indexes.push(curr_idx + 1);
            node = &edge.node;
            while node.value.is_none() {
                self.indexes.push(0);
                node = &node.edges[0].node;
            }

            return res;
        }

        // 回退父节点 p1
        // 先找到当前父节点在兄弟中的索引如果父节点是根节点就直接结束了
        while let Some(index) = self.indexes.pop() {
            // 因为要找父节点的兄弟节点所以要记录父节点的父节点 pp1
            // 上层循环的时候已经把 p1 的索引弹出去了所以直接查找
            let mut pparent = self.root;
            for &v in &self.indexes {
                pparent = &pparent.edges[v].node;
            }

            // 寻找兄弟节点 p2
            if let Some(edge) = pparent.edges.get(index + 1) {
                self.indexes.push(curr_idx + 1);
                node = &edge.node;
                while node.value.is_none() {
                    self.indexes.push(0);
                    node = &node.edges[0].node;
                }

                return res;
            }

            // 找不到的时候要继续回退上一级的父节点
        }

        res
    }
}

/// 迭代器
pub struct IntoIter<T> {
    tree: RadixTree<T>,
    indexes: Vec<usize>,
}

impl<T> IntoIterator for RadixTree<T> {
    type Item = (String, T);

    type IntoIter = IntoIter<T>;

    /// 转换成值迭代器
    fn into_iter(self) -> Self::IntoIter {
        let mut indexes = vec![];
        let mut node = &self.root;
        while !node.edges.is_empty() && node.value.is_none() {
            indexes.push(0);
            node = &node.edges[0].node;
        }

        let tree = self;
        IntoIter { tree, indexes }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = (String, T);

    fn next(&mut self) -> Option<Self::Item> {
        let indexes = &self.indexes;
        let mut node = &self.tree.root;
        for &v in indexes {
            node = &node.edges[v].node;
        }

        match &node.value {
            None => None,
            Some((key, _)) => {
                // 这里为了避免处理不可变引用 key 和可变引用 self.tree 的冲突直接复制了 key 进行删除
                // 正确的流程应该是和 delete 类似先删除子节点然后处理合并父节点
                let key = String::from(key);
                let res = self.tree.delete(key.as_str());
                self.indexes.clear();

                let mut node = &self.tree.root;
                while !node.edges.is_empty() && node.value.is_none() {
                    self.indexes.push(0);
                    node = &node.edges[0].node;
                }

                res
            }
        }
    }
}

/// 求两字符串的最长前缀长度
pub fn longest_commin_prefix(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .take_while(|x| x.0 == x.1)
        .map(|x| x.0.len_utf8())
        .sum()
}

/// 获取字符串的首个字符
fn first_char(s: &str) -> char {
    s.chars().next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(2, longest_commin_prefix("abc", "abd"));
        assert_eq!(0, longest_commin_prefix("abc", "bcd"));

        let (a, b) = ("你们好", "你们");
        let size = longest_commin_prefix(a, b);
        assert_eq!(&a[size..], "好");
        assert_eq!(&b[size..], "");

        let (a, b) = ("好good好", "好golden");
        let size = longest_commin_prefix(a, b);
        assert_eq!(&a[size..], "od好");
        assert_eq!(&b[size..], "lden");
    }

    #[test]
    fn test_radix_tree() {
        let mut t = RadixTree::new();

        t.insert("a", 1);
        t.insert("ab", 2);
        t.insert("abc", 3);
        t.insert("abcd", 4);
        t.insert("abcde", 5);
        t.insert("abcdef", 6);

        assert_eq!(t.len(), 6);

        assert_eq!(t.find("a"), Some(&1));
        assert_eq!(t.find("abc"), Some(&3));
        assert_eq!(t.find("abcdef"), Some(&6));

        assert_eq!(t.delete("abc"), Some(("abc".into(), 3)));
        assert_eq!(t.delete("abcdef"), Some(("abcdef".into(), 6)));

        assert!(t.delete("abc").is_none());
        assert!(t.delete("abcdef").is_none());

        assert_eq!(t.len(), 4);
        assert_eq!(t.delete_prefix("a"), 4);

        let mut t = RadixTree::new();
        t.insert("aaa", 1);
        t.insert("aab", 2);
        t.insert("abb", 3);

        assert_eq!(t.len(), 3);

        assert!(t.delete("aa").is_none());
        assert!(t.delete("ab").is_none());

        assert_eq!(t.delete("aaa"), Some(("aaa".into(), 1)));
        assert_eq!(t.delete("abb"), Some(("abb".into(), 3)));
        assert_eq!(t.delete("aab"), Some(("aab".into(), 2)));

        let mut t = RadixTree::new();
        t.insert("/aaa", 1);
        t.insert("/bbb", 1);

        assert_eq!(t.delete_prefix(""), 2);
        assert_eq!(t.delete_prefix("/"), 0);

        t.insert("abc", 1);
        t.insert("abcd", 1);
        t.insert("abce", 1);
        t.insert("abcf", 1);

        assert_eq!(t.delete_prefix("abc"), 4);
    }

    #[test]
    fn test_radix_tree_iter() {
        let mut t = RadixTree::new();

        t.insert("a", 1);
        t.insert("ab", 2);
        t.insert("abc", 3);
        t.insert("ac", 4);
        t.insert("acd", 5);
        t.insert("b", 6);

        let mut it = t.iter();
        assert_eq!(it.next(), Some(("a", &1)));
        assert_eq!(it.next(), Some(("ab", &2)));
        assert_eq!(it.next(), Some(("abc", &3)));
        assert_eq!(it.next(), Some(("ac", &4)));
        assert_eq!(it.next(), Some(("acd", &5)));
        assert_eq!(it.next(), Some(("b", &6)));
        assert_eq!(it.next(), None);

        t.delete_prefix("");
        assert!(t.iter().next().is_none());
    }

    #[test]
    fn test_radix_tree_into_iter() {
        let mut t = RadixTree::new();

        t.insert("a", 1);
        t.insert("ab", 2);
        t.insert("abc", 3);
        t.insert("ac", 4);
        t.insert("acd", 5);
        t.insert("b", 6);

        let mut it = t.into_iter();
        assert_eq!(it.next(), Some(("a".into(), 1)));
        assert_eq!(it.next(), Some(("ab".into(), 2)));
        assert_eq!(it.next(), Some(("abc".into(), 3)));
        assert_eq!(it.next(), Some(("ac".into(), 4)));
        assert_eq!(it.next(), Some(("acd".into(), 5)));
        assert_eq!(it.next(), Some(("b".into(), 6)));
        assert_eq!(it.next(), None);
    }
}
