use itertools::Itertools;

pub fn solve(input: &str) -> usize {
    const XMAS: &[char; 4] = &['X', 'M', 'A', 'S'];
    const SAMX: &[char; 4] = &['S', 'A', 'M', 'X'];
    let grid: Vec<Vec<char>> = input
        .split_whitespace()
        .map(|s| s.chars().collect())
        .collect();
    input.matches("XMAS").count()
        + input.matches("SAMX").count()
        + vert_matches(&grid, XMAS)
        + vert_matches(&grid, SAMX)
        + diag_matches(&grid, XMAS)
        + diag_matches(&grid, SAMX)
}

fn vert_matches(grid: &[Vec<char>], arg: &[char]) -> usize {
    type Word = (char, char, char, char);
    let word = (arg[0], arg[1], arg[2], arg[3]);
    (0..grid[0].len())
        .map(|x| (0..grid.len()).map(move |y| grid[y][x]))
        .map(|i| i.tuple_windows().filter(|&t: &Word| t == word).count())
        .sum()
}

fn diag_matches(grid: &[Vec<char>], arg: &str) -> usize {
    todo!()
}
