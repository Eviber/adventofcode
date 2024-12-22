use crate::secret::Secret;

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(Secret::from)
        .map(|mut secret| secret.nth(1999).unwrap())
        .sum()
}

#[test]
fn part1() {
    let input = "1
10
100
2024";
    assert_eq!(solve(input), 37327623);
}

#[test]
fn part1_1() {
    let input = "1";
    assert_eq!(solve(input), 8685429);
}

#[test]
fn part1_10() {
    let input = "10";
    assert_eq!(solve(input), 4700978);
}

#[test]
fn part1_100() {
    let input = "100";
    assert_eq!(solve(input), 15273692);
}

#[test]
fn part1_2024() {
    let input = "2024";
    assert_eq!(solve(input), 8667524);
}
