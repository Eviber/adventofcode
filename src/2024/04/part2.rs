use std::iter::repeat;

pub fn solve(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .split_whitespace()
        .map(|s| s.chars().collect())
        .collect();
    (1..(grid.len() - 1))
        .flat_map(|y| {
            (1..(grid.len() - 1))
                .zip(repeat(y))
                .filter(|&(x, y)| grid[y][x] == 'A')
                .filter(|(x, y)| valid_pair(grid[y - 1][x - 1], grid[y + 1][x + 1]))
                .filter(|(x, y)| valid_pair(grid[y - 1][x + 1], grid[y + 1][x - 1]))
        })
        .count()
}

#[inline]
fn valid_pair(a: char, b: char) -> bool {
    (a == 'M' && b == 'S') || (a == 'S' && b == 'M')
}
