use std::{
    collections::VecDeque,
    fmt::Display,
    ops::{Index, IndexMut},
};
use Dir::{East, North, South, West};
use Tile::{Empty, Wall};

pub fn solve(input: &str) -> usize {
    let map = Map::from(input);
    let reindeer = map.starting_reindeer;
    let mut best_scores = vec![vec![Scores::new(); map.tiles[0].len()]; map.tiles.len()];
    let mut file: VecDeque<Reindeer> = VecDeque::new();
    file.push_back(reindeer);
    while let Some(reindeer) = file.pop_front() {
        if best_scores[reindeer.pos.y][reindeer.pos.x][reindeer.dir] <= reindeer.score {
            continue;
        }
        best_scores[reindeer.pos.y][reindeer.pos.x][reindeer.dir] = reindeer.score;
        file.push_back(reindeer.turn_left());
        file.push_back(reindeer.turn_right());
        file.push_back(reindeer.walk(&map));
    }
    eprintln!("{map}");
    best_scores[map.end.y][map.end.x].scores.into_iter().min().unwrap()
}

#[derive(Clone, Copy)]
struct Scores {
    scores: [usize; 4],
}

impl Scores {
    fn new() -> Scores {
        Scores {
            scores: [usize::MAX; 4],
        }
    }
}

#[derive(Clone, Copy)]
struct Reindeer {
    pos: UVec2,
    dir: Dir,
    score: usize,
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

#[derive(Clone, Copy, PartialEq, Eq)]
struct UVec2 {
    x: usize,
    y: usize,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    starting_reindeer: Reindeer,
    end: UVec2,
}

impl Reindeer {
    fn walk(&self, map: &Map) -> Reindeer {
        let next_pos = self.pos + self.dir;
        if map[next_pos] == Wall {
            return *self;
        }
        let mut res = *self;
        res.pos = next_pos;
        res.score += 1;
        res
    }

    fn turn_left(&self) -> Reindeer {
        let mut res = *self;
        res.score += 1000;
        res.dir = match res.dir {
            North => West,
            West => South,
            South => East,
            East => North,
        };
        res
    }

    fn turn_right(&self) -> Reindeer {
        let mut res = self.turn_left();
        res.dir = -res.dir;
        res
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

impl IndexMut<Dir> for Scores {
    fn index_mut(&mut self, dir: Dir) -> &mut Self::Output {
        &mut self.scores[dir as usize]
    }
}

impl Index<Dir> for Scores {
    type Output = usize;

    fn index(&self, dir: Dir) -> &Self::Output {
        &self.scores[dir as usize]
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
            North => self.y -= 1,
            South => self.y += 1,
            East => self.x += 1,
            West => self.x -= 1,
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
                if pos == self.starting_reindeer.pos {
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
        let starting_reindeer = Reindeer {
            pos: UVec2::new(x, y),
            dir: East,
            score: 0,
        };
        Map {
            tiles,
            starting_reindeer,
            end,
        }
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
