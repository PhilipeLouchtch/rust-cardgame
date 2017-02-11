use card;

#[derive(Debug)]
pub struct Hand {
    cards: Vec<card::Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    pub fn accept(&mut self, card: card::Card) {
        self.cards.push(card);
    }
}