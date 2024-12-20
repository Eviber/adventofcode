mod part2;

fn main() {
    let (input, threshold, threshold_p2) = if std::env::args().len() > 1 {
        (include_str!("input_test"), 1, 50)
    } else {
        (include_str!("input"), 100, 100)
    };
    println!("Part 1: {}", part2::solve(input, 2, threshold));
    println!("Part 2: {}", part2::solve(input, 20, threshold_p2));
}
