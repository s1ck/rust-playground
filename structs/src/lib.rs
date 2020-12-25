#[cfg(test)]
mod tests {

    #[derive(Debug)]
    struct User {
        name: String,
        age: u16,
        email: String,
    }

    #[test]
    fn update_syntax() {
        let user1 = User {
            name: String::from("Alice"),
            age: 42,
            email: String::from("alice@unknown.org"),
        };

        println!("{:?}", user1);

        let user2 = User {
            name: String::from("Bob"),
            ..user1
        };

        println!("{:?}", user2);
    }

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    fn area2(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn new(width: u32, height: u32) -> Self {
            Rectangle { width, height }
        }

        fn square(size: u32) -> Self {
            Rectangle {
                width: size,
                height: size,
            }
        }

        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    #[test]
    fn rectangles() {
        let width1 = 30;
        let height1 = 50;

        let area = area(width1, height1);
        println!("area = {}", area);
        let area2 = area2((width1, height1));
        println!("area2 = {}", area2);

        let r = Rectangle {
            width: width1,
            height: height1,
        };
        println!("rectangle = {:?}", r);
        println!("rectangle.area() = {}", r.area());

        let r2 = Rectangle::new(15, 25);
        assert_eq!(true, r.can_hold(&r2))
    }
}
