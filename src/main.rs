#[macro_use] extern crate macro_attr;
#[macro_use] extern crate enum_derive;
extern crate rand;

mod gamelogic;

use gamelogic::carddeck;
use gamelogic::hand;
use gamelogic::board;

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

    hand.draw_from(&mut card_deck, 2);

    print!("Hand: {:#?}", hand);
}

fn make_deck_and_deal_board() {
    let mut card_deck = carddeck::CardDeck::new();
    let board = board::Board::draw_from(&mut card_deck, 5);

    print!("{:?}", board);
    print!("{:?}", card_deck);
}

fn make_deck_draw_hand_and_return_to_deck() {
    let mut card_deck = carddeck::CardDeck::new();
    
    let mut hand = hand::Hand::new();
    hand.draw_from(&mut card_deck, 2);

    card_deck.return_card(hand.card(0));
}

fn main() {
    make_deck_and_print();
    make_deck_and_take_hand();
    make_deck_and_deal_board();
    make_deck_draw_hand_and_return_to_deck();
}
