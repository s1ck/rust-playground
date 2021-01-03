#[cfg(test)]
mod tests {
    use super::*;

    fn add(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    #[test]
    fn function_pointer() {
        let x = do_twice(add, 5);

        assert_eq!(12, x)
    }

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    #[test]
    fn initializer_function() {
        let v = (0_u32..42).map(Status::Value).collect::<Vec<_>>();
        println!("{:?}", v);
    }

    fn return_closure() -> Box<dyn Fn(i32) -> i32> {
        // won't work since closure is not sized
        // |x| x + 1
        // Box is sized -> works
        Box::new(|x| x + 1)
    }


    #[test]
    fn return_closure_test() {
        let x = return_closure()(5);
        assert_eq!(6, x)
    }



}