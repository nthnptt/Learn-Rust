//!# Art
//!
//! A library for modeling artistique concepts.
pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;
pub mod kinds {
    /// The primary colors according to RYB color model
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue
    }

    ///The seconday colors accord to RYB color model
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple
    }
}

pub mod utils {
    use super::kinds::*;

    ///Combines two primaries colors to get one secondary color
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        //
    }
}
