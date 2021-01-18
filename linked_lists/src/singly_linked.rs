/// Following along the tutorial by Ryan Levick (https://www.youtube.com/watch?v=IiDHTIsmUi4)
/// Which in turn follows https://rust-unofficial.github.io/too-many-lists/first.html

pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, element: T) {
        // Use mem::replace
        // let old_head = std::mem::replace(&mut self.head, None);
        // let node = Node::new(element, old_head);
        // self.head = Some(Box::new(node));

        // Or use Option::take
        let node = Node::new(element, self.head.take());
        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
    }
}

// nice .. implement functions only for lists that contain u32
impl LinkedList<u32> {
    fn size(&self) -> usize {
        let mut size = 0;
        let mut link = &self.head;
        while let Some(node) = link {
            size += 1;
            link = &node.next;
        }
        size
    }
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(element: T, next: Link<T>) -> Self {
        Self { element, next }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut old_head = self.head.take();
        while let Some(mut next_node) = old_head {
            old_head = next_node.next.take()
        }
    }
}

// IntoIter
pub struct IntoIter<T>(LinkedList<T>);

impl<T> LinkedList<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// Iter
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.element
        })
    }
}

// IterMut
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.element
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn drop_test() {
        let now = Instant::now();
        {
            let mut list = LinkedList::new();
            for i in 0..10_000_000 {
                list.push(i);
            }
        }
        println!("{}", now.elapsed().as_millis());
    }

    #[test]
    fn node_test() {
        let mut l = LinkedList::new();
        assert_eq!(0, l.size());

        l.push(42);
        l.push(84);

        assert_eq!(2, l.size());
        assert_eq!(Some(&84), l.peek());
        assert_eq!(Some(84), l.pop());
        assert_eq!(Some(&42), l.peek());
        assert_eq!(Some(&mut 42), l.peek_mut());
        l.peek_mut().map(|value| *value = 43);
        assert_eq!(Some(43), l.pop());
        assert_eq!(None, l.peek());
        assert_eq!(None, l.pop());
    }

    #[test]
    fn into_iter() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
