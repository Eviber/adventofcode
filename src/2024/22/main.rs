mod part1;
mod part2;

pub mod secret;

fn main() {
    let input = include_str!("input");
    println!("Part 1: {}", part1::solve(input));
    println!("Part 2: {}", part2::solve(input));
}
