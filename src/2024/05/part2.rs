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
    let updates: Vec<Vec<usize>> = updates
        .split_whitespace()
        .map(|s| s.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();
    updates
        .into_iter()
        .filter(|update| {
            update.iter().enumerate().any(|(i, page)| {
                update
                    .iter()
                    .take(i)
                    .any(|n| rules.get(page).map(|v| {
                        v.contains(n)
                    }).unwrap_or(false))
            })
        })
        .map(|mut update| {
            let mut corrected = vec![];
            while !update.is_empty() {
                let i = update
                    .iter()
                    .position(|page| {
                        update
                            .iter()
                            .all(|n| !rules.get(n).map(|v| v.contains(page)).unwrap_or(false))
                    })
                    .expect("one value should be free to go first");
                corrected.push(update.remove(i))
            }
            // no need to reverse, we only need the middle value from this point
            // corrected.into_iter().rev().collect()
            corrected
        })
        .map(|update| update[update.len() / 2])
        .sum()
}
