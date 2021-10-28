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
        let mixedColor: (PrimaryColor, PrimaryColor) = (c1,c2);
        // write by switch sentence
        match mixedColor {
            (Red, Yellow) => crate::kinds::SecondaryColor::Green,
            (Red, Blue) => crate::kinds::SecondaryColor::Purple,
            (Green, Purple) => crate::kinds::SecondaryColor::Orange,
        }



    }


}


