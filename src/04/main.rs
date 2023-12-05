mod part1;
mod part2;

use part1::part1;
use part2::part2;

fn main() {
    let input = include_str!("input");
    println!("{}", part1(input));
    println!("{}", part2(input));
}
