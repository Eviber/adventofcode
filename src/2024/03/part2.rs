use regex::Regex;

pub fn solve(input: &str) -> usize {
    // let do_groups = Regex::new(r"(?m)(?:^|do\(\)).*?(?:mul\((\d+),(\d+)\))+?.*?(?:don't\(\)|$)").unwrap();
    let do_groups = Regex::new(r"(?m)(?:^|do\(\))(.+?)(?:don't\(\)|$)").unwrap();
    let mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    println!("{}", do_groups.is_match(input));
    do_groups
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [g])| g)
        .flat_map(|group| {
            println!("{group}");
            mul.captures_iter(group)
                .map(|c| c.extract())
                .map(|(_, [a, b])| (a.parse().unwrap(), b.parse().unwrap()))
                .map(|(a, b): (usize, usize)| a * b)
        })
        .sum()
}
