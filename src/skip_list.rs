//! 跳跃表
//!
//! - [OI Wiki - 跳表](https://oi-wiki.org/ds/skiplist/)
//!
//! 实现细节: 每个 key 只需要一个节点, 有多条指向其他层的链接
//!
//! TODO: 没想清楚最左侧怎么做哨兵节点(最小值)所以很多代码在处理边界情况

use std::fmt::Debug;
use std::ptr::NonNull;

use rand::rngs::ThreadRng;
use rand::Rng;

pub struct SkipListNode<const N: usize, K, V> {
    key: K,
    value: V,
    level: usize, // 节点最高层
    forward: [Link<N, K, V>; N],
}

type Link<const N: usize, K, V> = Option<NonNull<SkipListNode<N, K, V>>>;

pub struct SkipList<const N: usize, K, V> {
    length: usize,             // 元素数量
    level: usize,              // 最高层
    rand: ThreadRng,           // 随机生成器
    lists: [Link<N, K, V>; N], // 每层链表的头节点
}

impl<const N: usize, K, V> SkipListNode<N, K, V> {
    const NONE_NODE: Link<N, K, V> = None;

    pub fn new(key: K, value: V, level: usize) -> Self {
        Self {
            key,
            value,
            level,
            forward: [Self::NONE_NODE; N],
        }
    }
}

impl<const N: usize, K: Ord, V> SkipList<N, K, V> {
    const P: usize = 2;

