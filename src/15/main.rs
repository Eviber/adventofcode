mod part1;
mod part2;

fn main() {
    let input = if std::env::args().len() > 1 {
        include_str!("input_test")
    } else {
        include_str!("input")
    };
    println!("Part 1: {}", part1::solve(input));
    println!();
    println!("Part 2: {}", part2::solve(input));
}
