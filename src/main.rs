#[macro_use] extern crate macro_attr;
#[macro_use] extern crate enum_derive;

mod card;
mod carddeck;

// use card;

enum Henk {
    StructLike { a: i32, b: i32, },
    StructLike2 { a: i32, b: i32, c: i32 },
    TupleLike(i32, i32),
    UnitLike,
}

fn main() {
    // println!("{:?}", Rank);
    let ace_of_spades = card::Card::of(card::Suit::Spades, card::Rank::Ace);

    println!("{:?}", ace_of_spades);

    let card_deck = carddeck::CardDeck::new();
    println!("{:#?}", card_deck);    
}
