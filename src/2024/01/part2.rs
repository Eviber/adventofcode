use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let mut occurences: HashMap<&str, usize> = HashMap::new();
    for s in input.split_whitespace().skip(1).step_by(2) {
        occurences.entry(s).and_modify(|n| *n += 1).or_insert(1);
    }
    input
        .split_whitespace()
        .step_by(2)
        .map(|s| (s, s.parse::<usize>().expect("correctly formatted number")))
        .map(|(s, n)| n * occurences.get(s).unwrap_or(&0))
        .sum()
}
