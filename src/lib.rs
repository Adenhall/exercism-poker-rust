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
    hands: Vec<&'a str>,
}

impl<'a> Winners<'a> {
    fn new() -> Self {
        Winners {
            rank: HandRanking::HighCard,
            hands: vec![],
        }
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
    if hands.len() == 1 {
        return hands.to_vec();
    }

    let mut winners = Winners::new();
    for hand in hands {
        let current_rank = determine_rank(hand);
        let prev_rank = &winners.rank;

        match prev_rank.cmp(&current_rank) {
            Ordering::Equal => winners.add_new_winner(hand),
            Ordering::Less => winners.renew_winners(hand),
            _ => (),
        }
    }

    winners.hands
}

fn determine_rank<'a>(hand: &'a str) -> HandRanking {
    // "3S 4S 5D 6H JH"

    let (ranks, suits) = get_ranks_and_suits(hand);

    if is_straight(ranks) & is_flush(suits) {
        return HandRanking::StraightFlush;
    }

    HandRanking::HighCard
}

fn get_ranks_and_suits<'a>(hand: &'a str) -> (Vec<char>, Vec<char>) {
    let cards = hand.split(" "); // [3S, 4S, ...]
    let mut ranks: Vec<char> = vec![];
    let mut suits: Vec<char> = vec![];
    cards.for_each(|card| {
        let mut chars = card.chars();
        ranks.push(chars.next().unwrap());
        suits.push(chars.next().unwrap());
    });

    (ranks, suits)
}

fn is_straight(ranks: Vec<char>) -> bool {
    let order: Vec<char> = "A12345678910JQKA".chars().collect();

    for straight in order.windows(5) {
        let is_straight = ranks
            .iter()
            .all(|rank| straight.contains(rank));

        if is_straight {
            return true
        }
    }

    false
}

fn is_flush(ranks: Vec<char>) -> bool {
    let first = ranks[0];
    ranks.iter().all(|rank| rank.eq(&first))
}
