use card;
use std::collections::HashMap;

#[derive(Debug)]
struct Private {
    deck: Vec<card::Card>,
    cardSingletons: HashMap<card::Suit, HashMap<card::Rank, card::Card>>,
}

#[derive(Debug)]
pub struct CardDeck {
    private: Private,
}

impl  CardDeck {
    pub fn new() -> CardDeck {
        let mut cardz : HashMap<card::Suit, HashMap<card::Rank, card::Card>> = HashMap::with_capacity(4);

        for suit in card::Suit::iter_variants() {
            let mut suitCards = HashMap::with_capacity(13);

            for rank in card::Rank::iter_variants() {
                suitCards.insert(rank, card::Card::of(suit, rank));
            }

            cardz.insert(suit, suitCards);
        }

        let private = Private { deck: vec![], cardSingletons: cardz };
        CardDeck{ private: private }
    }
}
