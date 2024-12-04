use itertools::Itertools;
use std::iter::{repeat, zip};

pub fn solve(input: &str) -> usize {
    const XMAS: &[char; 4] = &['X', 'M', 'A', 'S'];
    let grid: Vec<Vec<char>> = input
        .split_whitespace()
        .map(|s| s.chars().collect())
        .collect();
    println!("h {}", input.matches("XMAS").count() + input.matches("SAMX").count());
    println!("v {}", vert_matches(&grid, XMAS));
    println!("\\ {}", diag_matches(&grid, XMAS));
    println!("/ {}", diag2_matches(&grid, XMAS));
    input.matches("XMAS").count()
    + input.matches("SAMX").count()
    + vert_matches(&grid, XMAS)
    + diag_matches(&grid, XMAS)
    + diag2_matches(&grid, XMAS)
}

fn vert_matches(grid: &[Vec<char>], arg: &[char]) -> usize {
    type Word = (char, char, char, char);
    let word = (arg[0], arg[1], arg[2], arg[3]);
    let back = (arg[3], arg[2], arg[1], arg[0]);
    (0..grid[0].len())
        .map(|x| (0..grid.len()).map(move |y| grid[y][x]))
        .map(|i| {
            i.tuple_windows()
                .filter(|&t: &Word| t == word || t == back)
                .count()
        })
        .sum()
}

fn diag_matches(grid: &[Vec<char>], arg: &[char]) -> usize {
    type Word = (char, char, char, char);
    let word = (arg[0], arg[1], arg[2], arg[3]);
    let back = (arg[3], arg[2], arg[1], arg[0]);
    let x = (1..grid[0].len()).rev().chain(repeat(0).take(grid.len()));
    let y = (repeat(0).take(grid[0].len())).chain(1..grid.len());
    x.zip(y)
        .map(|(x, y)| zip(x..grid[0].len(), y..grid.len()))
        .map(|i| i.map(|(x, y)| grid[y][x]))
        .map(|i| {
            i.tuple_windows()
                .filter(|&t: &Word| t == word || t == back)
                .count()
        })
        .sum()
}

fn diag2_matches(grid: &[Vec<char>], arg: &[char]) -> usize {
    type Word = (char, char, char, char);
    let word = (arg[0], arg[1], arg[2], arg[3]);
    let back = (arg[3], arg[2], arg[1], arg[0]);
    let x = (0..grid[0].len()).chain(repeat(grid[0].len()).take(grid.len() - 1));
    let y = (repeat(0).take(grid[0].len())).chain(1..grid.len());
    x.zip(y)
        .map(|(x, y)| zip((0..x).rev(), y..grid.len()))
        .map(|i| i.map(|(x, y)| grid[y][x]))
        .map(|i| {
            i.tuple_windows()
                .filter(|&t: &Word| t == word || t == back)
                .count()
        })
        .sum()
}
