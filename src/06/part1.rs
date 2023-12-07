pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let times = parse_line(lines.next().unwrap());
    let distances = parse_line(lines.next().unwrap());
    let mut product = 1;
    for (time, record) in times.zip(distances) {
        let possible_winning_hold_times = (0..time)
            .filter(|&i| record < distance_from_hold(i, time))
            .count();
        println!("{:3}ms, {:4}mm => {}", time, record, possible_winning_hold_times);
        product *= possible_winning_hold_times;
    }
    product
}

fn parse_line(line: &str) -> impl Iterator<Item = u32> + '_ {
    line.split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
}

fn distance_from_hold(hold_time: u32, race_duration: u32) -> u32 {
    let speed = hold_time;
    speed * (race_duration - hold_time)
}
