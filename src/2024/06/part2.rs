use rayon::prelude::*;

use Dir::{Down, Left, Right, Up};
use Tile::{Empty, Obstacle};

pub fn solve(input: &str) -> usize {
    let mut map = Map::from(input);

    let mut visited = Vec::new();
    visited.push((map.guard.x, map.guard.y));
    while map.walk_guard() {
        let x = map.guard.x;
        let y = map.guard.y;
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.push((x, y));
    }
    map.reset_guard();
    visited
        .into_par_iter()
        .filter(|&(x, y)| {
            let mut map = map.clone();
            map.tiles[y][x] = Obstacle;
            map.is_looping()
        })
        .count()
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    guard: Guard,
    initial_guard: Guard,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Empty,
    Obstacle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    x: usize,
    y: usize,
    dir: Dir,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Map {
    fn walk_guard(&mut self) -> bool {
        self.guard.walk(&self.tiles)
    }

    fn reset_guard(&mut self) {
        self.guard = self.initial_guard;
    }

    fn is_looping(&mut self) -> bool {
        for _ in 0..10000 {
            if !self.walk_guard() {
                self.reset_guard();
                return false;
            }
        }
        let mut passed = Vec::new();
        loop {
            passed.push(self.guard);
            for _ in 0..1000 {
                if !self.walk_guard() {
                    self.reset_guard();
                    return false;
                }
                if passed.contains(&self.guard) {
                    self.reset_guard();
                    return true;
                }
            }
        }
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
        Map {
            tiles,
            guard,
            initial_guard: guard,
        }
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
