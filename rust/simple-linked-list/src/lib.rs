use std::iter::FromIterator;

type OBNode<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: OBNode<T>,
}

pub struct SimpleLinkedList<T> {
    head: OBNode<T>,
    size: usize,
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, data: T) {
        self.size += 1;
        self.head = Some(Box::new(Node {
            data,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                self.size -= 1;
                Some(node.data)
            }
            _ => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.data),
            _ => None,
        }
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();

        while let Some(node) = self.pop() {
            reversed.push(node);
        }

        reversed
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = Self::new();

        _iter.into_iter().for_each(|d| list.push(d));
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

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut current = linked_list.head;
        let mut vec = vec![];

        while let Some(node) = current {
            vec.insert(0, node.data);
            current = node.next;
        }
        vec
    }
}
