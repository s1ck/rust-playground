#[cfg(test)]
mod tests {
    #[test]
    fn strings() {
        let mut s = String::from("Hello");
        s.push_str(", World!");

        println!("{}", s);
    }

    #[test]
    fn move_example() {
        let s = String::from("Foo");
        // s is "moved" into t
        let t = s;
        println!("{}", t);
    }

    #[test]
    fn deep_copy() {
        let s = String::from("webserver");
        // deep copy
        let t = s.clone();
        println!("s = {}, t = {}", s, t);
    }

    #[test]
    fn double_free() {
        let s = String::from("Hello");
        {
            // let t = s; moves ownership from s to t
            let t = &s;
            println!("t = {}", *t);
        }
        println!("s = {}", s);
    }

    // Annotating with Copy is possible, because
    // no struct member implements Drop.
    #[derive(Debug, Copy, Clone)]
    struct Foo {
        x: i32,
    }

    #[test]
    fn foo_test() {
        let s = Foo { x: 42 };
        let t = s;
        println!("s = {:?}, t = {:?}", s, t);
    }

    #[test]
    fn ownership() {
        let s = String::from("foo");

        // s is moved
        take_ownership(s);
        // not possible
        // println!("s = {}", s);


        let x = 5;
        // x is copied
        makes_copy(x);
        println!("x = {}", x);
    }

    fn take_ownership(s: String) {
        println!("s = {}", s);
    }

    fn makes_copy(x: i32) {
        println!("{}", x);
    }

    #[test]
    fn ownership_2() {
        let a = gives_ownership();
        println!("a = {}", a);

        let s = String::from("foo");
        let s = takes_and_gives_back(s);
        println!("s = {}", s);
    }

    fn gives_ownership() -> String {
        String::from("webserver")
    }

    fn takes_and_gives_back(a: String) -> String {
        a
    }

    #[test]
    fn references() {
        let s = String::from("haha");
        // s is "borrowed" here
        // using references is "borrowing"
        let l = length(&s);
        println!("s = {}, l = {}", s, l);
    }

    fn length(s: &String) -> usize {
        s.len()
    }

    #[test]
    fn mutable_references() {
        let mut s = String::from("huehue");
        change(&mut s);
        println!("s = {}", s);
    }

    fn change(s: &mut String) {
        s.push_str(" harhar")
    }

    #[test]
    fn slice() {
        let s = String::from("webserver world");
        let t = first_word(&s);
        println!("t = {}", t);

        let s = "hola hola";
        let t = first_word(&s);
        println!("t = {}", t);

    }

    fn first_word(s: &str) -> &str {
        for (i, &c) in s.as_bytes().iter().enumerate() {
            if c == b' ' {
                return &s[0..i]
            }
        }
        &s[..]
    }

    #[test]
    fn slice2() {
        let s = String::from("foobar");
        let x = &s[0..3];
        println!("{}", x);
    }
}
