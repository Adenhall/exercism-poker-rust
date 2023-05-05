use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRanking {
    HighCard(i16),
    OnePair(i16),
    TwoPair(i16),
    ThreeOfAKind(i16),
    Straight(i16),
    Flush(i16),
    FullHouse(i16),
    FourOfAKind(i16),
    StraightFlush(i16),
}

const CARD_RANKING: [char; 14] = [
    'A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

struct Winners<'a> {
    rank: HandRanking,
    hands: Vec<&'a str>,
}

impl<'a> Winners<'a> {
    fn new() -> Self {
        Winners {
            rank: HandRanking::HighCard(0),
            hands: vec![],
        }
    }

    fn add_new_winner(&mut self, hand: &'a str) {
        self.hands.push(hand);
    }

    fn renew_winners(&mut self, hand: &'a str, hand_ranking: HandRanking) {
        self.rank = hand_ranking;
        self.hands = vec![hand];
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 1 {
        return hands.to_vec();
    }

    let mut winners = Winners::new();
    for hand in hands {
        let next_rank = determine_rank(hand);
        let current_rank = &winners.rank;
        match current_rank.cmp(&next_rank) {
            Ordering::Equal => winners.add_new_winner(hand),
            Ordering::Less => winners.renew_winners(hand, next_rank),
            Ordering::Greater => (),
        }
    }

    winners.hands
}

fn determine_rank<'a>(hand: &'a str) -> HandRanking {
    let (ranks, suits) = get_ranks_and_suits(hand);
    let score = get_high_score(&ranks);
    let grouped_ranks = group_ranks(&ranks);

    if is_straight(&ranks) & is_flush(&suits) {
        return HandRanking::StraightFlush(score);
    }

    if let (4, 0) = grouped_ranks {
        return HandRanking::FourOfAKind(score);
    }

    if let (3, 2) = grouped_ranks {
        return HandRanking::FullHouse(score);
    }

    if is_flush(&suits) {
        return HandRanking::Flush(score);
    }

    if is_straight(&ranks) {
        return HandRanking::Straight(score);
    }

    if let (3, 0) = grouped_ranks {
        return HandRanking::ThreeOfAKind(score);
    }

    if let (2, 2) = grouped_ranks {
        return HandRanking::TwoPair(score);
    }

    if let (2, 0) = grouped_ranks {
        return HandRanking::OnePair(score);
    }

    HandRanking::HighCard(score)
}

fn get_ranks_and_suits<'a>(hand: &'a str) -> (Vec<char>, Vec<char>) {
    let cards = hand.split(" ").map(|card| card.replace("10", "T"));
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
    for straight in CARD_RANKING.windows(5) {
        let mut sorted_ranks = ranks.to_vec();
        let mut sorted_straight = straight.to_vec();
        sorted_ranks.sort();
        sorted_straight.sort();

        if sorted_ranks == sorted_straight {
            return true;
        }
    }

    false
}

fn is_flush(suits: &Vec<char>) -> bool {
    let first = suits[0];
    suits.iter().all(|suit| suit.eq(&first))
}

fn group_ranks(ranks: &[char]) -> (i16, i16) {
    let mut sorted_ranks = ranks.to_vec();
    sorted_ranks.sort();
    let first_four = {
        let first = &sorted_ranks[..=3].first().unwrap();
        let last = &sorted_ranks[..=3].last().unwrap();
        match first.eq(last) {
            true => &sorted_ranks[..=3],
            false => &sorted_ranks[1..=4],
        }
    };
    let first_four_are_same = first_four.first().map_or(false, |first| first_four.iter().all(|elem| elem == first));
    let first_three = {
        let first = &sorted_ranks[..=2].first().unwrap();
        let last = &sorted_ranks[..=2].last().unwrap();
        match first.eq(last) {
            true => &sorted_ranks[..=2],
            false => &sorted_ranks[2..=4],
        }
    };
    let first_three_are_same = first_three.first().map_or(false, |first| first_three.iter().all(|elem| elem == first));

    sorted_ranks.dedup();
    match sorted_ranks.len() {
        2 if first_four_are_same => (4, 0),
        2 => (3, 2),
        3 if first_three_are_same => (3, 0),
        3 => (2, 2),
        4 => (2, 0),
        _ => (0, 0)
    }
}

fn get_high_score(ranks: &Vec<char>) -> i16 {
    ranks.iter().fold(0, |acc, rank| {
        let rank_by_idx = CARD_RANKING.iter().position(|x| x == rank).unwrap_or(0) as i16;
        rank_by_idx.max(acc)
    })
}
