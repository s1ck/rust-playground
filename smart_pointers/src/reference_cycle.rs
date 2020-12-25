use std::cell::RefCell;
use std::rc::Rc;

use crate::reference_cycle::List::Cons;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::reference_cycle::List::{Cons, Nil};

    use super::*;

    #[test]
    fn memory_leak() {
        let a = Rc::new(Cons(42, RefCell::new(Rc::new(Nil))));
        let b = Rc::new(Cons(84, RefCell::new(Rc::clone(&a))));

        if let Some(tail) = a.tail() {
            *tail.borrow_mut() = Rc::clone(&b);
        }

        // will cause stack overflow
        // println!("a next item = {:?}", a.tail());
    }
}
