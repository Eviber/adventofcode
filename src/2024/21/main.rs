mod part1;

fn main() {
    let input = include_str!("input");
    println!("Part 1: {}", part1::solve(input));
}

#[test]
fn part1_029() {
    assert_eq!(part1::solve("029A"), 68 * 29);
}

#[test]
fn part1_980() {
    assert_eq!(part1::solve("980A"), 60 * 980);
}

#[test]
fn part1_179() {
    assert_eq!(part1::solve("179A"), 68 * 179);
}

#[test]
fn part1_456() {
    assert_eq!(part1::solve("456A"), 64 * 456);
}

#[test]
fn part1_379() {
    assert_eq!(part1::solve("379A"), 64 * 379);
}

#[test]
fn part1() {
    let input = "029A
980A
179A
456A
379A
";
    assert_eq!(part1::solve(input), 126384);
}
