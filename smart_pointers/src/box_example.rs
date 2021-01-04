use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("drop it like it's hot");
    }
}

fn deref_coercion(s: &str) {
    println!("{}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_example::MyBox;

    #[test]
    fn reference() {
        let x = 4;
        let y = &x;

        assert_eq!(x, *y);
    }

    #[test]
    fn reference_with_box() {
        let x = 4;
        let y = Box::from(x);

        assert_eq!(x, *y)
    }

    #[test]
    fn reference_with_mybox() {
        let x = 4;
        let y = MyBox::new(x);

        assert_eq!(x, *y);
    }

    #[test]
    fn deref_coercion_test() {
        let s = MyBox::new(String::from("webserver"));
        deref_coercion(&s);
        // same as
        deref_coercion(&(*s)[..]);
    }
}
