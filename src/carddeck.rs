use card;
use std::collections::HashMap;

#[derive(Debug)]
struct Private<'a> {
    deck: Vec<& 'a card::Card>,
    cardSingletons: HashMap<card::Suit, HashMap<card::Rank, card::Card>>,
}

#[derive(Debug)]
pub struct CardDeck<'a> {
    private: Private<'a>,
}

impl <'a> CardDeck<'a> {
    pub fn new() -> CardDeck<'a> {
        let mut cardz : HashMap<card::Suit, HashMap<card::Rank, card::Card>> = HashMap::with_capacity(4);

        for suit in card::Suit::iter_variants() {
            let mut suitCards = HashMap::with_capacity(13);

            for rank in card::Rank::iter_variants() {
                suitCards.insert(rank, card::Card::of(suit, rank));
            }

            cardz.insert(suit, suitCards);
        }


        // let mut clubs = HashMap::with_capacity(13);
        // clubs.insert(card::Rank::Ace, card::Card::of(card::Suit::Spades, card::Rank::Ace));
        // cards.insert(card::Suit::Spades, clubs);

        let private = Private { deck: vec![], cardSingletons: cardz };
        CardDeck{ private: private }
    }
}
