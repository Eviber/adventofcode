use std::fmt::Display;

pub fn solve(input: &str) -> usize {
    let mut grid = vec![vec![]; input.lines().next().unwrap().len()];
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            grid[i].push(Tile::from(c));
        }
    }
    grid.iter_mut().for_each(|column| incline(column));
    for y in 0..grid[0].len() {
        for row in &grid {
            print!("{}", row[y]);
        }
        println!();
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

fn incline(column: &mut [Tile]) {
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

#[derive(Clone, Copy, PartialEq, Eq)]
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
