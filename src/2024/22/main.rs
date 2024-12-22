mod part1;

mod secret;

fn main() {
    let input = include_str!("input");
    println!("Part 1: {}", part1::solve(input));
}
