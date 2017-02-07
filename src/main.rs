#[macro_use] extern crate macro_attr;
#[macro_use] extern crate enum_derive;

mod card;
mod carddeck;

// A function for testing the creation and popping of deck
fn make_deck_and_print() {
    let mut card_deck = carddeck::CardDeck::new();
    println!("{:?}", card_deck);
    
    while let Some(card) = card_deck.drawTopCard() {
        println!("{:?}", card);
    }
}

fn main() {
    make_deck_and_print();
}
