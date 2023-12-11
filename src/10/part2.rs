use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    str::FromStr,
};

pub fn solve(input: &str) -> u64 {
    let mut grid: Grid = input.parse().unwrap();
    let in_loop = grid.find_loop();
    println!("{}", grid);
    grid.filter(in_loop);
    println!("{}", grid);
    grid.scale_up();
    println!("{}", grid);
    grid.count_enclosed()
}

struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl Grid {
    fn get_start(&self) -> Option<(usize, usize)> {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x].is_start() {
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn find_loop(&self) -> HashSet<Position> {
        let (x, y) = self.get_start().unwrap();
        let mut visited: HashSet<Position> = HashSet::new();
        visited.insert(Position { x, y });
        let mut queue: VecDeque<(Position, Direction)> = VecDeque::new();
        for dir in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let Some((x, y)) = dir.position(x, y) else {
                continue;
            };
            if self.grid[y][x].has_direction(dir.opposite()) {
                queue.push_back((Position { x, y }, *dir));
            }
        }
        while let Some((pos, dir)) = queue.pop_front() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);
            let dir = self.grid[pos.y][pos.x].out_direction(dir).unwrap();
            let (x, y) = dir.position(pos.x, pos.y).unwrap();
            if self.grid.get(y).is_none() || self.grid[y].get(x).is_none() {
                unreachable!();
            }
            queue.push_back((Position { x, y }, dir));
        }
        visited
    }

    fn filter(&mut self, to_keep: HashSet<Position>) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if !to_keep.contains(&Position { x, y }) {
                    self.grid[y][x] = Tile::Empty;
                }
            }
        }
    }

    fn scale_up(&mut self) {
        use Direction::*;
        let mut new_grid: Vec<Vec<Tile>> = Vec::new();
        for y in 0..self.grid.len() {
            let mut new_row: Vec<Tile> = Vec::new();
            new_row.push(self.grid[y][0]);
            for x in 1..self.grid[y].len() {
                if self.grid[y][x].has_direction(Left) && self.grid[y][x - 1].has_direction(Right) {
                    new_row.push('-'.try_into().unwrap());
                } else {
                    new_row.push(Tile::Empty);
                }
                new_row.push(self.grid[y][x]);
            }
            new_grid.push(new_row);
        }
        self.grid = new_grid;
        let mut new_grid: Vec<Vec<Tile>> = Vec::new();
        new_grid.push(self.grid[0].clone());
        for y in 1..self.grid.len() {
            let mut new_row: Vec<Tile> = Vec::new();
            for x in 0..self.grid[y].len() {
                if self.grid[y][x].has_direction(Up) && self.grid[y - 1][x].has_direction(Down) {
                    new_row.push('|'.try_into().unwrap());
                } else {
                    new_row.push(Tile::Empty);
                }
            }
            new_grid.push(new_row);
            new_grid.push(self.grid[y].clone());
        }
        self.grid = new_grid;
    }

    fn count_enclosed(&self) -> u64 {
        // Mark all empty tiles reachable from an edge
        // Then count all empty tiles not marked
        // This is the number of enclosed tiles
        //
        // First, find any empty tile on an edge
        let start = find_edges(&self.grid);
        let mut visited: HashSet<Position> = HashSet::new();
        let mut queue: VecDeque<Position> = VecDeque::new();
        for pos in start {
            queue.push_back(pos);
        }
        while let Some(pos) = queue.pop_front() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);
            for dir in &[
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let Some((x, y)) = dir.position(pos.x, pos.y) else {
                    continue;
                };
                if self.grid.get(y).is_none() || self.grid[y].get(x).is_none() {
                    continue;
                }
                if self.grid[y][x] == Tile::Empty {
                    queue.push_back(Position { x, y });
                }
            }
        }
        let mut count = 0;
        // Note: skip every other tile to account for scaling
        for y in (0..self.grid.len()).step_by(2) {
            for x in (0..self.grid[y].len()).step_by(2) {
                if self.grid[y][x] == Tile::Empty && !visited.contains(&Position { x, y }) {
                    count += 1;
                    print!("I");
                } else {
                    print!("{}", self.grid[y][x]);
                }
            }
            println!();
        }
        println!();
        count
    }
}

fn find_edges(grid: &Vec<Vec<Tile>>) -> Vec<Position> {
    let mut edges: Vec<Position> = Vec::new();
    for y in 0..grid.len() {
        if grid[y][0] == Tile::Empty {
            edges.push(Position { x: 0, y });
        }
        if grid[y][grid[y].len() - 1] == Tile::Empty {
            edges.push(Position {
                x: grid[y].len() - 1,
                y,
            });
        }
    }
    for x in 0..grid[0].len() {
        if grid[0][x] == Tile::Empty {
            edges.push(Position { x, y: 0 });
        }
        if grid[grid.len() - 1][x] == Tile::Empty {
            edges.push(Position {
                x,
                y: grid.len() - 1,
            });
        }
    }
    edges
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<Tile>> = s
            .lines()
            .map(|l| l.chars().map(|c| Tile::try_from(c).unwrap()).collect())
            .collect();
        Ok(Grid { grid })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn position(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        let x = match self {
            Direction::Left => x.checked_sub(1)?,
            Direction::Right => x + 1,
            _ => x,
        };
        let y = match self {
            Direction::Up => y.checked_sub(1)?,
            Direction::Down => y + 1,
            _ => y,
        };
        Some((x, y))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Pipe(Direction, Direction),
    Start,
}

impl Tile {
    fn out_direction(self, in_dir: Direction) -> Option<Direction> {
        match self {
            Tile::Empty | Tile::Start => None,
            Tile::Pipe(d1, d2) => {
                if d1 != in_dir.opposite() {
                    Some(d1)
                } else {
                    Some(d2)
                }
            }
        }
    }

    fn has_direction(self, dir: Direction) -> bool {
        match self {
            Tile::Empty => false,
            Tile::Start => true,
            Tile::Pipe(d1, d2) => d1 == dir || d2 == dir,
        }
    }

    #[inline]
    fn is_start(self) -> bool {
        self == Tile::Start
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '.' => Ok(Tile::Empty),
            '|' => Ok(Tile::Pipe(Direction::Up, Direction::Down)),
            '-' => Ok(Tile::Pipe(Direction::Left, Direction::Right)),
            'L' => Ok(Tile::Pipe(Direction::Right, Direction::Up)),
            'J' => Ok(Tile::Pipe(Direction::Left, Direction::Up)),
            '7' => Ok(Tile::Pipe(Direction::Left, Direction::Down)),
            'F' => Ok(Tile::Pipe(Direction::Right, Direction::Down)),
            'S' => Ok(Tile::Start),
            _ => Err(()),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Pipe(Direction::Up, Direction::Down) => write!(f, "|"),
            Tile::Pipe(Direction::Left, Direction::Right) => write!(f, "-"),
            Tile::Pipe(Direction::Right, Direction::Up) => write!(f, "L"),
            Tile::Pipe(Direction::Left, Direction::Up) => write!(f, "J"),
            Tile::Pipe(Direction::Left, Direction::Down) => write!(f, "7"),
            Tile::Pipe(Direction::Right, Direction::Down) => write!(f, "F"),
            Tile::Start => write!(f, "S"),
            _ => unreachable!(),
        }
    }
}
