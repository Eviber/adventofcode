use Dir::*;
use Tile::{Empty, Obstacle};

pub fn solve(input: &str) -> usize {
    let mut map = Map::from(input);
    while map.walk_guard() {}
    map.count_visited()
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    visited: Vec<Vec<bool>>,
    guard: Guard,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Obstacle,
}

struct Guard {
    x: usize,
    y: usize,
    dir: Dir,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Map {
    fn walk_guard(&mut self) -> bool {
        self.visited[self.guard.y][self.guard.x] = true;
        self.guard.walk(&self.tiles)
    }

    fn count_visited(&self) -> usize {
        self.visited.iter().flat_map(|v| v.iter().filter(|b| **b)).count()
    }
}

impl Guard {
    fn new(x: usize, y: usize) -> Self {
        Guard { x, y, dir: Up }
    }

    fn walk(&mut self, map: &[Vec<Tile>]) -> bool {
        let (x, y) = self.next_pos();
        if y >= map.len() || x >= map[0].len() {
            self.x = x;
            self.y = y;
            return false;
        }
        if map[y][x] == Empty {
            self.x = x;
            self.y = y;
            return true;
        }
        self.turn_right();
        self.walk(map)
    }

    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }

    fn next_pos(&self) -> (usize, usize) {
        let mut x = self.x;
        let mut y = self.y;
        match self.dir {
            Up => y = y.wrapping_sub(1),
            Down => y += 1,
            Left => x = x.wrapping_sub(1),
            Right => x += 1,
        }
        (x, y)
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let mut tiles = vec![];
        let (mut x, mut y) = (0, 0);
        for (yi, s) in s.split_whitespace().enumerate() {
            let mut line = Vec::with_capacity(s.len());
            for (xi, c) in s.chars().enumerate() {
                line.push(Tile::from(c));
                if c == '^' {
                    (x, y) = (xi, yi);
                }
            }
            tiles.push(line);
        }
        let guard = Guard::new(x, y);
        let visited = vec![vec![false; tiles[0].len()]; tiles.len()];
        Map { tiles, guard, visited }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' | '^' => Empty,
            '#' => Obstacle,
            _ => panic!("invalid character"),
        }
    }
}
