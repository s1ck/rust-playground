use std::ops::Rem;
use std::{convert::TryInto, fmt::Debug};

trait Even {
    fn is_even(self) -> bool;
}

// generic blanket impl
impl<T> Even for T
where
    T: Rem<Output = T> + PartialEq<T> + Sized,
    u8: TryInto<T>,
    <u8 as TryInto<T>>::Error: Debug,
{
    fn is_even(self) -> bool {
        println!("{}", std::any::type_name::<T>());
        self % 2.try_into().unwrap() == 0.try_into().unwrap()
    }
}

// does not work to ensure trait coherence
//impl Even for u8 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_even_test() {
        assert!(32.is_even());
        assert!(32_u8.is_even())
    }
}