    pub fn new() -> Self {
        Self {
            length: 0,
            level: 0,
            rand: rand::thread_rng(),
            lists: [SkipListNode::NONE_NODE; N],
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length > 0
    }

    /// 随机层数
    fn rand_lelve(&mut self) -> usize {
        let mut level = 0;
        while self.rand.gen_range(0..Self::P) < 1 {
            level += 1;
            if level > N {
                break;
            }
        }

        level.min(N - 1)
    }

    /// 找到任意一个小于等于 key 的链表头节点作为搜索的起点
    fn find_start_node(&self, key: &K) -> Link<N, K, V> {
        let mut head = None;
        for i in (0..=self.level).rev() {
            if let Some(node) = self.lists[i] {
                let node_key = unsafe { &node.as_ref().key };
                if node_key <= key {
                    head = Some(node);
                    break;
                }
            }
        }

        head
    }

    /// 查找 key 对应的节点值
    pub fn find(&self, key: &K) -> Option<&V> {
        let mut head = match self.find_start_node(key) {
            None => {
                return None;
            }
            Some(node) => {
                if unsafe { &node.as_ref().key } == key {
                    return Some(unsafe { &node.as_ref().value });
                }

                node
            }
        };

        // 从 head 节点开始, 先向右找到每一层小于 key 的最大节点
        // 接着下降到下一层, 继续向右找小于 key 的最大节点
        // 这里 update 记录每一层小于 key 的最大节点用于后续插入
        let max_level = unsafe { head.as_ref().level };
        for i in (0..=max_level).rev() {
            let mut head_ref = unsafe { head.as_ref() };
            while let Some(node) = head_ref.forward[i] {
                let node_ref = unsafe { node.as_ref() };
                if &node_ref.key < key {
                    head = node;
                    head_ref = node_ref;
                } else {
                    break;
                }
            }
        }

        // 当前 head 是第 0 层小于 key 的最大节点
        // 需要确认下个节点的值是否等于 key
        // 如果等于则找到了相同 key 节点直接替换
        let head_ref = unsafe { head.as_ref() };
        if let Some(node) = head_ref.forward[0] {
            let node_key = unsafe { &node.as_ref().key };
            if node_key == key {
                return Some(unsafe { &node.as_ref().value });
            }
        }

        None
    }

    /// 插入指定元素对, 如果 key 对应的节点存在则更新节点 value 把旧的 value 替换出来
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut head = match self.find_start_node(&key) {
            None => {
                // 如果找不到比 key 小的头节点则此 key 是最小值
                // 则执行插入当前最小值的特殊逻辑
                self.insert_min(key, value);
                return None;
            }
            Some(mut node) => {
                let node_key = unsafe { &node.as_ref().key };
                // 如果对应 key 相等则直接替换出来
                if node_key == &key {
                    let old = unsafe { &mut node.as_mut().value };
                    return Some(std::mem::replace(old, value));
                }

                node
            }
        };

        // 从 head 节点开始, 先向右找到每一层小于 key 的最大节点
        // 接着下降到下一层, 继续向右找小于 key 的最大节点
        // 这里 update 记录每一层小于 key 的最大节点用于后续插入
        let max_level = unsafe { head.as_ref().level };
        let mut update = [None; N];
        for i in (0..=max_level).rev() {
            let mut head_ref = unsafe { head.as_ref() };
            while let Some(node) = head_ref.forward[i] {
                let node_ref = unsafe { node.as_ref() };
                if node_ref.key < key {
                    head = node;
                    head_ref = node_ref;
                } else {
                    break;
                }
            }

            update[i] = Some(head);
        }

        // 当前 head 是第 0 层小于 key 的最大节点
        // 需要确认下个节点的值是否等于 key
        // 如果等于则找到了相同 key 节点直接替换
        let head_ref = unsafe { head.as_ref() };
        if let Some(mut node) = head_ref.forward[0] {
            let node_key = unsafe { &node.as_ref().key };
            if node_key == &key {
                let old = unsafe { &mut node.as_mut().value };
                return Some(std::mem::replace(old, value));
            }
        }

        // 创建新节点随机 level 执行 0..level 层的插入
        let new_level = self.rand_lelve();
        let new_node = SkipListNode::new(key, value, new_level);
        let new_node = Box::new(new_node);
        let new_node = unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) };

        for (i, item) in update.iter_mut().enumerate().take(new_level + 1) {
            unsafe {
                match item {
                    None => {
                        (*new_node.as_ptr()).forward[i] = self.lists[i].take();
                        self.lists[i] = Some(new_node);
                    }
                    Some(mut node) => {
                        let node = node.as_mut();
                        let next = node.forward[i].take();
                        (*new_node.as_ptr()).forward[i] = next;
                        node.forward[i] = Some(new_node);
                    }
                }
            }
        }

        self.length += 1;
        self.level = self.level.max(new_level);
        None
    }

    /// 插入最小值节点
    ///
    /// 随机出层数后添加到每层链表的头节点
    fn insert_min(&mut self, key: K, value: V) {
        let new_level = self.rand_lelve();
        let new_node = SkipListNode::new(key, value, new_level);
        let new_node = Box::new(new_node);
        let new_node = unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) };

        for i in 0..=new_level {
            unsafe {
                (*new_node.as_ptr()).forward[i] = self.lists[i].take();
                self.lists[i] = Some(new_node);
            }
        }

        self.length += 1;
        self.level = self.level.max(new_level);
    }

    /// 删除指定 key 的节点
    ///
    /// 需要从上至下找到 key 所在的节点或者前一个节点, 更新每层的链表, 最后 drop 堆内存
    pub fn delete(&mut self, key: &K) -> Option<V> {
        let mut prev = None;
        let mut update = [None; N];
        for i in (0..=self.level).rev() {
            if prev.is_none() {
                prev = self.lists[i];
            }

            if let Some(mut head) = prev {
                let mut head_ref = unsafe { head.as_ref() };

                // 第 i 层的头节点都大于 key 说明 key 所在节点没有在第 i 层无需处理
                if &head_ref.key > key {
                    prev = None;
                    continue;
                }

                // 第 i 层的头节点等于 key 添加到 update 等待后续替换掉头节点
                if &head_ref.key == key {
                    prev = None;
                    update[i] = Some(head);
                    continue;
                }

                // 第 i 层的头节点小于 key 则需要找到当前层小于 key 的最大节点
                if &head_ref.key < key {
                    while let Some(node) = head_ref.forward[i] {
                        let node_ref = unsafe { node.as_ref() };
                        if &node_ref.key < key {
                            head_ref = node_ref;
                            head = node;
                        } else {
                            break;
                        }
                    }

                    prev = Some(head);
                    update[i] = Some(head);
                }
            }
        }

        let mut raw_ptr = None;
        for i in (0..=self.level).rev() {
            match update[i] {
                None => continue,
                Some(mut head) => {
                    let head_ref = unsafe { head.as_ref() };
                    let head_mut = unsafe { head.as_mut() };

                    // 替换当前层的头节点
                    if &head_ref.key == key {
                        let head_next = head_mut.forward[i].take();
                        self.lists[i] = head_next;
                        raw_ptr = Some(head.as_ptr());
                        continue;
                    }

                    if let Some(mut node) = head_ref.forward[i] {
                        let node_key = unsafe { &node.as_ref().key };
                        if node_key == key {
                            let node_next = unsafe { node.as_mut().forward[i] };
                            head_mut.forward[i] = node_next;
                            raw_ptr = Some(node.as_ptr());
                        }
                    }
                }
            }
        }

        // 清理原始指针对应的堆内存
        if let Some(ptr) = raw_ptr {
            self.length -= 1;
            for i in (0..=self.level).rev() {
                if self.lists[i].is_some() {
                    self.level = i;
                    break;
                }
            }

            let node = unsafe { Box::from_raw(ptr) };
            return Some(node.value);
        }

        None
    }
}

