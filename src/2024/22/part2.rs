use std::collections::HashMap;

use itertools::Itertools;

use crate::secret::{int, Secret};

pub fn solve(input: &str) -> int {
    let prices: Vec<Vec<int>> = input
        .lines()
        .map(Secret::from)
        .map(|secret| secret.map(|s| s % 10).take(2000).collect::<Vec<_>>())
        .collect();
    let changes: Vec<Vec<int>> = prices
        .iter()
        .map(|v| v.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>())
        .collect();
    let mut sequences_value = HashMap::new();
    for (price, sequence) in prices
        .into_iter()
        .zip(changes.iter())
        .flat_map(|(prices, changes)| {
            prices
                .into_iter()
                .skip(4)
                .zip(changes.windows(4))
                .unique_by(|(_, seq)| *seq)
        })
    {
        sequences_value
            .entry(sequence)
            .and_modify(|p| *p += price)
            .or_insert(price);
    }
    sequences_value.into_values().max().unwrap()
}

#[test]
fn part2() {
    let input = "1
2
3
2024";
    assert_eq!(solve(input), 23);
}
