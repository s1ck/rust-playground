use std::{mem, str::FromStr};

trait LeftPad {
    fn left_pad(&mut self, fill: char, n_chars: usize);
}

impl LeftPad for String {
    fn left_pad(&mut self, fill: char, n_chars: usize) {
        (0..n_chars).for_each(|_idx| self.insert(0, fill));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut input = String::from("foobar");

        input.left_pad('_', 4);

        assert_eq!(input, String::from("____foobar"));
    }
}
