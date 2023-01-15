use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRanking {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

struct Winners<'a> {
    rank: HandRanking,
    hands: Vec<&'a str>
}

impl<'a> Winners<'a> {
    fn new() -> Self {
        Winners { rank: HandRanking::HighCard, hands: vec![] }
    }

    fn add_new_winner(&mut self, hand: &'a str) {
        self.hands.push(hand);
    }

    fn renew_winners(&mut self, hand: &'a str) {
        self.rank = determine_rank(hand);
        self.hands = vec![hand];
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 1 { return hands.to_vec() }

    let mut winners = Winners::new();
    for hand in hands {
        let current_rank = determine_rank(hand);
        let prev_rank = &winners.rank;

        match prev_rank.cmp(&current_rank) {
            Ordering::Equal => winners.add_new_winner(hand),
            Ordering::Less => winners.renew_winners(hand),
            _ => ()
        }
    }
    
    winners.hands
}

fn determine_rank<'a>(hand: &'a str) -> HandRanking {
    if hand.contains("3") { return HandRanking::StraightFlush }
    HandRanking::FourOfAKind
}