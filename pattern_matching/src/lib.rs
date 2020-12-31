#[cfg(test)]
mod tests {
    #[test]
    fn mixing() {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {} else if is_tuesday {} else if let Ok(age) = age {} else {
            // smooth
        }
    }

    #[test]
    fn while_let() {
        let mut stack = Vec::new();
        stack.push(42);
        stack.push(1337);
        stack.push(84);

        while let Some(e) = stack.pop() {
            println!("{}", e);
        }
    }

    #[test]
    fn let_test() {
        if let (x, y, None) = (1, 2, Some(3)) {
            println!("match");
        } else {
            println!("no match");
        }
    }

    fn print_point(&(x, y): &(u32, u32)) {
        println!("x = {}, y = {}", x, y);
    }

    #[test]
    fn print_point_test() {
        let point = (42, 1337);
        print_point(&point);
    }

    #[test]
    fn match_literals() {
        let x = 43;

        match x {
            1 => println!("foo"),
            42 => println!("tada"),
            _ => println!("😿"),
        }
    }

    #[test]
    fn match_named() {
        let x = Some(42);
        let y = 10;

        match x {
            Some(50) => println!("foo"),
            // y shadows outer variable
            Some(y) => println!("matched y = {}", y),
            _ => println!("{:?}", x),
        }
    }

    #[test]
    fn match_multiple() {
        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => print!("{}", x),
        }
    }

    #[test]
    fn match_range() {
        let x = 5;

        match x {
            // or `...` (inclusive range)
            1..=5 => println!("between one and five"),
            _ => println!("{}", x),
        }

        let x = 'm';

        match x {
            'a'..='k' => println!("early ascii letter"),
            'l'..='z' => println!("late ascii letter"),
            _ => println!("{}", x),
        }
    }

    #[derive(Debug)]
    struct MyStruct {
        x: i32,
        y: String,
        z: Vec<usize>,
    }

    #[test]
    fn destructuring_structs() {
        let s = MyStruct { x: 42, y: String::from("foobar"), z: vec![1, 3, 3, 7] };

        // x is short-hand, y and z rename the field variable
        let MyStruct { x, y: b, z: c } = s;

        // doesn't work since s is partially moved because of z (Vec)
        // println!("{:?}", s);

        println!("a = {:?}, b = {:?}, c = {:?}", x, b, c);
    }

    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn match_literal() {
        let p = Point { x: 0, y: 7 };

        match p {
            Point { x, y: 0 } => println!("on the x-axis at {}", x),
            Point { x: 0, y } => println!("on the y-axis at {}", y),
            Point { x, y } => println!("on neither axis at: ({},{})", x, y),
        }
    }

    #[derive(Debug)]
    enum Exercise {
        Cycle(Intensity),
        Run,
    }

    #[derive(Debug)]
    enum Intensity {
        High,
        Medium,
        Low,
    }

    #[test]
    fn match_nested() {
        // let c = Exercise::Cycle(Intensity::Low);
        let c = Exercise::Run;

        match c {
            Exercise::Cycle(Intensity::High) => println!("wow"),
            Exercise::Cycle(intensity) => println!("cycling with intensity: {:?}", intensity),
            _ => println!("nice exercise: {:?}", c),
        }
    }

    #[test]
    fn ignore_matches() {
        let x = Some(42);
        let y = Some(1337);

        match (x, y) {
            (Some(_), Some(_)) => println!("quite some tuples"),
            _ => println!("moep"),
        }

        let nums = (2, 4, 6, 8, 16);

        match nums {
            (one, two, _, _, _) => println!("{}{}", two, one),
        }

        let s = MyStruct { x: 42, y: String::from("foobar"), z: vec![1, 3, 3, 7] };

        // ignore parts of the struct
        let MyStruct { x, .. } = s;

        println!("{}", x);
    }

    #[test]
    fn match_conditional() {
        let num = Some(42);

        match num {
            Some(num) if num < 40 => println!("less than 40"),
            Some(num) if num == 42 => println!("the answer!"),
            Some(num) => println!("{}", num),
            None => println!(":("),
        }
    }

    enum Message {
        Hello { id: i32 },
    }

    #[test]
    fn match_binding() {
        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello { id: id_variable @ 3...7 } => println!("found id in range 3 to 7: {}", id_variable),
            // doesn't know which one it is ... can't assign id in pattern
            Message::Hello { id: 10...12 } => println!("found id in range 10 to 12"),
            Message::Hello { id: id } => println!("found some other id"),
        }
    }
}
