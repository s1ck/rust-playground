use std::rc::Rc;

pub struct LinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    pub fn append(&self, element: T) -> LinkedList<T> {
        LinkedList {
            head: Some(Rc::new(Node {
                element,
                next: self.head.clone(),
            }))
        }
    }

    pub fn tail(&self) -> LinkedList<T> {
        LinkedList {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
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

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;
    use std::time::Instant;

    // default drop is actually faster
    #[test]
    fn drop_test() {
        let now = Instant::now();
        {
            let mut list = LinkedList::new();
            for i in 0..10_000_000 {
                list.append(i);
            }
        }
        println!("{}", now.elapsed().as_millis());
    }

    #[test]
    fn basics() {
        let list = LinkedList::new();
        assert_eq!(list.head(), None);
        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));
        let list = list.tail();
        assert_eq!(list.head(), Some(&2));
        let list = list.tail();
        assert_eq!(list.head(), Some(&1));
        let list = list.tail();
        assert_eq!(list.head(), None);
        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = LinkedList::new().append(1).append(2).append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}