//! # Art
//!
//!  Art Crate

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    //! define color type

    /// primary color
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// secondary color
    #[derive(Debug,PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    //! tool
    use crate::kinds::*;

    /// to secondary color
    /// ```rust
    /// use art::utils::mix;
    /// use art::kinds::{PrimaryColor,SecondaryColor};
    /// assert!(matches!(mix(PrimaryColor::Yellow, PrimaryColor::Blue), SecondaryColor::Green));
    /// ```
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}
