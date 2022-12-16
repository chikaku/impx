//! 字典树
//!
//! 一颗多叉树，树的每个节点是一个字母，每个根结点到叶节点的路径上的节点组成一个单词  
//! 根结点到树中间也可能组成一个单词比如 `abc` 在 `abcde` 的字典树上  
//! 所以在节点上需要保存一个标记, 标识是否有从根节点到此节点的路径组成的单词  
//! 字典树一般用于查找字符串是否存在
//!
//! 一般有两种实现方式:
//!
//! - 以完全二叉树的形式将所有节点存储在一个数组中，类似于最大最小堆
//! - 以指针或者 `Option` 的方式将节点存储在堆上

/// 字典树节点
#[derive(Default)]
pub struct Trie {
    child: [Option<Box<Trie>>; 26],
    mark: bool,
}

impl Trie {
    /// 创建一个新节点
    pub fn new() -> Self {
        Self::default()
    }

    /// 插入一个单词
    pub fn insert(&mut self, word: &str) {
        let mut root = self;
        for &c in word.as_bytes() {
            let i = (c - b'a') as usize;
            root = root.child[i].get_or_insert_with(|| Box::new(Self::new()));
        }

        root.mark = true;
    }

    /// 查找一个单词是否存在
    pub fn find(&mut self, word: &str) -> bool {
        let mut root = self;
        for &c in word.as_bytes() {
            let i = (c - b'a') as usize;
            root = match &mut root.child[i] {
                Some(node) => node.as_mut(),
                None => return false,
            };
        }

        root.mark
    }

    /// 删除一个单词，递归删除如果节点的某个子节点没有被标记且所有子节点为空  
    /// 则可以删除这个子节点
    pub fn delete(&mut self, word: &str) {
        match word.as_bytes().first() {
            None => {
                self.mark = false;
            }
            Some(&ch) => {
                let i = (ch - b'a') as usize;
                if let Some(node) = &mut self.child[i] {
                    node.as_mut().delete(&word[1..]);
                    if !node.mark && node.child.iter().all(|slot| slot.is_none()) {
                        self.child[i] = None;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trie() {
        use super::*;

        let mut t = Trie::new();
        t.insert("abcd");
        t.insert("ab");
        t.insert("bcd");
        t.insert("bcdf");
        t.insert("adfg");

        assert!(t.find("ab"));
        assert!(t.find("abcd"));
        assert!(t.find("adfg"));
        assert!(t.find("bcd"));
        assert!(!t.find("abc"));
        assert!(!t.find("ad"));

        t.delete("a");
        assert!(t.find("ab"));
        t.delete("abc");
        assert!(t.find("ab"));
        t.delete("ab");
        assert!(!t.find("a"));
        assert!(!t.find("ab"));
        assert!(!t.find("abc"));
        assert!(t.find("abcd"));
    }
}
