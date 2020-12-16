use strum::{Display, EnumIter, IntoEnumIterator};

/// Representation of the standard card suit
#[derive(Debug, Clone, Copy, Display, EnumIter)]
#[allow(missing_docs)]
pub enum Suit {
    #[strum(to_string = "♣")]
    Clubs,
    #[strum(to_string = "♦")]
    Diamonds,
    #[strum(to_string = "♥")]
    Hearts,
    #[strum(to_string = "♠")]
    Spades,
}

impl Suit {
    /// Create an iterator over all of the possible card suits
    pub fn all_suits() -> SuitIter {
        Self::iter()
    }
}
