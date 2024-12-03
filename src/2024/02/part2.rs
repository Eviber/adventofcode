use rayon::iter::ParallelIterator;
use std::cmp::Ordering;

use itertools::Itertools;
use rayon::str::ParallelString;

fn get_dir(report: &[i8]) -> Ordering {
    let report = report.iter().tuple_windows();
    let dir = report.fold(0, |acc, (a, b)| {
        acc + match a.cmp(b) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        }
    });
    dir.cmp(&0)
}

pub fn solve(input: &str) -> usize {
    input
        .par_lines()
        .filter(|line| {
            let report: Vec<_> = line
                .split_whitespace()
                .map(|s| s.parse::<i8>().unwrap())
                .collect();
            let dir = get_dir(&report);
            if dir == Ordering::Equal {
                return false;
            }
            (0..=report.len()).rev().any(|i| {
                report
                    .iter()
                    .enumerate()
                    .filter_map(|(j, n)| (i != j).then_some(n))
                    .tuple_windows()
                    .all(|(a, b)| a.cmp(b) == dir && a.abs_diff(*b) <= 3)
            })
        })
        .count()
}
