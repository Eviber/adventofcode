pub fn solve(input: &str) -> usize {
    let (patterns, target_patterns) = input.split_once("\n\n").expect("one empty line");
    let patterns: Vec<_> = patterns.split(", ").collect();

    target_patterns.lines().filter(|target| is_pattern_possible(target, &patterns)).count()
}

fn is_pattern_possible(target: &str, patterns: &[&str]) -> bool {
    let mut possible_until = vec![false; target.len() + 1];
    possible_until[0] = true;

    for i in 0..possible_until.len() {
        if !possible_until[i] {
            continue;
        }
        for pattern in patterns {
            if target[i..].starts_with(pattern) {
                possible_until[i + pattern.len()] = true;
            }
        }
    }
    possible_until[target.len()]
}
