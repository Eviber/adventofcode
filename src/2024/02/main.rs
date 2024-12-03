use std::hint::black_box;
use std::time::Instant;

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
    let start = Instant::now();
    let res = part2::solve(input);
    for _ in 0..1000 {
        black_box(part2::solve(input));
    }
    println!("Part 2: {} - {}ms", res, start.elapsed().as_millis());
    println!();
}
