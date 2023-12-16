use std::sync::{Mutex, OnceLock};
use std::{collections::HashMap, fmt::Display};

use rayon::prelude::*;

// HashMap of every call made to count_solutions with the result
// (aka memoization)
static COUNTS: OnceLock<Mutex<HashMap<Row, u64>>> = OnceLock::new();

pub fn solve(input: &str) -> u64 {
    println!("{}", input);
    // initialize the memoization HashMap
    COUNTS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut rows: Vec<Row> = input.lines().map(Row::from).collect();
    rows.iter_mut().for_each(Row::unfold);
    rows.iter_mut().for_each(Row::simplify);
    println!("\nafter unfold:");
    for row in &rows {
        println!("{}", row);
    }
    println!();
    println!();
    let count = rows
        .into_par_iter()
        .map(|mut row| row.count_solutions())
        .inspect(|s| println!("{}", s))
        .sum();
    println!("\n\n");
    println!(
        "memoization HashMap size: {}",
        COUNTS.get().unwrap().lock().unwrap().len()
    );
    count
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Row {
    springs: Vec<Spring>,
    broken_sets: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Broken,
    Intact,
    Unknown,
}

impl Row {
    fn unfold(&mut self) {
        let mut springs = Vec::with_capacity(self.springs.len() * 5 + 4);
        springs.extend(&self.springs);
        springs.push(Spring::Unknown);
        springs.extend(&self.springs);
        springs.push(Spring::Unknown);
        springs.extend(&self.springs);
        springs.push(Spring::Unknown);
        springs.extend(&self.springs);
        springs.push(Spring::Unknown);
        springs.extend(&self.springs);
        self.springs = springs;
        self.broken_sets = self.broken_sets.repeat(5);
    }

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

    fn can_be_valid(&self) -> bool {
        self.springs
            .split(|s| s.is_intact())
            .filter(|s| !s.is_empty())
            .take_while(|s| !s.contains(&Spring::Unknown))
            .zip(self.broken_sets.iter())
            .all(|(springs, &len)| springs.len() == len)
    }

    fn count_solutions(&mut self) -> u64 {
        // look up the result in the memoization HashMap
        {
            let counts = COUNTS.get().unwrap();
            if let Some(count) = counts.lock().unwrap().get(self) {
                println!("memoized!");
                return *count;
            }
        }
        if !self.can_be_valid() {
            // update the memoization HashMap
            {
                let mut counts = COUNTS.get().unwrap().lock().unwrap();
                counts.insert(self.clone(), 0);
            }
            return 0;
        }
        {
            // let copy = self.clone();
            self.deduce();
            // if copy != *self {
            // println!("{}\n{}\n", copy, self);
            // }
        }
        if !self.can_be_valid() {
            {
                let mut counts = COUNTS.get().unwrap().lock().unwrap();
                counts.insert(self.clone(), 0);
            }
            return 0;
        }
        let Some(i) = self.springs.iter().position(|s| s.is_unknown()) else {
            {
                let mut counts = COUNTS.get().unwrap().lock().unwrap();
                counts.insert(self.clone(), self.is_valid() as u64);
            }
            return self.is_valid() as u64;
        };
        let mut count = 0;
        {
            let mut row = self.clone();
            row.springs[i] = Spring::Broken;
            if row.can_be_valid() {
                count += row.count_solutions();
            }
            let mut row = self.clone();
            row.springs[i] = Spring::Intact;
            if row.can_be_valid() {
                count += row.count_solutions();
            }
        }
        {
            let mut counts = COUNTS.get().unwrap().lock().unwrap();
            counts.insert(self.clone(), count);
        }
        count
    }

    fn deduce(&mut self) {
        let Some(first_unknown) = self.springs.iter().position(|s| s.is_unknown()) else {
            return;
        };
        if first_unknown == 0 || self.springs[first_unknown - 1].is_intact() {
            return;
        }
        let mut broken_set_start = first_unknown - 1;
        while broken_set_start > 0 && self.springs[broken_set_start - 1].is_broken() {
            broken_set_start -= 1;
        }
        let mut broken_set_len = first_unknown - broken_set_start;
        let broken_set_index = self
            .springs
            .split(|s| s.is_intact())
            .filter(|s| !s.is_empty())
            .enumerate()
            .find(|(_, springs)| springs.contains(&Spring::Unknown))
            .map(|(i, _)| i)
            .expect("There should be an unknown spring");
        if broken_set_index >= self.broken_sets.len() {
            return;
        }
        let mut i = first_unknown;
        while i < self.springs.len()
            && broken_set_len < self.broken_sets[broken_set_index]
            && !self.springs[i].is_intact()
        {
            self.springs[i] = Spring::Broken;
            i += 1;
            broken_set_len += 1;
        }
        if i < self.springs.len() && self.springs[i].is_unknown() {
            self.springs[i] = Spring::Intact;
        }
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
