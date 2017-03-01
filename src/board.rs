use card::Card;
use carddeck::CardDeck;

#[derive(Debug)]
pub struct Board {
    cards: Vec<Card>,
}

impl Board {
    pub fn draw_from(mut deck: CardDeck, number_cards_to_draw: usize) -> Board {
        let mut cards = Vec::with_capacity(number_cards_to_draw);

        for _ in 0..number_cards_to_draw {
            match deck.draw_card() {
                Some(card) => cards.push(card),
                None => panic!("Could not draw card to make board"),
            }
        }

        Board { cards: cards }
    }
}