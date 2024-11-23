mod part1;
mod part2;

fn test(input: &str) {
    println!("Part 1: {}", part1::solve(input));
    println!("Part 2: {}", part2::solve(input));
    println!();
}

fn main() {
    let input = if std::env::args().len() > 1 {
        include_str!("input_test")
    } else {
        include_str!("input")
    };
    test(input);
    test("???.### 1,1,3");
    test(".??..??...?##. 1,1,3");
    test("?#?#?#?#?#?#?#? 1,3,1,6");
    test("????.#...#... 4,1,1");
    test("????.######..#####. 1,6,5");
    test("?###???????? 3,2,1");
}
