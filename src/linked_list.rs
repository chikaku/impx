//! 双向链表 `unsafe` 实现
//!
//! <https://rust-unofficial.github.io/too-many-lists/sixth.html>

use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

/// 节点
pub struct Node<T> {
    elem: T,
    front: Link<T>,
    back: Link<T>,
}

/// 节点指针
type Link<T> = Option<NonNull<Node<T>>>;

/// 双向链表
pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,

    _p: PhantomData<T>,
}

/// 引用迭代器
///
/// 从 `front` 开始向后迭代
pub struct Iter<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,

    _p: PhantomData<&'a T>,
}

/// 可变引用迭代器
pub struct IterMut<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,

    _p: PhantomData<&'a mut T>,
}

/// 迭代器
pub struct IntoIter<T> {
    list: LinkedList<T>,
}

/// 游标
///
/// `CursorMut` 保留一个 `curr` 表示当前节点  
/// 但是每次向前或者向后 `Cursor` 都相当与两个节点中间的一个虚构的位置  
/// 初始时处在于一个 `front` 和 `back` 节点之间的 `ghost` 节点  
/// `ghost` 节点向后 `curr` 是 `front` 向前 `curr` 是 `back`
pub struct CursorMut<'a, T> {
    curr: Link<T>,
    list: &'a mut LinkedList<T>,
    index: Option<usize>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,

            _p: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { self.front.map(|node| &(*node.as_ptr()).elem) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.front.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { self.back.map(|node| &(*node.as_ptr()).elem) }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.back.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));

            if let Some(old) = self.front {
                (*old.as_ptr()).front = Some(new);
                (*new.as_ptr()).back = Some(old);
            } else {
                debug_assert!(self.front.is_none());
                debug_assert!(self.back.is_none());
                debug_assert!(self.len == 0);

                self.back = Some(new);
            }

            self.front = Some(new);
            self.len += 1;
        }
    }

    pub fn push_back(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                elem,
                front: None,
                back: None,
            })));

            if let Some(old) = self.back {
                (*new.as_ptr()).front = Some(old);
                (*old.as_ptr()).back = Some(new);
            } else {
                self.front = Some(new);
            }

            self.back = Some(new);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.front.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());
                let elem = boxed_node.elem;

                self.front = boxed_node.back;
                if let Some(new) = self.front {
                    (*new.as_ptr()).front = None;
                } else {
                    debug_assert!(self.len == 1);
                    self.back = None;
                }

                self.len -= 1;
                elem
            })
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            self.back.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());
                let elem = boxed_node.elem;

                self.back = boxed_node.front;
                if let Some(new) = self.back {
                    (*new.as_ptr()).back = None;
                } else {
                    self.front = None;
                }

                self.len -= 1;
                elem
            })
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            front: self.front,
            back: self.back,
            len: self.len,

            _p: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            front: self.front,
            back: self.back,
            len: self.len,

            _p: PhantomData,
        }
    }

    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut {
            curr: None,
            list: self,
            index: None,
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        for item in self {
            // 这里 item 是从 iter 一直 pop &front 出来的
            // 所以重新添加时要用 push_back
            new_list.push_back(item.clone());
        }

        new_list
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut new_list = Self::new();
        new_list.extend(iter);
        new_list
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().eq(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter().cmp(other)
    }
}

impl<T: Hash> Hash for LinkedList<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.len.hash(state);
        for item in self {
            item.hash(state);
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.len -= 1;
                self.front = (*node.as_ptr()).back;

                &(*node.as_ptr()).elem
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.back.map(|node| unsafe {
                self.len -= 1;
                self.back = (*node.as_ptr()).front;

                &(*node.as_ptr()).elem
            })
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.len -= 1;
                self.front = (*node.as_ptr()).back;

                &mut (*node.as_ptr()).elem
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.back.map(|node| unsafe {
                self.len -= 1;
                self.back = (*node.as_ptr()).front;

                &mut (*node.as_ptr()).elem
            })
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.list.len
    }
}

