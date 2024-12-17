mod part1;

fn main() {
    let input = if std::env::args().len() > 1 {
        include_str!("input_test")
    } else {
        include_str!("input")
    };
    let output = part1::solve(input);
    println!("Part 1: {:?}", output);
    println!();
}
