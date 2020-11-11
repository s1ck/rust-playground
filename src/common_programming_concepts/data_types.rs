#[cfg(test)]
mod tests {
    #[test]
    fn explicit_types() {
        let x = "42".parse::<u32>().expect("Not a number!");
        println!("{}", x);
    }

    #[test]
    fn twos_complement() {
        println!("{:#b}", 5);
        println!("{:#b}", -5);
    }

    #[test]
    fn binary() {
        let x = 0b101;
        println!("{}", x);
    }

    #[test]
    fn binary2() {
        let x = 63;
        println!("{:#b}", x);
        println!("{:#b}", !x);

        const BLOCK_SIZE: i32 = 64;

        let page = 170;
        let block = page / BLOCK_SIZE;

        println!("page = {}", page);
        println!("block = {}", block);

        let offset_in_block = block * BLOCK_SIZE;
        let offset_in_block_bin = page & !63;

        println!("o1 = {:#b}", offset_in_block);
        println!("o2 = {:#b}", offset_in_block_bin);

        assert_eq!(offset_in_block, offset_in_block_bin)
    }

    #[test]
    fn tuple() {
        let x = (1, 3, 3, 7);
        let (a, b, c, d) = x;
        println!("{}", a);
        println!("{}", b);
        println!("{}", c);
        println!("{}", d);
        println!("{}", x.2);
    }

    #[test]
    fn array() {
        // arrays are allocated on the stack
        let x: [i32; 4] = [1, 3, 3, 7];
        println!("{:?}", x);
        let y = [42; 4];
        println!("{:?}", y);
        println!("{}", y[0]);
    }

    #[test]
    fn expression() {
        let x = {
            let y = 40;
            y + 2
        };
        println!("{}", x);
    }

    #[test]
    fn ternary() {
        let x = 5;
        let y = if x % 2 == 0 {
            0
        } else {
            1
        };
        println!("{}", y);
    }

    #[test]
    fn loop_example() {
        let mut i = 0;

        let x = loop {
            i = i + 1;

            if i == 10 {
                break i * 2
            }
        };

        println!("{}", x);
    }
}
