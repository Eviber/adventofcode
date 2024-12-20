use std::iter;
use std::{
    collections::VecDeque,
    fmt::Display,
    ops::{Index, IndexMut},
};
use Dir::{East, North, South, West};
use Tile::{Empty, Wall};

pub fn solve(input: &str, cheat_len: usize, score_threshold: usize) -> usize {
    let map = Map::from(input);
    let width = map.tiles[0].len();
    let height = map.tiles.len();

    let dist_to_end = map.get_len_map();
    map.iter_pos()
        .filter(|&pos| map[pos] != Wall)
        .flat_map(|start_cheat| {
            let window = (start_cheat - cheat_len).iter_to((start_cheat + cheat_len).bound_to(width, height));
            iter::repeat(start_cheat)
                .zip(window)
                .filter(|&(start_cheat, end_cheat)| {
                    let dist = start_cheat.manhattan(end_cheat);
                    if map[end_cheat] == Wall || dist > cheat_len {
                        return false;
                    }
                    let dist_orig = dist_to_end[start_cheat.y][start_cheat.x];
                    let dist_cheat = dist_to_end[end_cheat.y][end_cheat.x] + dist + score_threshold;
                    dist_orig >= dist_cheat
                })
        })
        .count()
}

impl Map {
    fn get_len_map(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![vec![usize::MAX; self.tiles[0].len()]; self.tiles.len()];
        let mut file: VecDeque<(usize, UVec2)> = VecDeque::new();
        file.push_back((0, self.end));
        while let Some((len, pos)) = file.pop_front() {
            if pos.y >= self.tiles.len()
                || pos.x >= self.tiles[0].len()
                || visited[pos.y][pos.x] < usize::MAX
                || self[pos] == Wall
            {
                continue;
            }
            visited[pos.y][pos.x] = len;
            if pos == self.start {
                return visited;
            }
            file.push_back((len + 1, pos + North));
            file.push_back((len + 1, pos + South));
            file.push_back((len + 1, pos + East));
            file.push_back((len + 1, pos + West));
        }
        panic!("End unreachable");
    }

    fn iter_pos(&self) -> impl Iterator<Item = UVec2> {
        let start = UVec2 { x: 0, y: 0 };
        let end = UVec2 {
            x: self.tiles[0].len() - 1,
            y: self.tiles.len() - 1,
        };
        start.iter_to(end)
    }
}

#[derive(Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct UVec2 {
    x: usize,
    y: usize,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: UVec2,
    end: UVec2,
}

impl UVec2 {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn manhattan(self, other: UVec2) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn bound_to(mut self, width: usize, height: usize) -> Self {
        self.x = self.x.min(width - 1);
        self.y = self.y.min(height - 1);
        self
    }

    fn iter_to(self, end: UVec2) -> impl Iterator<Item = UVec2> {
        (self.y..=end.y).flat_map(move |y| (self.x..=end.x).map(move |x| UVec2 { x, y }))
    }
}

impl IndexMut<UVec2> for Map {
    fn index_mut(&mut self, index: UVec2) -> &mut Self::Output {
        &mut self.tiles[index.y][index.x]
    }
}

impl Index<UVec2> for Map {
    type Output = Tile;

    fn index(&self, index: UVec2) -> &Self::Output {
        &self.tiles[index.y][index.x]
    }
}

impl std::ops::Add<usize> for UVec2 {
    type Output = UVec2;

    fn add(self, rhs: usize) -> Self::Output {
        UVec2::new(self.x + rhs, self.y + rhs)
    }
}

impl std::ops::Sub<usize> for UVec2 {
    type Output = UVec2;

    fn sub(self, rhs: usize) -> Self::Output {
        UVec2::new(self.x.saturating_sub(rhs), self.y.saturating_sub(rhs))
    }
}

impl std::ops::Add<Dir> for UVec2 {
    type Output = UVec2;

    fn add(mut self, rhs: Dir) -> Self {
        self += rhs;
        self
    }
}

impl std::ops::AddAssign<Dir> for UVec2 {
    fn add_assign(&mut self, rhs: Dir) {
        match rhs {
            North => self.y = self.y.wrapping_sub(1),
            South => self.y += 1,
            East => self.x += 1,
            West => self.x = self.x.wrapping_sub(1),
        }
    }
}

impl std::ops::Sub<Dir> for UVec2 {
    type Output = UVec2;

    fn sub(mut self, rhs: Dir) -> UVec2 {
        self -= rhs;
        self
    }
}

impl std::ops::SubAssign<Dir> for UVec2 {
    fn sub_assign(&mut self, rhs: Dir) {
        let rhs = -rhs;
        *self += rhs;
    }
}

impl std::ops::Neg for Dir {
    type Output = Dir;

    fn neg(self) -> Self::Output {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, line) in self.tiles.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let pos = UVec2::new(x, y);
                if pos == self.end {
                    write!(f, "E")?;
                    continue;
                }
                if pos == self.start {
                    write!(f, "S")?;
                    continue;
                }
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Wall => write!(f, "#"),
            Empty => write!(f, "."),
        }
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let mut start = None;
        let mut end = None;
        let tiles: Vec<Vec<_>> = s
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start = Some((x, y));
                        }
                        if c == 'E' {
                            end = Some((x, y));
                        }
                        Tile::from(c)
                    })
                    .collect()
            })
            .collect();
        let (x, y) = end.expect("Invalid map, missing 'E'");
        let end = UVec2::new(x, y);
        let (x, y) = start.expect("Invalid map, missing 'S'");
        let start = UVec2::new(x, y);
        Map { tiles, start, end }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '#' => Wall,
            '.' | 'S' | 'E' => Empty,
            _ => panic!("Invalid tile character '{c}'"),
        }
    }
}
