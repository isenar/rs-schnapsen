use strum::{Display, EnumIter, IntoEnumIterator};

/// Rank of the card (from 2 to Ace, including the Joker as well)
#[derive(Debug, Copy, Clone, PartialEq, Display, EnumIter)]
#[allow(missing_docs)]
pub enum Rank {
    #[strum(to_string = "2")]
    Two,
    #[strum(to_string = "3")]
    Three,
    #[strum(to_string = "4")]
    Four,
    #[strum(to_string = "5")]
    Five,
    #[strum(to_string = "6")]
    Six,
    #[strum(to_string = "7")]
    Seven,
    #[strum(to_string = "8")]
    Eight,
    #[strum(to_string = "9")]
    Nine,
    #[strum(to_string = "10")]
    Ten,
    #[strum(to_string = "J")]
    Jack,
    #[strum(to_string = "Q")]
    Queen,
    #[strum(to_string = "K")]
    King,
    #[strum(to_string = "A")]
    Ace,
}

impl Rank {
    /// Create an iterator over all of the possible card ranks
    pub fn all_ranks() -> RankIter {
        Self::iter()
    }
}
