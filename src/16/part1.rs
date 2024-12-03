// .|...\....
// |.-.\.....
// .....|-...
// ........|.
// ..........
// .........\
// ..../.\\..
// .-.-/..|..
// .|....-|.\
// ..//.|....
pub fn solve(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input
        .split_whitespace()
        .map(|s| s.chars().map(Cell::from).collect())
        .collect();
    let energized = vec![vec![false; grid[0].len()]; grid.len()];

    todo!()
}

use Cell::*;

enum Cell {
    Empty,
    MirrorDescending,
    MirrorAscending,
    HorizSplitter,
    VertiSplitter,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Empty,
            '\\' => MirrorDescending,
            '/' => MirrorAscending,
            '-' => HorizSplitter,
            '|' => VertiSplitter,
            _ => panic!("invalid character"),
        }
    }
}
