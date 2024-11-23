use std::{collections::HashMap, fmt::Display};

use rayon::prelude::*;

pub fn solve(input: &str) -> u64 {
    let mut rows: Vec<Row> = input.lines().map(Row::from).collect();
    rows.iter_mut().for_each(Row::unfold);
    rows.into_par_iter().map(Row::count_solutions).sum()
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

    #[inline]
    fn iter_springs(&self) -> std::iter::Copied<std::slice::Iter<'_, Spring>> {
        self.springs.iter().copied()
    }

    fn count_solutions_inner(mut self, memo: &mut HashMap<Row, u64>) -> u64 {
        // skip intact springs
        let n = self.iter_springs().take_while(Spring::is_intact).count();
        self.springs.drain(..n);
        if self.broken_sets.is_empty() && !self.springs.iter().any(Spring::is_broken) {
            return 1;
        }
        if self.broken_sets.is_empty()
            || self.springs.is_empty()
            || self.broken_sets.iter().sum::<usize>() + self.broken_sets.len() - 1
                > self.springs.len()
        {
            return 0;
        }
        if let Some(&res) = memo.get(&self) {
            return res;
        }
        let input = self.clone();

        if self.springs[0].is_unknown() {
            let mut broken = self.clone();
            let mut intact = self;
            broken.springs[0] = Spring::Broken;
            intact.springs[0] = Spring::Intact;
            let res = broken.count_solutions_inner(memo) + intact.count_solutions_inner(memo);
            memo.insert(input, res);
            return res;
        }

        let mut set_len = self.broken_sets[0];
        if Some(&Spring::Broken) == self.springs.get(set_len)
            || self.springs.iter().take(set_len).any(Spring::is_intact)
        {
            // if the set cannot be ended or finished, no solutions
            return 0;
        }
        // add one for the separating dot, if needed
        set_len += (self.springs.len() > set_len) as usize;
        self.springs.drain(..set_len);
        self.broken_sets.remove(0);
        let res = self.count_solutions_inner(memo);
        memo.insert(input, res);
        res
    }

    fn count_solutions(self) -> u64 {
        let mut memo = HashMap::new();
        self.count_solutions_inner(&mut memo)
    }
}

impl Spring {
    fn is_intact(&self) -> bool {
        *self == Spring::Intact
    }

    fn is_broken(&self) -> bool {
        *self == Spring::Broken
    }

    fn is_unknown(&self) -> bool {
        *self == Spring::Unknown
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
