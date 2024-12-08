use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let mut coords = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            coords.entry(c).or_insert_with(Vec::new).push((x, y));
        }
    }
    let w = input.chars().take_while(|&c| c != '\n').count();
    let h = input.lines().count();
    let mut antinodes = vec![vec![false; w]; h];
    for coords in coords.into_values() {
        // eprintln!("{coords:?}");
        for &(x, y) in &coords {
            for &(x2, y2) in &coords {
                if (x, y) == (x2, y2) {
                    continue;
                }
                // eprint!("{x}, {y} <=> {x2}, {y2} ==> ");
                let x = (x + x).wrapping_sub(x2);
                let y = (y + y).wrapping_sub(y2);
                if x < w && y < h {
                    // eprint!("{x}, {y}");
                    antinodes[y][x] = true;
                } else {
                    // eprint!("X");
                }
                // eprintln!();
            }
        }
    }
    // for line in &antinodes {
    //     for &b in line {
    //         if b {
    //             eprint!("#");
    //         } else {
    //             eprint!(".");
    //         }
    //     }
    //     eprintln!();
    // }
    antinodes
        .into_iter()
        .map(|v| v.into_iter().filter(|&b| b).count())
        .sum()
}
