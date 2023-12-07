use std::fmt::{Debug, Display};
use std::str::FromStr;
use std::{cmp::Ordering, num::ParseIntError};

mod card;
mod hand_type;

use card::{Card, CardError};
use hand_type::HandType;

#[derive(PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    pub bid: u64,
    hand_type: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
                    match card1.cmp(card2) {
                        Ordering::Equal => (),
                        o => return o,
                    }
                }
                Ordering::Equal
            }
            o => o,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in self.cards.iter() {
            write!(f, "{}", card)?;
        }
        write!(f, " {:5}", self.bid)?;
        write!(f, " - {}", self.hand_type)
    }
}

impl FromStr for Hand {
    type Err = HandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let cards: [Card; 5] = parts
            .next()
            .ok_or(HandError::MissingHand)?
            .chars()
            .map(Card::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| HandError::WrongNumberOfCards)?;
        let bid = parts.next().ok_or(HandError::MissingBid)?.parse::<u64>()?;
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
                2 => pairs += 1,
                3 => three_of_a_kind = true,
                4 => four_of_a_kind = true,
                5 => five_of_a_kind = true,
                _ => (),
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
        Ok(Hand {
            cards,
            bid,
            hand_type,
        })
    }
}

pub enum HandError {
    MissingHand,
    MissingBid,
    WrongNumberOfCards,
    CardError(CardError),
    ParseIntError(ParseIntError),
}

impl From<CardError> for HandError {
    fn from(e: CardError) -> Self {
        HandError::CardError(e)
    }
}

impl From<ParseIntError> for HandError {
    fn from(e: ParseIntError) -> Self {
        HandError::ParseIntError(e)
    }
}

impl Debug for HandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandError::MissingHand => write!(f, "Missing hand"),
            HandError::MissingBid => write!(f, "Missing bid"),
            HandError::WrongNumberOfCards => write!(f, "Wrong number of cards"),
            HandError::CardError(e) => write!(f, "{:?}", e),
            HandError::ParseIntError(e) => write!(f, "{:?}", e),
        }
    }
}
