#![deny(missing_docs)]
//! Utility crate for creating generic decks of cards

mod card;
mod deck;

pub use card::{Card, Rank, Suit};
pub use deck::Deck;
