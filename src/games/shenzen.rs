use crate::cards::{Card, Deck, Rank, Suit};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShenzenSuit {
    Green,
    Red,
    Black,
    Flower,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShenzenRank {
    Dragon,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Rank for ShenzenRank {
    fn ordinal(&self) -> usize {
        match self {
            ShenzenRank::Dragon => 11,
            ShenzenRank::One => 1,
            ShenzenRank::Two => 2,
            ShenzenRank::Three => 3,
            ShenzenRank::Four => 4,
            ShenzenRank::Five => 5,
            ShenzenRank::Six => 6,
            ShenzenRank::Seven => 7,
            ShenzenRank::Eight => 8,
            ShenzenRank::Nine => 9,
        }
    }
}

impl Suit for ShenzenSuit {}

pub struct ShenzenDeck {}

impl ShenzenDeck {
    fn generate() -> Vec<Card<ShenzenRank, ShenzenSuit>> {
        vec![
            Card {
                rank: ShenzenRank::Dragon,
                suit: ShenzenSuit::Green,
            },
            Card {
                rank: ShenzenRank::Dragon,
                suit: ShenzenSuit::Green,
            },
            Card {
                rank: ShenzenRank::Dragon,
                suit: ShenzenSuit::Green,
            },
            Card {
                rank: ShenzenRank::Dragon,
                suit: ShenzenSuit::Green,
            },
            Card {
                rank: ShenzenRank::Dragon,
                suit: ShenzenSuit::Red,
            },
            Card {
                rank: ShenzenRank::Dragon,
                suit: ShenzenSuit::Red,
            },
            Card {
                rank: ShenzenRank::Dragon,
                suit: ShenzenSuit::Red,
            },
            Card {
                rank: ShenzenRank::Dragon,
                suit: ShenzenSuit::Red,
            },
            Card {
                rank: ShenzenRank::Dragon,
                suit: ShenzenSuit::Black,
            },
            Card {
                rank: ShenzenRank::Dragon,
                suit: ShenzenSuit::Black,
            },
            Card {
                rank: ShenzenRank::Dragon,
                suit: ShenzenSuit::Black,
            },
            Card {
                rank: ShenzenRank::Dragon,
                suit: ShenzenSuit::Black,
            },
        ]
    }
}
