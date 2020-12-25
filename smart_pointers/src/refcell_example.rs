// Chapter 15
// A Use Case for Interior Mutability: Mock Objects

use std::cell::RefCell;
use std::rc::Rc;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of you quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use crate::refcell_example::List::{Cons, Nil};


    struct MockMessenger {
        messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> Self {
            MockMessenger { messages: RefCell::new(vec![]) }
        }

        fn len(&self) -> usize {
            self.messages.borrow().len()
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.messages.borrow_mut().push(String::from(msg))
        }
    }

    #[test]
    fn mock_messenger() {
        let c = MockMessenger::new();
        let mut tracker = LimitTracker::new(&c, 10);

        tracker.set_value(5);
        tracker.set_value(8);
        tracker.set_value(9);
        tracker.set_value(10);

        assert_eq!(c.len(), 3)
    }

    #[test]
    fn mutable_list() {
        let v = Rc::new(RefCell::new(42));

        let a = Rc::new(Cons(Rc::clone(&v), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(2)), Rc::clone(&a));

        println!("a before = {:?}", a);
        println!("b before = {:?}", b);
        println!("c before = {:?}", c);

        *v.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);

    }
}
