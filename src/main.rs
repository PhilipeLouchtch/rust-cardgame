#[macro_use] extern crate macro_attr;
#[macro_use] extern crate enum_derive;
extern crate rand;

mod card;
mod carddeck;
mod hand;
mod vecshuffle;

// A function for testing the creation and popping of deck
fn make_deck_and_print() {
    let mut card_deck = carddeck::CardDeck::new();
    println!("{:?}", card_deck);

    while let Some(card) = card_deck.draw_card() {
        println!("{:?}", card);
    }
}

fn make_deck_and_take_hand() {
    let mut card_deck = carddeck::CardDeck::new();

    let mut hand = hand::Hand::new();

    for _ in 0..2 {
        match card_deck.draw_card() {
            Some(card) => hand.accept(card),
            None => panic!("Deck contained insufficient cards to draw a hand"),
        }
    }

    print!("Hand: {:#?}", hand);
}

fn main() {
    make_deck_and_take_hand();
}
