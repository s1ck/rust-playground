#[cfg(test)]
mod tests {

    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    enum IpAddrEnum {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 }, // anonymous struct definition
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            match self {
                Message::Move {x, y} => println!("x = {}, y = {}", x, y),
                _ => {}
            }
        }
    }

    #[test]
    fn enums() {
        let _four = IpAddrKind::V4;

        let _home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let _loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };

        let _home = IpAddrEnum::V4(127, 0, 0, 1);

        let m = Message::Move { x: 13, y: 37 };
        m.call();
    }

    #[test]
    fn options() {
        let _some_number = Some(42);
        let _some_string = Some("string");
        let _absent_number: Option<i32> = None;
    }

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // ...
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(UsState::Alabama) => {
                println!("Sweet home Alabama!");
                25
            },
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            },
        }
    }

    #[test]
    fn matching() {
        let c = value_in_cents(Coin::Nickel);
        println!("{}", c);
        let c = value_in_cents(Coin::Quarter(UsState::Alabama));
        println!("{}", c);
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            None => None,
        }
    }

    #[test]
    fn plus_one_test() {
        let x = plus_one(Some(42));
        println!("{:?}", x);
        let x = plus_one(None);
        println!("{:?}", x);
    }

    #[test]
    fn if_let() {
        let x = Some(3);

        if let Some(3) = x {
            println!("woop");
        }
    }
}
