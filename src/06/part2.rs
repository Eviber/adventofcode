pub fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let time = parse_line(lines.next().unwrap());
    let record = parse_line(lines.next().unwrap());
    let winning_hold_times_amount = (0..time)
        .filter(|&i| record < distance_from_hold(i, time))
        .count();
    println!(
        "{:3}ms, {:4}mm => {}",
        time, record, winning_hold_times_amount
    );
    winning_hold_times_amount
}

fn parse_line(line: &str) -> usize {
    line.split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap()
}

fn distance_from_hold(hold_time: usize, race_duration: usize) -> usize {
    let speed = hold_time;
    speed * (race_duration - hold_time)
}
