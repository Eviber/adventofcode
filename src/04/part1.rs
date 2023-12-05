pub fn part1(input: &str) -> u32 {
    input.lines().map(points).sum()
}

fn points(line: &str) -> u32 {
    let begin = line.chars().position(|c| c == ':').unwrap() + 2;
    let mut line = line[begin..].split('|');
    let winning_numbers: Vec<u32> = line
        .next()
        .unwrap()
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();
    let count = line
        .next()
        .unwrap()
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .filter(|n| winning_numbers.contains(n))
        .count();
    if count == 0 {
        return 0;
    }
    1 << (count - 1)
}

#[test]
fn test() {
    let input = include_str!("input_test");
    assert_eq!(part1(input), 13);
}
