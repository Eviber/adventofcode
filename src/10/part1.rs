use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) -> u64 {
    let mut debug_grid = input.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| l.chars().map(|c| Tile::try_from(c).unwrap()).collect())
        .collect();
    let (x, y) = grid
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter()
                .enumerate()
                .find(|(_, c)| c.is_start())
                .map(|(x, _)| (x, y))
        })
        .unwrap();
    let mut max_weight = 0;
    let mut visited: HashSet<Position> = HashSet::new();
    let mut queue: VecDeque<(Position, Direction, u64)> = VecDeque::new();
    for dir in &[
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let Some((x, y)) = dir.position(x, y) else {
            continue;
        };
        if grid[y][x].has_direction(dir.opposite()) {
            queue.push_back((Position { x, y }, *dir, 1));
        }
    }
    while let Some((pos, dir, weight)) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        debug_grid[pos.y].replace_range(pos.x..=pos.x, &format!("{:x}", weight));
        if weight > max_weight {
            max_weight = weight;
        }
        let dir = grid[pos.y][pos.x]
            .out_direction(dir)
            .unwrap();
        let (x, y) = dir.position(pos.x, pos.y).unwrap();
        if grid.get(y).is_none() || grid[y].get(x).is_none() {
            unreachable!();
        }
        queue.push_back((Position { x, y }, dir, weight + 1));
    }
    println!("{}", debug_grid.join("\n"));
    max_weight
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
            Tile::Empty | Tile::Start => false,
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
