pub fn solve(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        sum += next_value(line.split_whitespace().map(|s| s.parse::<i64>().unwrap()));
    }
    sum
}

fn next_value(values: impl Iterator<Item = i64> + Clone) -> i64 {
    if values.clone().all(|n| n == 0) {
        return 0;
    }
    values.clone().last().unwrap() + next_value(values.map_windows(|[a, b]| b - a))
}
