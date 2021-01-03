use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Foobar {}

#[macro_export]
macro_rules! vector {
    ( $( $x: expr ), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declarative_macro_test() {
        let v = vector!(1, 3, 3, 7);
        assert_eq!(vec![1, 3, 3, 7], v)
    }

    #[test]
    fn hello_macro() {
        Foobar::hello_macro();
    }
}