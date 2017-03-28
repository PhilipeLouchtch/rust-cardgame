use gamelogic::card;
use gamelogic::card::Card;
use gamelogic::vecshuffle::Shufflable;

#[derive(Debug)]
pub struct CardDeck {
    cards: Vec<Card>,
}

impl CardDeck {
    pub fn new() -> CardDeck {

        let mut vec_cards = Vec::with_capacity(52);

        for suit in card::Suit::iter_variants() {
            for rank in card::Rank::iter_variants() {;
                vec_cards.push(Card::of(suit, rank));
            }
        }

        vec_cards.shuffle();
        CardDeck{ cards: vec_cards }
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn return_card(&mut self, card_to_return: Card) {
        self.cards.push(card_to_return);
    }

    pub fn reshuffle(&mut self) {
        self.cards.shuffle();
    }
}
