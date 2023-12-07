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

impl Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandType::FiveOfAKind => write!(f, "Five of a kind"),
            HandType::FourOfAKind => write!(f, "Four of a kind"),
            HandType::FullHouse => write!(f, "Full house"),
            HandType::TreeOfAKind => write!(f, "Tree of a kind"),
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
