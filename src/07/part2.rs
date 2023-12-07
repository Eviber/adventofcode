mod hand;

use hand::Hand;

pub fn solve(input: &str) -> u64 {
    let mut hands: Vec<Hand> = input.lines().map(|s| s.parse().unwrap()).collect();
    hands.sort_unstable();
    hands.iter().zip(1..).map(|(h, i)| h.bid * i).sum()
}
