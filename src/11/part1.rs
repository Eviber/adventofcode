use std::{fmt::Display, str::FromStr};

pub fn solve(input: &str) -> u64 {
    let mut map: Map = input.parse().unwrap();
    println!("{}", map);
    map.expand();
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
    fn distance(&self, other: &Position) -> u64 {
        let dx = self.x as i64 - other.x as i64;
        let dy = self.y as i64 - other.y as i64;
        dx.unsigned_abs() + dy.unsigned_abs()
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
}

impl Map {
    fn expand(&mut self) {
        let mut new_tiles = Vec::with_capacity(self.tiles.len() * 2);
        let mut empty_columns = vec![false; self.tiles[0].len()];
        let mut empty_columns_count = 0;
        for x in 0..self.tiles[0].len() {
            empty_columns[x] = self.tiles.iter().all(|row| row[x] == Tile::Empty);
            if empty_columns[x] {
                empty_columns_count += 1;
            }
        }
        for row in &self.tiles {
            let mut new_row = Vec::with_capacity(row.len() + empty_columns_count);
            let mut empty_row = true;
            for (x, tile) in row.iter().enumerate() {
                new_row.push(*tile);
                if empty_columns[x] {
                    new_row.push(Tile::Empty);
                }
                if *tile != Tile::Empty {
                    empty_row = false;
                }
            }
            if empty_row {
                new_tiles.push(vec![Tile::Empty; new_row.len()]);
            }
            new_tiles.push(new_row);
        }
        self.tiles = new_tiles;
    }

    fn galaxies_positions(&self) -> Vec<Position> {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, tile)| **tile == Tile::Galaxy)
                    .map(move |(x, _)| Position { x, y })
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
        Ok(Map { tiles })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
