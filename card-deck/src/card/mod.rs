mod rank;
mod suit;

pub use rank::Rank;
pub use suit::Suit;

/// Standard
#[derive(Debug, Copy, Clone)]
pub struct Card {
    /// Suit of the card
    suit: Suit,
    /// Rank of the card
    rank: Rank,
}

impl Card {
    /// Create a card with given suit and rank
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}
