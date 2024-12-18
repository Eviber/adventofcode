use std::{
    collections::VecDeque,
    fmt::Display,
    ops::{Index, IndexMut},
};

use Byte::{Corrupted, Safe};
use Dir::{Down, Left, Right, Up};

pub fn solve(input: &str, size: usize) -> String {
    let corrupted_pos: Vec<_> = input.split_whitespace().map(Pos::from).collect();

    let mut min = 0;
    let mut max = corrupted_pos.len();
    while min + 1 < max {
        let mut memory_map = Memory::new(size);
        let next = (max - min) / 2 + min;
        corrupted_pos
            .iter()
            .take(next)
            .for_each(|&pos| memory_map.corrupt(pos));
        if memory_map.end_reachable() {
            min = next;
        } else {
            max = next;
        }
    }
    format!("{},{}", corrupted_pos[min].x, corrupted_pos[min].y)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Byte {
    Safe,
    Corrupted,
}

struct Memory {
    bytes: Vec<Vec<Byte>>,
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Memory {
    fn new(size: usize) -> Self {
        Self {
            bytes: vec![vec![Byte::Safe; size]; size],
        }
    }

    fn corrupt(&mut self, pos: Pos) {
        assert!(self[pos] == Safe);
        self[pos] = Corrupted;
    }

    fn end_reachable(&self) -> bool {
        let size = self.bytes.len();
        let mut file = VecDeque::new();
        file.push_back(Pos { x: 0, y: 0 });
        let mut visited = vec![vec![false; size]; size];

        while let Some(pos) = file.pop_front() {
            if !pos.in_square(size) || visited[pos.y][pos.x] || self[pos] == Corrupted {
                continue;
            }
            visited[pos.y][pos.x] = true;
            if pos.x == size - 1 && pos.y == size - 1 {
                return true;
            }
            file.push_back(pos + Up);
            file.push_back(pos + Down);
            file.push_back(pos + Left);
            file.push_back(pos + Right);
        }
        false
    }
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn in_square(&self, size: usize) -> bool {
        (0..size).contains(&self.x) && (0..size).contains(&self.y)
    }
}

impl IndexMut<Pos> for Memory {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.bytes[index.y][index.x]
    }
}

impl Index<Pos> for Memory {
    type Output = Byte;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.bytes[index.y][index.x]
    }
}

impl std::ops::Add<Dir> for Pos {
    type Output = Pos;

    fn add(mut self, rhs: Dir) -> Self {
        self += rhs;
        self
    }
}

impl std::ops::AddAssign<Dir> for Pos {
    fn add_assign(&mut self, rhs: Dir) {
        match rhs {
            Up => self.y = self.y.wrapping_sub(1),
            Down => self.y += 1,
            Left => self.x = self.x.wrapping_sub(1),
            Right => self.x += 1,
        }
    }
}

impl std::ops::Sub<Dir> for Pos {
    type Output = Pos;

    fn sub(mut self, rhs: Dir) -> Pos {
        self -= rhs;
        self
    }
}

impl std::ops::SubAssign<Dir> for Pos {
    fn sub_assign(&mut self, rhs: Dir) {
        let rhs = -rhs;
        *self += rhs;
    }
}

impl std::ops::Neg for Dir {
    type Output = Dir;

    fn neg(self) -> Self::Output {
        match self {
            Up => Down,
            Down => Up,
            Right => Left,
            Left => Right,
        }
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, line) in self.bytes.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let _pos = Pos::new(x, y);
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Corrupted => write!(f, "#"),
            Safe => write!(f, "."),
        }
    }
}

impl From<&str> for Pos {
    fn from(s: &str) -> Self {
        let (x, y) = s.trim().split_once(',').expect("a comma");
        let x = x.parse().expect("a number for x");
        let y = y.parse().expect("a number for x");
        Pos { x, y }
    }
}
