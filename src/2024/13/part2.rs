use rayon::iter::ParallelIterator;
use std::ops::{AddAssign, SubAssign};

use rayon::iter::ParallelBridge;
use regex::Regex;

pub fn solve(input: &str) -> usize {
    let machines = input.split("\n\n").par_bridge().map(Machine::from);

    machines
        .map(|m| m.compute_min_token())
        .sum()
}

#[derive(Clone, Copy)]
struct Machine {
    a: Pos,
    b: Pos,
    prize: Pos,
}

impl Machine {
    fn compute_min_token(self) -> usize {
        let d = self.a.x * self.b.y - self.a.y * self.b.x;
        assert!(d != 0);
        let a = (self.b.x * self.prize.y - self.b.y * self.prize.x) / d;
        let b = (self.prize.x * self.a.y - self.prize.y * self.a.x) / d;
        let a = -a;
        let b = -b;
        let res = Pos { x: a * self.a.x + b * self.b.x, y: a * self.a.y + b * self.b.y };
        if res != self.prize {
            return 0;
        }
        (a * 3 + b) as usize
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: i64,
    y: i64,
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl From<&str> for Machine {
    fn from(s: &str) -> Self {
        let reg_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
        let reg_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
        let reg_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
        let Some((_, [ax, ay])) = reg_a.captures(s).map(|c| c.extract()) else {
            panic!("Machine parsing error: cannot parse button a");
        };
        let Some((_, [bx, by])) = reg_b.captures(s).map(|c| c.extract()) else {
            panic!("Machine parsing error: cannot parse button b");
        };
        let Some((_, [px, py])) = reg_prize.captures(s).map(|c| c.extract()) else {
            panic!("Machine parsing error: cannot parse prize");
        };
        let a = Pos {
            x: ax.parse().unwrap(),
            y: ay.parse().unwrap(),
        };
        let b = Pos {
            x: bx.parse().unwrap(),
            y: by.parse().unwrap(),
        };
        let prize = Pos {
            x: px.parse::<i64>().unwrap() + 10000000000000,
            y: py.parse::<i64>().unwrap() + 10000000000000,
        };
        Machine { a, b, prize }
    }
}
