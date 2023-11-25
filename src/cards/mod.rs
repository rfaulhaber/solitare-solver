use std::fmt::Display;
use std::hash::Hash;

pub trait Rank: Ord + Hash + Eq + Clone {
    fn ordinal(&self) -> usize;
}

pub trait Suit: Ord + Hash + Eq + Clone {}

#[derive(PartialOrd, PartialEq, Clone, Debug, Hash)]
pub struct Card<R: Rank, S: Suit> {
    pub rank: R,
    pub suit: S,
}

impl<R: Rank + Display, S: Suit + Display> Display for Card<R, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}
