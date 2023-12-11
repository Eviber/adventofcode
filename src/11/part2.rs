use std::{fmt::Display, str::FromStr};

pub fn solve(input: &str) -> usize {
    let map: Map = input.parse().unwrap();
    println!("{}", map);
    let positions = map.galaxies_positions();
    println!("{:?}", positions);
    let mut sum = 0;
    for i in 0..(positions.len() - 1) {
        for j in (i + 1)..positions.len() {
            sum += positions[i].distance(&positions[j]);
        }
    }
    sum
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn distance(&self, other: &Position) -> usize {
        let dx = self.x.max(other.x) - self.x.min(other.x);
        let dy = self.y.max(other.y) - self.y.min(other.y);
        dx + dy
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Empty,
    Galaxy,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Galaxy,
            _ => panic!("Unknown tile: {}", c),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Empty => '.',
            Tile::Galaxy => '#',
        };
        write!(f, "{}", c)
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    empty_columns: Vec<usize>,
    empty_rows: Vec<usize>,
}

impl Map {
    fn galaxies_positions(&self) -> Vec<Position> {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, tile)| **tile == Tile::Galaxy)
                    .map(move |(x, _)| {
                        let empty_columns = self.empty_columns.iter().filter(|&&c| c < x).count();
                        let empty_rows = self.empty_rows.iter().filter(|&&r| r < y).count();
                        let x = x + empty_columns * 999_999;
                        let y = y + empty_rows * 999_999;
                        Position { x, y }
                    })
            })
            .collect()
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<Tile>> = s
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        let mut empty_columns = Vec::new();
        for x in 0..tiles[0].len() {
            if tiles.iter().all(|row| row[x] == Tile::Empty) {
                empty_columns.push(x);
            }
        }
        let mut empty_rows = Vec::new();
        for (y, row) in tiles.iter().enumerate() {
            if row.iter().all(|tile| *tile == Tile::Empty) {
                empty_rows.push(y);
            }
        }
        Ok(Map { tiles, empty_columns, empty_rows })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  ")?;
        for x in 0..self.tiles[0].len() {
            if self.empty_columns.contains(&x) {
                write!(f, "v")?;
            } else {
                write!(f, " ")?;
            }
        }
        writeln!(f)?;
        for (y, row) in self.tiles.iter().enumerate() {
            if self.empty_rows.contains(&y) {
                write!(f, "> ")?;
            } else {
                write!(f, "  ")?;
            }
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
