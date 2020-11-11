use std::rc::Rc;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smart_pointers::rc_example::List::{Cons, Nil};

    #[test]
    fn cons_list() {
        let _ = Cons(1, Box::from(Cons(2, Box::from(Cons(3, Box::from(Nil))))));
    }

    #[test]
    fn shared_const_list() {
        let shared = Rc::new(RcList::Cons(
            5,
            Rc::new(RcList::Cons(10, Rc::new(RcList::Nil))),
        ));
        println!("{}", Rc::strong_count(&shared));
        let _ = RcList::Cons(3, Rc::clone(&shared));
        println!("{}", Rc::strong_count(&shared));
        {
            let _ = RcList::Cons(4, Rc::clone(&shared));
            println!("{}", Rc::strong_count(&shared));
        }
        println!("{}", Rc::strong_count(&shared));
    }
}
