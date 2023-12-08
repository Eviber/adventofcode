use crate::math::lcm;
use std::collections::HashMap;

use rayon::prelude::*;

pub fn solve(input: &str) -> u64 {
    let (map, pos, instructions) = parse_input(input);
    inner(&map, &pos, &instructions)
}

pub fn inner(map: &HashMap<&str, (&str, &str)>, pos: &[&str], instructions: &[Instruction]) -> u64 {
    pos.par_iter()
        .fold(|| 1, |cm, pos| lcm(cm, loop_size(&map, pos, &instructions)))
        .reduce(|| 1, lcm)
}

#[allow(dead_code)] // Non-parallel version, for comparison
pub fn inner2(
    map: &HashMap<&str, (&str, &str)>,
    pos: &[&str],
    instructions: &[Instruction],
) -> u64 {
    pos.iter()
        .fold(1, |cm, pos| lcm(cm, loop_size(&map, pos, &instructions)))
}

fn loop_size(map: &HashMap<&str, (&str, &str)>, start: &str, instructions: &[Instruction]) -> u64 {
    let mut count = 0;
    let mut pos = start;
    'outer: loop {
        for instruction in instructions {
            let next = map.get(pos).unwrap();
            pos = instruction.get_pos(next);
            count += 1;
            if pos.ends_with('Z') {
                break 'outer;
            }
        }
    }
    count
}

pub fn parse_input(input: &str) -> (HashMap<&str, (&str, &str)>, Vec<&str>, Vec<Instruction>) {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| Instruction::try_from(c).unwrap())
        .collect::<Vec<_>>();
    lines.next().unwrap();
    let map: HashMap<&str, (&str, &str)> = lines
        .map(|line| {
            let (key, values) = line.split_once(" = ").unwrap();
            let (left, right) = values.trim_matches(&['(', ')']).split_once(", ").unwrap();
            (key, (left, right))
        })
        .collect();
    let pos: Vec<&str> = map.keys().filter(|k| k.ends_with('A')).copied().collect();
    (map, pos, instructions)
}

#[derive(Clone, Copy)]
pub enum Instruction {
    Left,
    Right,
}

impl Instruction {
    fn get_pos<'a>(&'a self, pos: &(&'a str, &'a str)) -> &str {
        match self {
            Instruction::Left => pos.0,
            Instruction::Right => pos.1,
        }
    }
}

impl TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(()),
        }
    }
}
