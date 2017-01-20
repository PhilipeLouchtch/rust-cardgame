use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;


#[derive(Debug)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

macro_attr! {
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, IterVariants!(SuitVariants), IterVariantNames!(SuitVariantNames))]
    pub enum Suit {
        Clubs,
        Diamonds,
        Hearts,
        Spades,
    }
}

macro_attr! {
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, IterVariants!(RankVariants), IterVariantNames!(RankVariantNames))]
    pub enum Rank {
        Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
        Jack, Queen, King,
    }
}

impl Card {
    pub fn of(suit: Suit, rank: Rank) -> Card {
        Card { suit: suit, rank: rank }
    }
}
