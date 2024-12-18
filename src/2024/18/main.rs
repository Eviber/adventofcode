mod part1;

fn main() {
    let (input, len) = if std::env::args().len() > 1 {
        (include_str!("input_test"), 7)
    } else {
        (include_str!("input"), 71)
    };
    println!("Part 1: {}", part1::solve(input, len));
    println!();
}
