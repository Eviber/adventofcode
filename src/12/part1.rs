use std::fmt::Display;

use rayon::prelude::*;

pub fn solve(input: &str) -> u64 {
    println!("{}", input);
    let mut rows: Vec<Row> = input.lines().map(Row::from).collect();
    rows.iter_mut().for_each(Row::simplify);
    rows.par_iter().map(Row::count_solutions).sum()
}

#[derive(Debug, Clone)]
struct Row {
    springs: Vec<Spring>,
    broken_sets: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Broken,
    Intact,
    Unknown,
}

impl Row {
    fn simplify(&mut self) {
        let sets: Vec<Vec<Spring>> = self
            .springs
            .split(|s| s.is_intact())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_vec())
            .collect();
        self.springs = sets.join(&Spring::Intact);
    }

    fn is_valid(&self) -> bool {
        self.springs
            .split(|s| !s.is_broken())
            .filter(|s| !s.is_empty())
            .count()
            == self.broken_sets.len()
            && self
                .springs
                .split(|s| !s.is_broken())
                .filter(|s| !s.is_empty())
                .zip(self.broken_sets.iter())
                .all(|(springs, &len)| springs.len() == len)
    }

    fn count_solutions(&self) -> u64 {
        let Some(i) = self.springs.iter().position(|s| s.is_unknown()) else {
            if self.is_valid() {
                println!("Valid: {}", self);
                return 1;
            }
            return 0;
        };
        let mut count = 0;
        {
            let mut row = self.clone();
            row.springs[i] = Spring::Broken;
            count += row.count_solutions();
            row.springs[i] = Spring::Intact;
            count += row.count_solutions();
        }
        count
    }
}

impl Spring {
    fn is_intact(self) -> bool {
        self == Spring::Intact
    }

    fn is_broken(self) -> bool {
        self == Spring::Broken
    }

    fn is_unknown(self) -> bool {
        self == Spring::Unknown
    }
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '.' => Spring::Intact,
            '#' => Spring::Broken,
            '?' => Spring::Unknown,
            _ => panic!("Invalid spring char"),
        }
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spring::Broken => write!(f, "#"),
            Spring::Intact => write!(f, "."),
            Spring::Unknown => write!(f, "?"),
        }
    }
}

impl From<&str> for Row {
    fn from(s: &str) -> Self {
        let (part1, part2) = s.split_once(' ').unwrap();
        let springs = part1.chars().map(Spring::from).collect();
        let broken_sets = part2
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        Row {
            springs,
            broken_sets,
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for spring in &self.springs {
            write!(f, "{}", spring)?;
        }
        write!(f, " ")?;
        for (i, broken_set) in self.broken_sets.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", broken_set)?;
        }
        Ok(())
    }
}
