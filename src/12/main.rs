use std::time::Instant;

mod part1;
mod part2;

fn main() {
    let input = if std::env::args().len() > 1 {
        include_str!("input_test")
    } else {
        include_str!("input")
    };
    let t = Instant::now();
    let res = part1::solve(input);
    println!("Elapsed: {}", t.elapsed().as_millis());
    println!("Part 1: {}", res);
    println!();
    let t = Instant::now();
    let res = part2::solve(input);
    println!("Elapsed: {}", t.elapsed().as_millis());
    println!("Part 2: {}", res);
}
