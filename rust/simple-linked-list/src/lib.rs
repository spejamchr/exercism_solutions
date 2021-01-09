use std::iter::FromIterator;

type List<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: List<T>,
}

pub struct SimpleLinkedList<T> {
    head: List<T>,
    len: usize,
}

impl<T> Node<T> {
    fn new(value: T, next: List<T>) -> Self {
        Self { value, next }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, element: T) {
        self.len += 1;
        self.head = Some(Box::new(Node::new(element, self.head.take())));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.len -= 1;
            self.head = n.next;
            n.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.value)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut list = Self::new();
        while let Some(e) = self.pop() {
            list.push(e)
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = Self::new();
        let iter = _iter.into_iter();
        for e in iter {
            list.push(e)
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(e) = self.pop() {
            vec.push(e)
        }
        vec.reverse();
        vec
    }
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
