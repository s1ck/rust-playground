// Chapter 15
// A Use Case for Interior Mutability: Mock Objects

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;


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
    fn test() {
        let c = MockMessenger::new();
        let mut tracker = LimitTracker::new(&c, 10);

        tracker.set_value(5);
        tracker.set_value(8);
        tracker.set_value(9);
        tracker.set_value(10);

        assert_eq!(c.len(), 3)
    }
}
