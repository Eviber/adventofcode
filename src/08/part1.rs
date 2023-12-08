use std::collections::HashMap;

pub fn solve(input: &str) -> u64 {
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
    let mut count = 0;
    let mut pos = "AAA";
    'outer: loop {
        for instruction in &instructions {
            pos = instruction.get_pos(map.get(pos).unwrap());
            count += 1;
            if pos == "ZZZ" {
                break 'outer;
            }
        }
    }
    count
}

#[derive(Clone, Copy)]
enum Instruction {
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