impl<'a, T> CursorMut<'a, T> {
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    pub fn current(&mut self) -> Option<&mut T> {
        unsafe { self.curr.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn peek_next(&mut self) -> Option<&mut T> {
        unsafe {
            let next = match self.curr {
                Some(curr) => (*curr.as_ptr()).back,
                None => self.list.front,
            };

            next.map(|node| &mut (*node.as_ptr()).elem)
        }
    }

    pub fn peek_prev(&mut self) -> Option<&mut T> {
        unsafe {
            let prev = match self.curr {
                Some(curr) => (*curr.as_ptr()).front,
                None => self.list.back,
            };

            prev.map(|node| &mut (*node.as_ptr()).elem)
        }
    }

    pub fn move_next(&mut self) {
        if let Some(curr) = self.curr {
            unsafe {
                self.curr = (*curr.as_ptr()).back;
                if self.curr.is_some() {
                    *self.index.as_mut().unwrap() += 1;
                } else {
                    // curr 和 index 都是 None
                    // 回到了初始时 front 和 tail 之间的 ghost 节点状态
                    self.index = None;
                }
            }
        } else if !self.list.is_empty() {
            // ghost 节点往后是链表的 back
            self.curr = self.list.front;
            self.index = Some(0);
        } else {
            // 空链表当前处在 ghost 节点什么也不做
        }
    }

    pub fn move_prev(&mut self) {
        if let Some(curr) = self.curr {
            unsafe {
                self.curr = (*curr.as_ptr()).front;
                if self.curr.is_some() {
                    *self.index.as_mut().unwrap() -= 1;
                } else {
                    // curr 和 index 都是 None
                    // 回到了初始时 front 和 tail 之间的 ghost 节点状态
                    self.index = None;
                }
            }
        } else if !self.list.is_empty() {
            // ghost 节点往前是链表的 back
            self.curr = self.list.back;
            self.index = Some(self.list.len() - 1);
        } else {
            // 空链表当前处在 ghost 节点什么也不做
        }
    }

    /// 按照当前位置将原始链表切割成两部分, 并返回前半部分, 当前位置属于后半部分
    pub fn split_before(&mut self) -> LinkedList<T> {
        // list.front -> A <-> B <-> C <-> D <- list.back
        //                           ^
        //                          cur
        //
        // `split_before` 后变成以下两部分
        //
        // list.front -> C <-> D <- list.back
        //               ^
        //              cur
        //
        // return.front -> A <-> B <- return.back
        if let Some(curr) = self.curr {
            unsafe {
                // 当前状态
                let old_len = self.list.len();
                let old_idx = self.index.unwrap();
                let cur_prev = (*curr.as_ptr()).front;

                // curr 属于当前部分所以新的长度可以直接减去 old_idx
                let new_len = old_len - old_idx;
                let new_front = self.curr;
                let new_back = self.list.back;
                let new_idx = Some(0);

                // 左半部分的数据
                let left_len = old_len - new_len;
                let left_back = cur_prev;
                let mut left_front = self.list.front; // 这里原书没有处理 curr 是头节点的情况

                if let Some(prev) = cur_prev {
                    // 如果 curr 的前一个节点不为空, 说明 curr 不是头节点

                    // 当前节点成为了新的头节点
                    (*curr.as_ptr()).front = None;
                    // prev 成为了 left 部分的尾节点
                    (*prev.as_ptr()).back = None;
                } else {
                    // 如果 curr 是头节点, 即 curr 等于 self.list.front
                    // 原本是将 left_front 设置为 self.list.front
                    // 但是此时返回的部分应该是个空链表, 应该将 left_front 置为空
                    left_front = None;
                }

                // 修改内部链表的头尾节点
                self.list.len = new_len;
                self.list.front = new_front;
                self.list.back = new_back;
                // 修改游标表示的当前 index
                self.index = new_idx;

                LinkedList {
                    front: left_front,
                    back: left_back,
                    len: left_len,
                    _p: PhantomData,
                }
            }
        } else {
            // 当前处于ghost 节点, 直接 replace 一个新的旧的整个返回, 反正也都是空的
            mem::replace(self.list, LinkedList::new())
        }
    }

    /// 按照当前位置将原始链表切割成两部分, 并返回后半部分, 当前位置属于前半部分
    pub fn split_after(&mut self) -> LinkedList<T> {
        // list.front -> A <-> B <-> C <-> D <- list.back
        //                     ^
        //                    cur
        //
        // `split_after` 后变成以下两部分
        //
        // list.front -> A <-> B <- list.back
        //                     ^
        //                    cur
        //
        // return.front -> C <-> D <- return.back
        if let Some(curr) = self.curr {
            unsafe {
                let old_len = self.list.len();
                let old_idx = self.index.unwrap();
                let curr_next = (*curr.as_ptr()).back;

                let new_len = old_idx + 1;
                let new_back = self.curr;
                let new_front = self.list.front;
                let new_idx = Some(old_idx);

                let right_len = old_len - new_len;
                let right_front = curr_next;
                let mut right_back = self.list.back; // 这里原书没有处理 curr 是头节点的情况

                if let Some(next) = curr_next {
                    (*curr.as_ptr()).back = None;
                    (*next.as_ptr()).front = None;
                } else {
                    // 原因同 split_before
                    right_back = None;
                }

                self.list.len = new_len;
                self.list.front = new_front;
                self.list.back = new_back;
                self.index = new_idx;

                LinkedList {
                    front: right_front,
                    back: right_back,
                    len: right_len,

                    _p: PhantomData,
                }
            }
        } else {
            // 当前处于ghost 节点, 直接 replace 一个新的旧的整个返回, 反正也都是空的
            mem::replace(self.list, LinkedList::new())
        }
    }

    /// 将输入的链表插入到游标之前的位置
    pub fn splice_before(&mut self, mut input: LinkedList<T>) {
        // We have this:
        //
        // input.front -> 1 <-> 2 <- input.back
        //
        // list.front -> A <-> B <-> C <- list.back
        //                     ^
        //                    cur
        //
        //
        // Becoming this:
        //
        // list.front -> A <-> 1 <-> 2 <-> B <-> C <- list.back
        //                                 ^
        //                                cur
        //
        unsafe {
            if input.is_empty() {
                // 什么也不用做
            } else if let Some(curr) = self.curr {
                // 两个链表都不为空且当前也不在 ghost 节点上
                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();

                if let Some(prev) = (*curr.as_ptr()).front {
                    // 当前不是头节点
                    // 把当前的前半部分和输入的头节点相连
                    (*prev.as_ptr()).back = Some(in_front);
                    (*in_front.as_ptr()).front = Some(prev);

                    // 把当前的后半部分和输入的尾节点相连
                    (*curr.as_ptr()).front = Some(in_back);
                    (*in_back.as_ptr()).back = Some(curr);
                } else {
                    // 当前处于头节点, 直接把前后相连接
                    (*curr.as_ptr()).front = Some(in_back);
                    (*in_back.as_ptr()).back = Some(curr);
                    // 别忘了更新整个链表的头节点
                    self.list.front = Some(in_front);
                }

                // 前面填充了新的元素, 更新下当前索引
                *self.index.as_mut().unwrap() += input.len();
            } else if let Some(back) = self.list.back {
                // 当前处于 ghost 节点但是链表本身不为空
                // 直接把新节点连接到当前节点的后面
                // 为什么是后面? 因为输入链表插入的位置应该在 ghost 节点前面
                // ghost 节点前面就是尾节点, 应该插在尾节点之后

                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();

                (*back.as_ptr()).back = Some(in_front);
                (*in_front.as_ptr()).front = Some(back);

                self.list.back = Some(in_back);
            } else {
                // 当前链表为空, 直接把当前链表置换成输入链表
                mem::swap(self.list, &mut input);
            }

            // 最后不忘忘记更新链表的 size
            self.list.len += input.len();

            // 这里没必要写但是可以写上可以避免后续再误用
            input.len = 0
            // input 在这之后就会被 drop 了
        }
    }

    /// 将输入的链表插入到游标之后的位置
    pub fn splice_after(&mut self, mut input: LinkedList<T>) {
        // We have this:
        //
        // input.front -> 1 <-> 2 <- input.back
        //
        // list.front -> A <-> B <-> C <- list.back
        //                     ^
        //                    cur
        //
        //
        // Becoming this:
        //
        // list.front -> A <-> B <-> 1 <-> 2 <-> C <- list.back
        //                     ^
        //                    cur
        //
        unsafe {
            if input.is_empty() {
                // 输入是空链表不做任何处理
            } else if let Some(curr) = self.curr {
                // 输入链表和当前链表都非空且当前节点不为空

                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();

                if let Some(next) = (*curr.as_ptr()).back {
                    // 当前节点不是尾节点
                    // 将输入链表插入中间收尾相连
                    (*next.as_ptr()).front = Some(in_back);
                    (*in_back.as_ptr()).back = Some(next);
                    (*curr.as_ptr()).back = Some(in_front);
                    (*in_front.as_ptr()).front = Some(curr);
                } else {
                    // 当前节点是尾节点直接添加到最后面
                    (*curr.as_ptr()).back = Some(in_front);
                    (*in_front.as_ptr()).front = Some(curr);
                    self.list.back = Some(in_back);
                }

                // 没有在前面填充元素索引无需更改
            } else if let Some(front) = self.list.front {
                // 当前链表不为空但是当前节点是 ghost 节点
                // 直接把输入链表放到当前链表的前面
                // 为什么是前面, 因为输入链表新加的位置应该在 ghost 节点后面
                // ghost 节点后面就是头节点, 所以要插在头节点之后

                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();

                (*front.as_ptr()).front = Some(in_back);
                (*in_back.as_ptr()).back = Some(front);

                self.list.front = Some(in_front);
            } else {
                // 当前链表为空, 直接把当前链表置换成输入链表
                mem::swap(self.list, &mut input);
            }
        }

        // 最后不忘忘记更新链表的 size
        self.list.len += input.len();

        // 这里没必要写但是可以写上可以避免后续再误用
        input.len = 0
        // input 在这之后就会被 drop 了
    }
}

/// 对于期望产生错误的文档测试可以添加 compile_fail  
/// 后面也可以跟上一个预期编译器产生的错误编号, 只在 nightly 下有效
///
/// ```compile_fail
/// use rlab::linked_list::IterMut;
///
/// fn iter_mut_covariant<'i, 'a, T>(
///     x: IterMut<'i, &'static T>,
/// ) -> IterMut<'i, &'a T> { x }
/// ```
#[allow(dead_code)]
fn iter_mut_invariant() {}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

unsafe impl<'a, T: Send> Send for Iter<'a, T> {}
unsafe impl<'a, T: Send> Sync for Iter<'a, T> {}

unsafe impl<'a, T: Send> Send for IterMut<'a, T> {}
unsafe impl<'a, T: Sync> Sync for IterMut<'a, T> {}

#[allow(dead_code)]
fn assert_properties() {
    fn is_sync<T: Sync>() {}
    fn is_send<T: Send>() {}

    is_send::<LinkedList<i32>>();
    is_sync::<LinkedList<i32>>();

    is_send::<IntoIter<i32>>();
    is_sync::<IntoIter<i32>>();

    is_send::<Iter<i32>>();
    is_sync::<Iter<i32>>();

    is_send::<IterMut<i32>>();
    is_sync::<IterMut<i32>>();

    fn linked_list_covariant<'a, T>(x: LinkedList<&'static T>) -> LinkedList<&'a T> {
        x
    }

