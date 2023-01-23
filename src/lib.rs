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

const CARD_RANKING: &str = "A2345678910JQKA";

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
    let grouped_ranks = group_ranks(&ranks);

    if is_straight(&ranks) & is_flush(&suits) {
        return HandRanking::StraightFlush;
    }

    if let (4, _) = grouped_ranks {
        return HandRanking::FourOfAKind;
    }

    if let (3, 2) = grouped_ranks {
        return HandRanking::FullHouse;
    }

    if is_flush(&suits) {
        return HandRanking::Flush;
    }

    if is_straight(&ranks) {
        return HandRanking::Straight;
    }

    if let (3, _) = grouped_ranks {
        return HandRanking::ThreeOfAKind;
    }

    if let (2, 2) = grouped_ranks {
        return HandRanking::TwoPair;
    }

    if let (2, _) = grouped_ranks {
        return HandRanking::OnePair;
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

fn is_straight(ranks: &Vec<char>) -> bool {
    let order: Vec<char> = CARD_RANKING.chars().collect();

    for straight in order.windows(5) {
        if ranks.iter().all(|rank| straight.contains(rank)) {
            return true;
        }
    }

    false
}

fn is_flush(suits: &Vec<char>) -> bool {
    let first = suits[0];
    suits.iter().all(|suit| suit.eq(&first))
}

fn group_ranks(ranks: &Vec<char>) -> (i16, i16) {
    let mut cloned_ranks = ranks.clone();
    cloned_ranks.sort();
    let mut result = (1, 0);
    for pair in cloned_ranks.windows(2) {
        if (result.1 as i16) == 0 {
            if pair[0] == pair[1] {
                result.0 += 1;
            }

            if pair[0] != pair[1] {
                result.1 += 1;
            }
        }

        if pair[0] == pair[1] {
            result.1 += 1;
        }
    }

    (result.0.max(result.1), result.0.min(result.1))
}
