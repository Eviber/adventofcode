use regex::Regex;

pub fn solve(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| (a.parse().unwrap(), b.parse().unwrap()))
        .map(|(a, b): (usize, usize)| a * b)
        .sum()
}
