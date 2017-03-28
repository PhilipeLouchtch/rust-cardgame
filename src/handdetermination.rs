use gamelogic::hand::Hand;

#[derive(Debug)]
pub enum WinningHand {
    Hero,
    Villain,
}

pub fn determine_winning_hand(hero_hand: Hand, villain_hand: Hand) -> WinningHand {
    WinningHand::Villain
}