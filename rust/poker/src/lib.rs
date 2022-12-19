// /// Given a list of poker hands, return a list of those hands which win.
// ///
// /// Note the type signature: this function should return _the same_ reference to
// /// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::cmp::Reverse;
use std::collections::HashMap;

/**
 * Card
 */
#[derive(PartialEq, Eq)]
enum CardKind {
    Club,
    Diamond,
    Heart,
    Spade,
    NotFound,
}

struct Card {
    kind: CardKind,
    value: u8,
}

impl Card {
    fn from(card: &str) -> Self {
        let length = card.len();
        let kind = Self::kind(card.get(length - 1..));

        let str_val = card.get(..length - 1).unwrap();
        let value: u8 = Self::value(str_val);

        Self { kind, value }
    }

    fn kind(k: Option<&str>) -> CardKind {
        match k {
            Some("C") => CardKind::Club,
            Some("D") => CardKind::Diamond,
            Some("H") => CardKind::Heart,
            Some("S") => CardKind::Spade,
            _ => CardKind::NotFound,
        }
    }

    fn value(val: &str) -> u8 {
        match val.parse() {
            Ok(v) => v,
            _ => match val {
                "J" => 11,
                "Q" => 12,
                "K" => 13,
                "A" => 14,
                _ => 0,
            },
        }
    }
}

/**
 * Hand
 */
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    Straight,
    Flush,
    FullHouse,
    FourOfKind,
    StraightFlush,
}

impl Default for HandRank {
    fn default() -> Self {
        HandRank::HighCard
    }
}

#[derive(Default)]
struct Rank {
    main: HandRank,
    major: u8,
    minor: u8,
    left: Vec<u8>,
}

#[derive(Default)]
struct Hand {
    idx: usize,
    rank: Rank,
}

impl Hand {
    fn from(hand: &str, idx: usize) -> Self {
        let str_cards = hand.split(' ').collect::<Vec<&str>>();
        let mut cards: Vec<Card> = vec![];

        for card in str_cards {
            cards.push(Card::from(card));
        }

        if cards.iter().max_by_key(|c| c.value).unwrap().value == 14
            && cards.iter().any(|c| c.value == 2)
            && cards.iter().any(|c| c.value == 3)
            && cards.iter().any(|c| c.value == 4)
            && cards.iter().any(|c| c.value == 5)
        {
            let ace = cards.iter().position(|c| c.value == 14).unwrap();
            cards.get_mut(ace).unwrap().value = 1;
        }

        cards.sort_by_key(|c| Reverse(c.value));

        let rank = Self::hand_rank(&cards);

        Self { idx, rank }
    }

