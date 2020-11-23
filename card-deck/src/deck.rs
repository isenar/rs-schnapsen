use crate::{Card, Rank, Suit};

use itertools::iproduct;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
/// Representation of the deck of cards
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Create a standard 52-card deck
    pub fn standard_deck() -> Self {
        let cards = iproduct!(Suit::all_suits(), Rank::all_ranks())
            .map(|(suit, rank)| Card::new(suit, rank))
            .collect();

        Self { cards }
    }

    /// Create a deck specific for Schnapsen games - ranks from 9 to Ace of all 4 suits
    pub fn schnapsen_deck() -> Self {
        let cards = iproduct!(Suit::all_suits(), Rank::all_ranks())
            .map(|(suit, rank)| Card::new(suit, rank))
            .collect();

        Self { cards }
    }

    /// Shuffle the deck
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    /// Draw a card from the top of the deck
    /// Returns [`None`] if the deck is empty
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}
