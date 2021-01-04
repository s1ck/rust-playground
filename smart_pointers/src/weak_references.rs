use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod tests {
    

    use super::*;

    #[test]
    fn leaf_node() {
        let leaf = Rc::new(Node {
            value: 42,
            children: RefCell::new(vec![]),
        });

        let _branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });


    }
}
