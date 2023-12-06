use std::collections::VecDeque;

pub fn part2(input: &str) -> u32 {
    let mut bonus_copies = VecDeque::new();
    let mut count = 0;
    for line in input.lines() {
        let copies = bonus_copies.pop_front().unwrap_or(0) + 1;
        count += copies;
        let matches = matching_numbers(line);
        for i in 0..matches as usize {
            if let Some(amount) = bonus_copies.get_mut(i) {
                *amount += copies;
            } else {
                bonus_copies.push_back(copies);
            }
        }
    }
    count
}

fn matching_numbers(line: &str) -> u32 {
    let begin = line.chars().position(|c| c == ':').unwrap() + 2;
    let mut line = line[begin..].split('|');
    let winning_numbers: Vec<u32> = line
        .next()
        .unwrap()
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();
    line.next()
        .unwrap()
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .filter(|n| winning_numbers.contains(n))
        .count() as u32
}

#[test]
fn test() {
    let input = include_str!("input_test");
    assert_eq!(part2(input), 30);
}
