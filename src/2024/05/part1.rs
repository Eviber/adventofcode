use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let (rules_str, updates) = input.split_once("\n\n").expect("correct format");
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    for s in rules_str.split_whitespace() {
        let (s1, s2) = s.split_once('|').unwrap();
        let page_1: usize = s1.parse().unwrap();
        let page_2: usize = s2.parse().unwrap();
        rules
            .entry(page_1)
            .and_modify(|v| v.push(page_2))
            .or_insert(vec![page_2]);
    }
    updates
        .split_whitespace()
        .map(|s| s.split(',').map(|s| s.parse().unwrap()).collect::<Vec<_>>())
        .filter(|update| is_valid(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn is_valid(update: &[usize], rules: &HashMap<usize, Vec<usize>>) -> bool {
    update.iter().enumerate().all(|(i, page)| {
        update
            .iter()
            .take(i)
            .all(|n| rules.get(page).map(|v| !v.contains(n)).unwrap_or(true))
    })
}
