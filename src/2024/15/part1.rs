use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};
use Dir::{Up, Down, Left, Right};
use Tile::{Wall, Box, Empty};

pub fn solve(input: &str) -> usize {
    let (map_str, dirs) = input.split_once("\n\n").unwrap();
    let mut map = Map::from(map_str);
    let dirs = dirs.chars().filter(|c| !c.is_whitespace()).map(Dir::from);
    for dir in dirs {
        map.move_bot(dir);
    }
    eprintln!("{map}");
    map.tiles
        .into_iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.into_iter()
                .enumerate()
                .filter(|(_, t)| *t == Box)
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
    Box,
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
        let mut next = self.robot.pos + dir;
        while self[next] == Box {
            next += dir;
        }
        if self[next] == Wall {
            return;
        }
        while next != self.robot.pos {
            self[next] = self[next - dir];
            next -= dir;
        }
        self.robot.pos += dir;
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
            Box => write!(f, "O"),
            Empty => write!(f, "."),
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
                    .map(|(x, c)| {
                        if c == '@' {
                            pos = Some((x, y));
                        }
                        Tile::from(c)
                    })
                    .collect()
            })
            .collect();
        let (x, y) = pos.expect("Invalid map, missing '@'");
        let robot = Robot {
            pos: UVec2::new(x, y),
        };
        Map { tiles, robot }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '#' => Wall,
            'O' => Box,
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