impl<const N: usize, K: Ord, V> Default for SkipList<N, K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize, K: Ord + Debug, V: Debug> Debug for SkipList<N, K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..N).rev() {
            write!(f, "{i}: ")?;
            if let Some(head) = self.lists[i] {
                let mut head_ref = unsafe { head.as_ref() };
                write!(f, " {:?}({:?}) ->", head_ref.key, head_ref.value)?;
                while let Some(node) = head_ref.forward[i] {
                    let node_ref = unsafe { node.as_ref() };
                    write!(f, " {:?}({:?}) ->", node_ref.key, node_ref.value)?;
                    head_ref = node_ref;
                }
            }

            writeln!(f, " None")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut sl: SkipList<4, i32, ()> = SkipList::new();
        sl.insert(1, ());
        sl.insert(3, ());
        sl.insert(5, ());
        sl.insert(7, ());
        sl.insert(8, ());
        sl.insert(6, ());
        sl.insert(4, ());
        sl.insert(2, ());

        assert!(sl.insert(1, ()).is_some());
        assert!(sl.insert(8, ()).is_some());
        assert!(sl.insert(10, ()).is_none());

        assert_eq!(sl.len(), 9);

        println!("{:?}", sl);

        assert!(sl.find(&2).is_some());
        assert!(sl.find(&3).is_some());
        assert!(sl.find(&7).is_some());
        assert!(sl.find(&8).is_some());

        assert!(sl.find(&0).is_none());
        assert!(sl.find(&9).is_none());
    }

    #[test]
    fn test_delete() {
        let mut sl: SkipList<4, i32, ()> = SkipList::new();
        sl.insert(1, ());
        sl.insert(3, ());
        sl.insert(5, ());
        sl.insert(7, ());
        sl.insert(8, ());
        sl.insert(6, ());
        sl.insert(4, ());
        sl.insert(2, ());

        assert!(sl.delete(&1).is_some());
        assert!(sl.delete(&2).is_some());
        assert!(sl.delete(&3).is_some());
        assert!(sl.delete(&8).is_some());
        assert!(sl.delete(&7).is_some());

        assert!(sl.delete(&9).is_none());
        assert!(sl.delete(&0).is_none());
        assert!(sl.delete(&2).is_none());
        assert!(sl.delete(&3).is_none());

        assert_eq!(sl.len(), 3);
        assert!(sl.find(&4).is_some());
        assert!(sl.find(&5).is_some());
        assert!(sl.find(&6).is_some());

        println!("{:?}", sl);
    }
}
