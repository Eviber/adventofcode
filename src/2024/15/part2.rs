use std::{
    fmt::Display,
    iter,
    ops::{Index, IndexMut},
};
use Dir::{Down, Left, Right, Up};
use Tile::*;

pub fn solve(input: &str) -> usize {
    let (map_str, dirs) = input.split_once("\n\n").unwrap();
    let mut map = Map::from(map_str);
    let dirs = dirs.chars().filter(|c| !c.is_whitespace()).map(Dir::from);
    for dir in dirs {
        map.move_bot(dir);
    }
    map.tiles
        .into_iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.into_iter()
                .enumerate()
                .filter(|(_, t)| *t == BoxLeft)
                .map(move |(x, _)| x + 100 * y)
        })
        .sum()
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    BoxLeft,
    BoxRight,
    Empty,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct UVec2 {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
struct Robot {
    pos: UVec2,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    robot: Robot,
}

impl Map {
    fn move_bot(&mut self, dir: Dir) {
        assert!(self[self.robot.pos] == Empty);
        let next = self.robot.pos + dir;
        if !self.can_push(next, dir) {
            return;
        }
        self.push(next, dir);
        self.robot.pos = next;
    }

    fn can_push(&self, pos: UVec2, dir: Dir) -> bool {
        if self[pos] == Empty {
            return true;
        }
        if self[pos] == Wall {
            return false;
        }
        let mut left = pos;
        if self[left] == BoxRight {
            left.x -= 1;
        }
        let mut right = left;
        right.x += 1;
        match dir {
            Up | Down => self.can_push(left + dir, dir) && self.can_push(right + dir, dir),
            Left | Right => self.can_push(pos + dir, dir),
        }
    }

    fn push(&mut self, pos: UVec2, dir: Dir) {
        assert!(self[pos] != Wall);
        if self[pos] == Empty {
            return;
        }
        let mut left = pos;
        if self[left] == BoxRight {
            left.x -= 1;
        }
        let mut right = left;
        right.x += 1;
        match dir {
            Up | Down => {
                self.push(left + dir, dir);
                self.push(right + dir, dir);
                self[left + dir] = self[left];
                self[right + dir] = self[right];
                self[left] = Empty;
                self[right] = Empty;
            }
            Left | Right => {
                self.push(pos + dir, dir);
                self[pos + dir] = self[pos];
                self[pos] = Empty;
            },
        }
    }
}

impl Tile {
    fn pair_from(c: char) -> impl Iterator<Item = Tile> {
        let tile = Tile::from(c);
        if tile == BoxLeft {
            return iter::once(tile).chain(iter::once(BoxRight));
        }
        iter::once(tile).chain(iter::once(tile))
    }
}

impl UVec2 {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
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
            Up => self.y -= 1,
            Down => self.y += 1,
            Left => self.x -= 1,
            Right => self.x += 1,
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
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, line) in self.tiles.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if UVec2::new(x, y) == self.robot.pos {
                    write!(f, "@")?;
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
            BoxLeft => write!(f, "["),
            BoxRight => write!(f, "]"),
            Empty => write!(f, "."),
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Up => write!(f, "up"),
            Down => write!(f, "down"),
            Left => write!(f, "left"),
            Right => write!(f, "right"),
        }
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let mut pos = None;
        let tiles: Vec<Vec<_>> = s
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .flat_map(|(x, c)| {
                        if c == '@' {
                            pos = Some((x, y));
                        }
                        Tile::pair_from(c)
                    })
                    .collect()
            })
            .collect();
        let (x, y) = pos.expect("Invalid map, missing '@'");
        let robot = Robot {
            pos: UVec2::new(x * 2, y),
        };
        Map { tiles, robot }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '#' => Wall,
            'O' => BoxLeft,
            '.' | '@' => Empty,
            _ => panic!("Invalid tile character '{c}'"),
        }
    }
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            '^' => Up,
            'v' => Down,
            '<' => Left,
            '>' => Right,
            _ => panic!("Invalid direction character '{c}'"),
        }
    }
}
