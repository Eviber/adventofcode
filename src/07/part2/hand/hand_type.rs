use super::card::Card;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq, Eq)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    TreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

fn from_cards_inner(cards: &[Card]) -> HandType {
    let mut hand_type = HandType::HighCard;
    let mut counts = [0; 13];
    for card in cards.iter() {
        match card {
            Card::Ace => counts[0] += 1,
            Card::Number(n) => counts[*n as usize - 1] += 1,
            Card::Jack => counts[10] += 1,
            Card::Queen => counts[11] += 1,
            Card::King => counts[12] += 1,
        }
    }
    let mut pairs = 0;
    let mut three_of_a_kind = false;
    let mut four_of_a_kind = false;
    let mut five_of_a_kind = false;
    for count in counts.iter() {
        match count {
            0 | 1 => (),
            2 => pairs += 1,
            3 => three_of_a_kind = true,
            4 => four_of_a_kind = true,
            5 => five_of_a_kind = true,
            _ => unreachable!(),
        }
    }
    if five_of_a_kind {
        hand_type = HandType::FiveOfAKind;
    } else if four_of_a_kind {
        hand_type = HandType::FourOfAKind;
    } else if three_of_a_kind && pairs == 1 {
        hand_type = HandType::FullHouse;
    } else if three_of_a_kind {
        hand_type = HandType::TreeOfAKind;
    } else if pairs == 2 {
        hand_type = HandType::TwoPairs;
    } else if pairs == 1 {
        hand_type = HandType::OnePair;
    }
    hand_type
}

impl HandType {
    pub fn from_cards(cards: &[Card; 5]) -> HandType {
        let mut hand_type = from_cards_inner(cards);
        if cards.iter().any(|&c| c == Card::Jack) {
            let mut types = cards.to_vec();
            types.sort_unstable();
            types.dedup();
            types.reverse();
            for card_type in types.iter().filter(|&&c| c != Card::Jack) {
                let cards = cards
                    .iter()
                    .map(|&c| if c == Card::Jack { *card_type } else { c })
                    .collect::<Vec<_>>();
                let hand_type_candidate = from_cards_inner(cards.as_slice());
                if hand_type_candidate > hand_type {
                    hand_type = hand_type_candidate;
                }
            }
        }
        hand_type
    }
}

impl Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandType::FiveOfAKind => write!(f, "Five of a kind"),
            HandType::FourOfAKind => write!(f, "Four of a kind"),
            HandType::FullHouse => write!(f, "Full house"),
            HandType::TreeOfAKind => write!(f, "Three of a kind"),
            HandType::TwoPairs => write!(f, "Two pairs"),
            HandType::OnePair => write!(f, "One pair"),
            HandType::HighCard => write!(f, "High card"),
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        match (self, other) {
            (HandType::FiveOfAKind, _) => Ordering::Greater,
            (_, HandType::FiveOfAKind) => Ordering::Less,
            (HandType::FourOfAKind, _) => Ordering::Greater,
            (_, HandType::FourOfAKind) => Ordering::Less,
            (HandType::FullHouse, _) => Ordering::Greater,
            (_, HandType::FullHouse) => Ordering::Less,
            (HandType::TreeOfAKind, _) => Ordering::Greater,
            (_, HandType::TreeOfAKind) => Ordering::Less,
            (HandType::TwoPairs, _) => Ordering::Greater,
            (_, HandType::TwoPairs) => Ordering::Less,
            (HandType::OnePair, _) => Ordering::Greater,
            (_, HandType::OnePair) => Ordering::Less,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
