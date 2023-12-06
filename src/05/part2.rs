use crate::rule::Range;
use crate::rule::Rule;

pub fn part2(input: &str) -> u64 {
    let mut parts = input.split("\n\n");
    let mut values = get_seeds(parts.next().unwrap());
    for part in parts {
        let mut next_values = Vec::new();
        let mut leftovers = Vec::new();
        let rules = get_rules(part);
        for rule in rules {
            println!("\nrule: {:?}", rule);
            let mut to_remove = Vec::new();
            for (i, &value) in values.iter().enumerate() {
                let (before, matched, after) = rule.apply_range(value);
                println!("{} -> {} => {:?}, {:?}, {:?}", value.start, value.end(), before, matched, after);
                if let Some(before) = before {
                    leftovers.push(before);
                }
                if let Some(after) = after {
                    leftovers.push(after);
                }
                if let Some(matched) = matched {
                    next_values.push(matched);
                    to_remove.push(i);
                }
            }
            for i in to_remove.into_iter().rev() {
                values.remove(i);
            }
            values.append(&mut leftovers);
        }
        values.append(&mut next_values);
        for value in &values {
            print!("{} -> {}, ", value.start, value.end());
        }
        println!();
        println!();
    }
    values.iter().map(|&range| range.start).min().unwrap()
}

fn get_seeds(line: &str) -> Vec<Range> {
    let line = &line[line.find(' ').unwrap() + 1..];
    line.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|chunk| Range {
            start: chunk[0],
            length: chunk[1],
        })
        .collect()
}

fn get_rules(part: &str) -> impl Iterator<Item = Rule> + '_ {
    part.lines()
        .skip(1)
        .map(|line| line.parse().unwrap())
}
