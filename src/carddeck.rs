use card;
use std::collections::HashMap;

#[derive(Debug)]
struct Private {
    deck: Vec<card::Card>,
}

#[derive(Debug)]
pub struct CardDeck {
    private: Private,
}

impl<'a> CardDeck {
    pub fn new() -> CardDeck {

        let mut vec_cards = Vec::with_capacity(52);

        for suit in card::Suit::iter_variants() {
            for rank in card::Rank::iter_variants() {;
                vec_cards.push(card::Card::of(suit, rank));
            }
        }

        CardDeck{ private: Private { deck: vec_cards } }
    }
}
