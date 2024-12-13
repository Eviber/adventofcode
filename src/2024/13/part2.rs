use cached::proc_macro::cached;
use std::{
    cmp::{max, min},
    ops::{AddAssign, SubAssign},
};

use regex::Regex;

pub fn solve(input: &str) -> usize {
    let machines = input.split("\n\n").map(Machine::from);

    machines
        .filter_map(|m| m.compute_min_token())
        .inspect(|n| eprintln!("{n}"))
        .sum()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Machine {
    a: Pos,
    b: Pos,
    prize: Pos,
}

#[cached]
fn compute_min_token_inner(machine: Machine) -> Option<usize> {
    if machine.prize.is_zero() {
        return Some(0);
    }
    if machine.prize.is_neg() {
        return None;
    }
    let mut machine_a = machine;
    let mut machine_b = machine;
    machine_a.prize -= machine_a.a;
    machine_b.prize -= machine_b.b;
    let a = machine_a.compute_min_token().map(|n| n + 3);
    let b = machine_b.compute_min_token().map(|n| n + 1);
    if a.is_none() || b.is_none() {
        return max(a, b);
    }
    min(a, b)
}

impl Machine {
    fn compute_min_token(self) -> Option<usize> {
        compute_min_token_inner(self)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    #[inline]
    fn is_neg(&self) -> bool {
        self.x < 0 || self.y < 0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }
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
        eprintln!("{s}");
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
