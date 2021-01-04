#[cfg(test)]
mod tests {
    
    use std::fmt::Display;

    struct Point<T> {
        x: T,
        y: T,
    }

    impl Point<f32> {
        fn distance(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    impl<T: Display + PartialOrd> Point<T> {
        fn display_largest(&self) {
            if self.x > self.y {
                println!("x = {}", self.x);
            } else {
                println!("y = {}", self.y);
            }
        }
    }

    #[test]
    fn type_specific_impl() {
        let _p1: Point<i32> = Point { x: 13, y: 37 };
        let p2: Point<f32> = Point { x: 13.0, y: 37.0 };
        // only available on p2
        println!("{}", p2.distance())
    }


    #[test]
    fn trait_bound_impl() {
        let p1: Point<i32> = Point { x: 13, y: 37 };
        let p2: Point<&str> = Point { x: "foo", y: "bar" };

        p1.display_largest();
        p2.display_largest()
    }

    trait Summary {
        fn summarize(&self) -> String;
    }

    fn notify1(summary: impl Summary) {
        println!("{}", summary.summarize());
    }

    fn notify2<T: Summary + Display>(summary: T) {
        println!("{}", summary.summarize());
    }

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &e in list.iter() {
            if e > largest {
                largest = e;
            }
        }

        largest
    }

    fn largest_no_copy<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = 0;

        for (i, e) in list.iter().enumerate() {
            if e > &list[largest] {
                largest = i;
            }
        }

        &list[largest]
    }

    #[test]
    fn generic_largest() {
        let list = vec![1, 3, 3, 7, 4, 2];
        let largest = largest(&list);
        let largest_no_copy = largest_no_copy(&list);
        assert_eq!(*largest_no_copy, largest)
    }

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    #[test]
    fn test_longest() {
        let s1 = String::from("very long string");
        {
            let s2 = String::from("xzy");
            let result = longest(s1.as_str(), s2.as_str());
            println!("longest = {}", result);
        }
    }
}
