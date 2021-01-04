pub mod kinds {
    pub enum PrimaryColor {
        Red, Yellow, Blue,
    }

    pub enum SecondaryColor {
        Orange, Green, Purple,
    }
}

pub mod utils {
    use crate::cargo_crates::kinds::{PrimaryColor, SecondaryColor};

    /// Combines the two primary colors in foo foo foo
    pub fn mix(_c1: PrimaryColor, _c2: PrimaryColor) -> SecondaryColor {
        todo!()
    }
}
