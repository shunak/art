//! # Art
//!
//! A library for modeling artistic concepts.

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;




pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;
    use crate::PrimaryColor::Red;
    use crate::PrimaryColor::Blue;
    use crate::PrimaryColor::Yellow;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> crate::kinds::SecondaryColor {
        // --snip--
        // write by switch sentence
        if c1 == Red && c2 == Yellow {
            return SecondaryColor::Green
        }
        if c1 == Red && c2 == Blue {
            return SecondaryColor::Purple
        }
        if c1 == Blue && c2 == Yellow {
            return SecondaryColor::Orange
        }
    }


}


