#[macro_use] extern crate macro_attr;
#[macro_use] extern crate enum_derive;

mod card;
mod carddeck;

fn main() {
    // println!("{:?}", Rank);
    // let ace_of_spades = card::Card::of(card::Suit::Spades, card::Rank::Ace);

    // println!("{:?}", ace_of_spades);
    
    for i in 1..100000 {
        let card_deck = carddeck::CardDeck::new();
    }
}
