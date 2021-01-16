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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(Some(42), l.pop());
        assert_eq!(None, l.peek());
        assert_eq!(None, l.pop());
    }
}
