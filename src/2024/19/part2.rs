pub fn solve(input: &str) -> usize {
    let (patterns, target_patterns) = input.split_once("\n\n").expect("one empty line");
    let patterns: Vec<_> = patterns.split(", ").collect();

    target_patterns
        .lines()
        .map(|target| count_pattern_comb(target, &patterns))
        .sum()
}

fn count_pattern_comb(target: &str, patterns: &[&str]) -> usize {
    let mut possible_count = vec![0; target.len() + 1];
    possible_count[0] = 1;

    for i in 0..possible_count.len() {
        if possible_count[i] == 0 {
            continue;
        }
        for pattern in patterns {
            if target[i..].starts_with(pattern) {
                possible_count[i + pattern.len()] += possible_count[i];
            }
        }
    }
    possible_count[target.len()]
}
