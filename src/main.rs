#[macro_use] extern crate macro_attr;
#[macro_use] extern crate enum_derive;
extern crate rand;

mod gamelogic;
mod handdetermination;

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

    println!("Hand: {:#?}", hand);
}

fn make_deck_and_deal_board() {
    let mut card_deck = carddeck::CardDeck::new();
    let board = board::Board::draw_from(&mut card_deck, 5);

    println!("{:?}", board);
    println!("{:?}", card_deck);
}

fn make_deck_draw_hand_and_return_to_deck() {
    let mut card_deck = carddeck::CardDeck::new();
    
    let mut hand = hand::Hand::new();
    hand.draw_from(&mut card_deck, 2);

    card_deck.return_card(hand.card(0));
}

fn determine_winner_from_two_hands() {
    let mut card_deck = carddeck::CardDeck::new();
    
    let mut villain_hand = hand::Hand::new();
    villain_hand.draw_from(&mut card_deck, 2);

    let mut hero_hand = hand::Hand::new();
    hero_hand.draw_from(&mut card_deck, 2);

    let winner = handdetermination::determine_winning_hand(hero_hand, villain_hand);

    print!("winner: {:?}", winner);
}

fn main() {
    make_deck_and_print();
    make_deck_and_take_hand();
    make_deck_and_deal_board();
    make_deck_draw_hand_and_return_to_deck();
    determine_winner_from_two_hands();
}
