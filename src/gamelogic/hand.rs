use gamelogic::card::Card;
use gamelogic::carddeck::CardDeck;

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    pub fn accept(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn draw_from(&mut self, deck: &mut CardDeck, num_to_draw: usize) {
        for _ in 0..num_to_draw {
            match deck.draw_card() {
                Some(card) => self.accept(card),
                None => panic!("Deck contained insufficient cards to draw a hand"),
            }
        }
    }

    pub fn card(&mut self, cardIndex: usize) -> Card {
        self.cards.remove(cardIndex)
    }
}