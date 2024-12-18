use std::{
    collections::VecDeque,
    fmt::Display,
    ops::{Index, IndexMut},
};

use Byte::{Corrupted, Safe};
use Dir::{Down, Left, Right, Up};

pub fn solve(input: &str, size: usize) -> usize {
    let corruption_amount = if size == 7 { 12 } else { 1024 };
    let mut memory_map = Memory::new(size);
    input
        .split_whitespace()
        .take(corruption_amount)
        .map(Pos::from)
        .for_each(|pos| memory_map.corrupt(pos));

    let mut file = VecDeque::new();
    file.push_back((0, Pos { x: 0, y: 0 }));
    let mut visited = vec![vec![false; size]; size];
    while let Some((len, pos)) = file.pop_front() {
        if !pos.in_square(size) || visited[pos.y][pos.x] || memory_map[pos] == Corrupted {
            continue;
        }
        visited[pos.y][pos.x] = true;
        if pos.x == size - 1 && pos.y == size - 1 {
            return len;
        }
        file.push_back((len + 1, pos + Up));
        file.push_back((len + 1, pos + Down));
        file.push_back((len + 1, pos + Left));
        file.push_back((len + 1, pos + Right));
    }
    panic!("No path to end found");
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