    fn hand_rank(cards: &Vec<Card>) -> Rank {
        if cards.is_empty() {
            return Rank::default();
        }

        let first = cards.first().unwrap();
        let mut is_straight_flush = true;
        let mut is_flush = true;
        let mut is_straight = true;

        if first.value >= 5 {
            let mut counter = 1;

            for card in cards.get(1..).unwrap() {
                if card.value != first.value - counter {
                    is_straight_flush = false;
                    is_straight = false;
                }

                if card.kind != first.kind {
                    is_straight_flush = false;
                    is_flush = false;
                }

                counter += 1;
            }
        }

        if is_straight_flush {
            return Rank {
                main: HandRank::StraightFlush,
                major: cards.iter().max_by_key(|card| card.value).unwrap().value,
                ..Default::default()
            };
        }

        let mut hm: HashMap<u8, u8> = HashMap::new();

        for card in cards.iter() {
            *hm.entry(card.value).or_insert(0) += 1;
        }

        let keys_count = hm.len();

        if hm.values().any(|&v| v == 4) {
            return Rank {
                main: HandRank::FourOfKind,
                major: *hm.iter().max_by_key(|(_, &v)| v).unwrap().0,
                minor: *hm.iter().min_by_key(|(_, &v)| v).unwrap().0,
                ..Default::default()
            };
        }

        if keys_count == 2 && hm.values().any(|&v| v == 3) {
            return Rank {
                main: HandRank::FullHouse,
                major: *hm.iter().max_by_key(|(_, &v)| v).unwrap().0,
                minor: *hm.iter().min_by_key(|(_, &v)| v).unwrap().0,
                ..Default::default()
            };
        }

        if is_flush {
            return Rank {
                main: HandRank::Flush,
                major: cards[0].value,
                minor: cards[1].value,
                left: cards[2..].iter().map(|card| card.value).collect(),
            };
        }

        if is_straight {
            return Rank {
                main: HandRank::Straight,
                major: cards[0].value,
                ..Default::default()
            };
        }

        if hm.values().any(|&v| v == 3) {
            let major = *hm.iter().max_by_key(|(_, &v)| v).unwrap().0;
            let minor = *hm.keys().filter(|&&key| key != major).max().unwrap();

            return Rank {
                main: HandRank::ThreeOfKind,
                major,
                minor,
                left: cards
                    .iter()
                    .filter(|card| ![major, minor].contains(&card.value))
                    .map(|card| card.value)
                    .collect(),
            };
        }

        if keys_count == 3 && hm.values().any(|&v| v == 2) {
            let mut pairs: Vec<u8> = hm
                .iter()
                .filter(|(_, &v)| v == 2)
                .map(|(&k, _)| k)
                .collect();

            pairs.sort();

            return Rank {
                main: HandRank::TwoPair,
                major: pairs[1],
                minor: pairs[0],
                left: cards
                    .iter()
                    .filter(|card| ![pairs[1], pairs[0]].contains(&card.value))
                    .map(|card| card.value)
                    .collect(),
            };
        }

        if hm.values().any(|&v| v == 2) {
            let major = *hm.iter().max_by_key(|(_, v)| **v).unwrap().0;
            let minor = *hm.keys().filter(|&&key| key != major).max().unwrap();

            return Rank {
                main: HandRank::OnePair,
                major,
                minor,
                left: cards
                    .iter()
                    .filter(|card| ![major, minor].contains(&card.value))
                    .map(|card| card.value)
                    .collect(),
            };
        }

        Rank {
            main: HandRank::HighCard,
            major: cards[0].value,
            minor: cards[1].value,
            left: cards[2..].iter().map(|card| card.value).collect(),
        }
    }
}

/// Main
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() <= 1 {
        return hands.into();
    }

    let poker_hands: Vec<Hand> = hands
        .iter()
        .enumerate()
        .map(|(idx, &hand)| Hand::from(hand, idx))
        .collect();

    let highest = poker_hands
        .iter()
        .max_by_key(|hand| hand.rank.main)
        .unwrap()
        .rank
        .main;

    let mut winners: Vec<&Hand> = poker_hands
        .iter()
        .filter(|hand| hand.rank.main == highest)
        .collect();

    if winners.len() > 1 {
        let highest_major = winners
            .iter()
            .max_by_key(|hand| hand.rank.major)
            .unwrap()
            .rank
            .major;

        winners.retain(|hand| hand.rank.major == highest_major);
    }

    if winners.len() > 1 {
        let highest_minor = winners
            .iter()
            .max_by_key(|hand| hand.rank.minor)
            .unwrap()
            .rank
            .minor;

        winners.retain(|hand| hand.rank.minor == highest_minor);
    }

    if winners.len() > 1 {
        let count = winners[0].rank.left.len();

        for i in 0..count {
            let highest_low = winners
                .iter()
                .max_by_key(|hand| hand.rank.left[i])
                .unwrap()
                .rank
                .left[i];

            winners.retain(|hand| hand.rank.left[i] == highest_low);

            if winners.len() == 1 {
                break;
            }
        }
    }

    winners
        .iter()
        .map(|hand| *hands.get(hand.idx).unwrap())
        .collect()
}
