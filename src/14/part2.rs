use std::{collections::HashMap, fmt::Display};

pub fn solve(input: &str) -> usize {
    let mut grid = vec![vec![]; input.lines().next().unwrap().len()];
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            grid[i].push(Tile::from(c));
        }
    }
    let mut states = HashMap::new();
    let mut i = 0;
    loop {
        if states.contains_key(&grid) {
            break;
        }
        states.insert(grid.clone(), i);
        rotate(&mut grid);
        i += 1;
    }
    let offset = states.get(&grid).expect("State should exist");
    let cycle = i - offset;
    i = ((1_000_000_000 - offset) / cycle) * cycle + offset;
    for _ in i..1_000_000_000 {
        rotate(&mut grid);
    }
    grid.iter()
        .map(|col| {
            col.iter()
                .rev()
                .enumerate()
                .filter(|(_, t)| **t == Tile::Round)
                .map(|(i, _)| i + 1)
                .sum::<usize>()
        })
        .sum()
}

fn rotate(grid: &mut [Vec<Tile>]) {
    incline_north(grid);
    incline_west(grid);
    incline_south(grid);
    incline_east(grid);
}

fn incline_north(grid: &mut [Vec<Tile>]) {
    for column in grid {
        let mut block_at = 0;
        for i in 0..column.len() {
            if column[i] == Tile::Cube {
                block_at = i + 1;
            } else if column[i] == Tile::Round {
                column[i] = Tile::Empty;
                column[block_at] = Tile::Round;
                block_at += 1;
            }
        }
    }
}

fn incline_south(grid: &mut [Vec<Tile>]) {
    for column in grid {
        let mut block_at = column.len() - 1;
        for i in (0..column.len()).rev() {
            if i > 0 && column[i] == Tile::Cube {
                block_at = i - 1;
            } else if column[i] == Tile::Round {
                column[i] = Tile::Empty;
                column[block_at] = Tile::Round;
                block_at = block_at.saturating_sub(1);
            }
        }
    }
}

fn incline_west(grid: &mut [Vec<Tile>]) {
    for y in 0..grid[0].len() {
        let mut block_at = 0;
        for x in 0..grid.len() {
            if grid[x][y] == Tile::Cube {
                block_at = x + 1;
            } else if grid[x][y] == Tile::Round {
                grid[x][y] = Tile::Empty;
                grid[block_at][y] = Tile::Round;
                block_at += 1;
            }
        }
    }
}

fn incline_east(grid: &mut [Vec<Tile>]) {
    for y in 0..grid[0].len() {
        let mut block_at = grid.len() - 1;
        for x in (0..grid.len()).rev() {
            if x > 0 && grid[x][y] == Tile::Cube {
                block_at = x - 1;
            } else if grid[x][y] == Tile::Round {
                grid[x][y] = Tile::Empty;
                grid[block_at][y] = Tile::Round;
                block_at = block_at.saturating_sub(1);
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Round,
    Cube,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            'O' => Tile::Round,
            '#' => Tile::Cube,
            _ => panic!("Unknown tile: {}", c),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Empty => '.',
            Tile::Round => 'O',
            Tile::Cube => '#',
        };
        write!(f, "{}", c)
    }
}
