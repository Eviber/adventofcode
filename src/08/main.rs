#![feature(test)]

mod part1;
mod part2;

mod math;

fn main() {
    let input = include_str!("input");
    println!("Part 1: {}", part1::solve(input));
    println!();
    println!("Part 2: {}", part2::solve(input));
}

extern crate test;

#[bench]
fn bench_part2(b: &mut test::Bencher) {
    let input = include_str!("input");
    let (map, pos, instructions) = part2::parse_input(input);
    let mut i = 0;
    for _ in 0..100 {
        i += part2::inner(&map, &pos, &instructions);
    }
    println!("{}", i);
    b.iter(|| part2::inner(&map, &pos, &instructions));
}

#[bench]
fn bench_part2_2(b: &mut test::Bencher) {
    let input = include_str!("input");
    let (map, pos, instructions) = part2::parse_input(input);
    let mut i = 0;
    for _ in 0..100 {
        i += part2::inner2(&map, &pos, &instructions);
    }
    println!("{}", i);
    b.iter(|| part2::inner2(&map, &pos, &instructions));
}

#[test]
fn test_part1() {
    let input = include_str!("input_test");
    assert_eq!(part1::solve(input), 2);
}

#[test]
fn test2_part1() {
    let input = include_str!("input_test_2");
    assert_eq!(part1::solve(input), 6);
}

#[test]
fn test_part2() {
    let input = include_str!("input_test_part2");
    assert_eq!(part2::solve(input), 6);
}
