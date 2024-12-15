mod part1;
mod part2;

fn call<const W: i32, const H: i32>(input: &str) {
    println!("Part 1: {}", part1::solve::<W, H>(input));
    println!("Part 2: {}", part2::solve::<W, H>(input));
}

fn main() {
    if std::env::args().len() > 1 {
        const INPUT: &str = include_str!("input_test");
        const W: i32 = 11;
        const H: i32 = 7;
        call::<W, H>(INPUT);
    } else {
        const INPUT: &str = include_str!("input");
        const W: i32 = 101;
        const H: i32 = 103;
        call::<W, H>(INPUT);
    };
}
