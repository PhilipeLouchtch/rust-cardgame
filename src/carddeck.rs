use card;
use vecshuffle::Shufflable;

#[derive(Debug)]
pub struct CardDeck {
    cards: Vec<card::Card>,
}

impl CardDeck {
    pub fn new() -> CardDeck {

        let mut vec_cards = Vec::with_capacity(52);

        for suit in card::Suit::iter_variants() {
            for rank in card::Rank::iter_variants() {;
                vec_cards.push(card::Card::of(suit, rank));
            }
        }

        vec_cards.shuffle();
        CardDeck{ cards: vec_cards }
    }

    pub fn draw_card(&mut self) -> Option<card::Card> {
        self.cards.pop()
    }
}
