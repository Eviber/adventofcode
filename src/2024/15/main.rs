mod part1;
mod part2;

fn run(input: &str) {
    println!("Part 1: {}", part1::solve(input));
    println!();
    println!("Part 2: {}", part2::solve(input));
    println!();
}

fn main() {
    if std::env::args().len() > 1 {
        run(include_str!("input_test"));
        run(include_str!("input_test_2"));
    } else {
        run(include_str!("input"));
    }
}
