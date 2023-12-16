use std::{fmt::Display, str::FromStr};

pub fn solve(input: &str) -> usize {
    let patterns: Vec<Pattern> = input.split("\n\n").map(|s| s.parse().unwrap()).collect();
    let mut count = 0;
    for mut pattern in patterns {
        print!("{}", pattern);
        let c = pattern.count();
        println!("{}", c);
        println!();
        count += c;
    }
    count
}

struct Pattern {
    rows: Vec<Row>,
}

#[derive(PartialEq, Eq, Clone)]
struct Row {
    contents: u32,
    length: usize,
}

impl Pattern {
    /// count the number of rows on top of the mirror
    /// or the number of columns on the left of the mirror
    fn count(&mut self) -> usize {
        let (orig_count, top) = if let Some(count) = self.count_top(0, true) {
            (count, true)
        } else if let Some(count) = self.count_left(0, false) {
            (count, false)
        } else {
            panic!("no original count found");
        };
        println!("orig_count: {}", orig_count);
        println!("left: {}", top);
        for i in 0..self.rows.len() {
            print!("{:2} ", i);
            for j in 0..self.rows[i].length {
                print!("{:2} ", j);
                self.rows[i].flip(j);
                if let Some(count) = self.count_left(orig_count, top) {
                    if count != orig_count || top {
                        println!("\n{},{}", j, i);
                        println!("{}", self);
                        return count;
                    }
                }
                if let Some(count) = self.count_top(orig_count, top) {
                    if count != orig_count || !top {
                        println!("\n{},{}", j, i);
                        println!("{}", self);
                        return count * 100;
                    }
                }
                self.rows[i].flip(j);
            }
            println!();
        }
        panic!("no solution found");
    }

    fn count_left(&self, avoid: usize, top: bool) -> Option<usize> {
        let rotated = self.rotate();
        rotated.count_top(avoid, !top)
    }

    fn count_top(&self, avoid: usize, top: bool) -> Option<usize> {
        for i in 1..self.rows.len() {
            let mut symmetric = true;
            for j in 0..i.min(self.rows.len() - i) {
                // println!(
                //     "{:b} ?= {:b}",
                //     self.rows[i + j].contents,
                //     self.rows[i - 1 - j].contents
                // );
                if self.rows[i + j] != self.rows[i - 1 - j] {
                    symmetric = false;
                    break;
                }
            }
            // println!("symmetric: {}", symmetric);
            // println!();
            if symmetric && (!top || i != avoid) {
                return Some(i);
            }
        }
        None
    }

    /// Creates a new pattern by rotating the current one 90 degrees clockwise
    fn rotate(&self) -> Pattern {
        let mut rows = Vec::new();
        for i in 0..self.rows[0].length {
            let mut contents = 0;
            for j in 0..self.rows.len() {
                contents <<= 1;
                contents |= (self.rows[j].contents >> (self.rows[j].length - 1 - i)) & 1;
            }
            rows.push(Row {
                contents,
                length: self.rows.len(),
            });
        }
        Pattern { rows }
    }
}

impl Row {
    #[inline]
    fn flip(&mut self, i: usize) {
        self.contents ^= 1 << (self.length - 1 - i);
    }
}

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Row> = s.lines().map(|line| line.parse::<Row>().unwrap()).collect();
        Ok(Pattern { rows })
    }
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let contents = s.chars().fold(0, |acc, c| acc << 1 | (c == '#') as u32);
        let length = s.len();
        Ok(Row { contents, length })
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>width$b}", self.contents, width = self.length)
    }
}
