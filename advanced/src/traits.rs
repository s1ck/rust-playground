#[cfg(test)]
mod tests {
    use std::fmt;
    use std::fmt::{Display, Formatter};
    use std::ops::{Add, Deref};

    #[derive(PartialEq, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Display for Point {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // add is generic, but uses Self as default RHS type
    impl Add for Point {
        type Output = Point;

        fn add(self, rhs: Self) -> Self::Output {
            Point { x: self.x + rhs.x, y: self.y + rhs.y }
        }
    }

    #[test]
    fn default_generic_type() {
        let p1 = Point { x: 42, y: 42 };
        let p2 = Point { x: 84, y: 123 };

        assert_eq!(p1 + p2, Point { x: 42 + 84, y: 42 + 123 })
    }

    #[derive(PartialEq, Debug)]
    struct Kilometers(u32);

    #[derive(PartialEq, Debug)]
    struct Meters(u32);

    impl Add<Kilometers> for Meters {
        type Output = Meters;

        fn add(self, rhs: Kilometers) -> Self::Output {
            Meters { 0: self.0 + rhs.0 * 1000 }
        }
    }

    #[test]
    fn km_add_m() {
        let km = Kilometers(10);
        let m = Meters(42);

        let m = m + km;

        assert_eq!(m, Meters(10042))
    }

    trait Foobar {
        fn foo() -> String;
    }

    struct Baz {}

    impl Baz {
        fn foo() -> String {
            String::from("baz")
        }
    }

    impl Foobar for Baz {
        fn foo() -> String {
            String::from("foobar")
        }
    }

    #[test]
    fn foobarbaz() {
        let mut s = String::from("tada");
        String::push_str(&mut s, "dadad");

        println!("{}", s);

        let s1 = Baz::foo();
        let s2 = <Baz as Foobar>::foo();

        println!("{} {}", s1, s2);
    }

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();

            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    impl OutlinePrint for Point {}

    #[test]
    fn outline_print_test() {
        let p = Point { x: 42, y: 84 };

        p.outline_print();
    }

    // newtype pattern
    struct Wrapper(Vec<String>);

    impl Display for Wrapper {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "<{}>", self.0.join(", "))
        }
    }

    // get methods from Vec using Deref
    impl Deref for Wrapper {
        type Target = Vec<String>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    // consume wrapper and convert into Vec
    impl From<Wrapper> for Vec<String> {
        fn from(wrapper: Wrapper) -> Self {
            wrapper.0
        }
    }

    #[test]
    fn newtype_test() {
        let w = Wrapper(vec![String::from("foo"), String::from("bar"), String::from("baz")]);


        println!("{}", w);
        println!("{}", w.len());

        let v: Vec<String> = w.into();
        println!("{:?}", v);
        println!("{}", v.len());
    }
}