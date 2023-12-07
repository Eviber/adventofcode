use std::fmt::{Debug, Display};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Card {
    Ace,
    Jack,
    Queen,
    King,
    Number(u8),
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Ace => write!(f, "A"),
            Card::Jack => write!(f, "J"),
            Card::Queen => write!(f, "Q"),
            Card::King => write!(f, "K"),
            Card::Number(n) if *n == 10 => write!(f, "T"),
            Card::Number(n) => write!(f, "{}", n),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        match (self, other) {
            (Card::Ace, _) => Ordering::Greater,
            (_, Card::Ace) => Ordering::Less,
            (Card::King, _) => Ordering::Greater,
            (_, Card::King) => Ordering::Less,
            (Card::Queen, _) => Ordering::Greater,
            (_, Card::Queen) => Ordering::Less,
            (Card::Jack, _) => Ordering::Greater,
            (_, Card::Jack) => Ordering::Less,
            (Card::Number(a), Card::Number(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<char> for Card {
    type Error = CardError;

    fn try_from(c: char) -> Result<Self, CardError> {
        match c {
            'A' => Ok(Card::Ace),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'T' => Ok(Card::Number(10)),
            c => {
                let n = c.to_digit(10).ok_or(CardError(c))? as u8;
                if (2..=9).contains(&n) {
                    Ok(Card::Number(n))
                } else {
                    Err(CardError(c))
                }
            }
        }
    }
}

pub struct CardError(char);

impl Debug for CardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\'{}\' is not a valid card", self.0)
    }
}