    fn iter_covariant<'i, 'a, T>(x: Iter<'i, &'static T>) -> Iter<'i, &'a T> {
        x
    }

    fn into_iter_covariant<'a, T>(x: IntoIter<&'static T>) -> IntoIter<&'a T> {
        x
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    fn generate_test() -> LinkedList<i32> {
        list_from(&[0, 1, 2, 3, 4, 5, 6])
    }

    fn list_from<T: Clone>(v: &[T]) -> LinkedList<T> {
        v.iter().map(|x| (*x).clone()).collect()
    }

    fn check_links<T: Eq + std::fmt::Debug>(list: &LinkedList<T>) {
        let from_front: Vec<_> = list.iter().collect();
        let from_back: Vec<_> = list.iter().rev().collect();
        let reversed: Vec<_> = from_back.into_iter().rev().collect();

        assert_eq!(from_front, reversed);
    }

    #[test]
    fn test_split_front_back() {
        let mut list = generate_test();
        let mut curs = list.cursor_mut();
        curs.move_next();

        let left_part = curs.split_before();
        assert!(left_part.front.is_none());
        assert!(left_part.back.is_none());
    }

    #[test]
    fn test_basic_front() {
        let mut list = LinkedList::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_basic() {
        let mut m = LinkedList::new();
        assert_eq!(m.pop_front(), None);
        assert_eq!(m.pop_back(), None);
        assert_eq!(m.pop_front(), None);
        m.push_front(1);
        assert_eq!(m.pop_front(), Some(1));
        m.push_back(2);
        m.push_back(3);
        assert_eq!(m.len(), 2);
        assert_eq!(m.pop_front(), Some(2));
        assert_eq!(m.pop_front(), Some(3));
        assert_eq!(m.len(), 0);
        assert_eq!(m.pop_front(), None);
        m.push_back(1);
        m.push_back(3);
        m.push_back(5);
        m.push_back(7);
        assert_eq!(m.pop_front(), Some(1));

        let mut n = LinkedList::new();
        n.push_front(2);
        n.push_front(3);
        {
            assert_eq!(n.front().unwrap(), &3);
            let x = n.front_mut().unwrap();
            assert_eq!(*x, 3);
            *x = 0;
        }
        {
            assert_eq!(n.back().unwrap(), &2);
            let y = n.back_mut().unwrap();
            assert_eq!(*y, 2);
            *y = 1;
        }
        assert_eq!(n.pop_front(), Some(0));
        assert_eq!(n.pop_front(), Some(1));
    }

    #[test]
    fn test_iterator() {
        let m = generate_test();
        for (i, elt) in m.iter().enumerate() {
            assert_eq!(i as i32, *elt);
        }
        let mut n = LinkedList::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_iterator_double_end() {
        let mut n = LinkedList::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        n.push_front(5);
        n.push_front(6);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(it.next().unwrap(), &6);
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(it.next_back().unwrap(), &4);
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next_back().unwrap(), &5);
        assert_eq!(it.next_back(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_rev_iter() {
        let m = generate_test();
        for (i, elt) in m.iter().rev().enumerate() {
            assert_eq!(6 - i as i32, *elt);
        }
        let mut n = LinkedList::new();
        assert_eq!(n.iter().rev().next(), None);
        n.push_front(4);
        let mut it = n.iter().rev();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_mut_iter() {
        let mut m = generate_test();
        let mut len = m.len();
        for (i, elt) in m.iter_mut().enumerate() {
            assert_eq!(i as i32, *elt);
            len -= 1;
        }
        assert_eq!(len, 0);
        let mut n = LinkedList::new();
        assert!(n.iter_mut().next().is_none());
        n.push_front(4);
        n.push_back(5);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert!(it.next().is_some());
        assert!(it.next().is_some());
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert!(it.next().is_none());
    }

    #[test]
    fn test_iterator_mut_double_end() {
        let mut n = LinkedList::new();
        assert!(n.iter_mut().next_back().is_none());
        n.push_front(4);
        n.push_front(5);
        n.push_front(6);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(*it.next().unwrap(), 6);
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(*it.next_back().unwrap(), 4);
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(*it.next_back().unwrap(), 5);
        assert!(it.next_back().is_none());
        assert!(it.next().is_none());
    }

    #[test]
    fn test_eq() {
        let mut n: LinkedList<u8> = list_from(&[]);
        let mut m = list_from(&[]);
        assert!(n == m);
        n.push_front(1);
        assert!(n != m);
        m.push_back(1);
        assert!(n == m);

        let n = list_from(&[2, 3, 4]);
        let m = list_from(&[1, 2, 3]);
        assert!(n != m);
    }

    #[test]
    fn test_ord() {
        let n = list_from(&[]);
        let m = list_from(&[1, 2, 3]);
        assert!(n < m);
        assert!(m > n);
        assert!(n <= n);
        assert!(n >= n);
    }

    #[test]
    fn test_ord_nan() {
        let nan = 0.0f64 / 0.0;
        let n = list_from(&[nan]);
        let m = list_from(&[nan]);
        assert!(!(n < m));
        assert!(!(n > m));
        assert!(!(n <= m));
        assert!(!(n >= m));

        let n = list_from(&[nan]);
        let one = list_from(&[1.0f64]);
        assert!(!(n < one));
        assert!(!(n > one));
        assert!(!(n <= one));
        assert!(!(n >= one));

        let u = list_from(&[1.0f64, 2.0, nan]);
        let v = list_from(&[1.0f64, 2.0, 3.0]);
        assert!(!(u < v));
        assert!(!(u > v));
        assert!(!(u <= v));
        assert!(!(u >= v));

        let s = list_from(&[1.0f64, 2.0, 4.0, 2.0]);
        let t = list_from(&[1.0f64, 2.0, 3.0, 2.0]);
        assert!(!(s < t));
        assert!(s > one);
        assert!(!(s <= one));
        assert!(s >= one);
    }

    #[test]
    fn test_debug() {
        let list: LinkedList<i32> = (0..10).collect();
        assert_eq!(format!("{:?}", list), "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]");

        let list: LinkedList<&str> = vec!["just", "one", "test", "more"]
            .iter()
            .copied()
            .collect();
        assert_eq!(format!("{:?}", list), r#"["just", "one", "test", "more"]"#);
    }

    #[test]
    fn test_hashmap() {
        // Check that HashMap works with this as a key

        let list1: LinkedList<i32> = (0..10).collect();
        let list2: LinkedList<i32> = (1..11).collect();
        let mut map = std::collections::HashMap::new();

        assert_eq!(map.insert(list1.clone(), "list1"), None);
        assert_eq!(map.insert(list2.clone(), "list2"), None);

        assert_eq!(map.len(), 2);

        assert_eq!(map.get(&list1), Some(&"list1"));
        assert_eq!(map.get(&list2), Some(&"list2"));

        assert_eq!(map.remove(&list1), Some("list1"));
        assert_eq!(map.remove(&list2), Some("list2"));

        assert!(map.is_empty());
    }

    #[test]
    fn test_cursor_move_peek() {
        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 1));
        assert_eq!(cursor.peek_next(), Some(&mut 2));
        assert_eq!(cursor.peek_prev(), None);
        assert_eq!(cursor.index(), Some(0));
        cursor.move_prev();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), Some(&mut 1));
        assert_eq!(cursor.peek_prev(), Some(&mut 6));
        assert_eq!(cursor.index(), None);
        cursor.move_next();
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 2));
        assert_eq!(cursor.peek_next(), Some(&mut 3));
        assert_eq!(cursor.peek_prev(), Some(&mut 1));
        assert_eq!(cursor.index(), Some(1));

        let mut cursor = m.cursor_mut();
        cursor.move_prev();
        assert_eq!(cursor.current(), Some(&mut 6));
        assert_eq!(cursor.peek_next(), None);
        assert_eq!(cursor.peek_prev(), Some(&mut 5));
        assert_eq!(cursor.index(), Some(5));
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), Some(&mut 1));
        assert_eq!(cursor.peek_prev(), Some(&mut 6));
        assert_eq!(cursor.index(), None);
        cursor.move_prev();
        cursor.move_prev();
        assert_eq!(cursor.current(), Some(&mut 5));
        assert_eq!(cursor.peek_next(), Some(&mut 6));
        assert_eq!(cursor.peek_prev(), Some(&mut 4));
        assert_eq!(cursor.index(), Some(4));
    }

    #[test]
    fn test_cursor_mut_insert() {
        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.splice_before(Some(7).into_iter().collect());
        cursor.splice_after(Some(8).into_iter().collect());
        // check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[7, 1, 8, 2, 3, 4, 5, 6]
        );
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_prev();
        cursor.splice_before(Some(9).into_iter().collect());
        cursor.splice_after(Some(10).into_iter().collect());
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[10, 7, 1, 8, 2, 3, 4, 5, 6, 9]
        );

        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 8, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        let mut p: LinkedList<u32> = LinkedList::new();
        p.extend([100, 101, 102, 103]);
        let mut q: LinkedList<u32> = LinkedList::new();
        q.extend([200, 201, 202, 203]);
        cursor.splice_after(p);
        cursor.splice_before(q);
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[200, 201, 202, 203, 1, 100, 101, 102, 103, 8, 2, 3, 4, 5, 6]
        );
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_prev();
        let tmp = cursor.split_before();
        assert_eq!(m.into_iter().collect::<Vec<_>>(), &[]);
        m = tmp;
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        let tmp = cursor.split_after();
        assert_eq!(
            tmp.into_iter().collect::<Vec<_>>(),
            &[102, 103, 8, 2, 3, 4, 5, 6]
        );
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[200, 201, 202, 203, 1, 100, 101]
        );
    }
}
