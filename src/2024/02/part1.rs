use itertools::Itertools;

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u8>().unwrap())
                .tuple_windows()
                .all(|(a, b, c)| a.cmp(&b) == b.cmp(&c) && a.abs_diff(b) <= 3 && b.abs_diff(c) <= 3)
        })
        .count()
}
