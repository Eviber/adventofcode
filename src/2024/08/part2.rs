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
        for &(x, y) in &coords {
            for &(x2, y2) in &coords {
                if (x, y) == (x2, y2) {
                    continue;
                }
                mark_antinodes(x, y, x2, y2, &mut antinodes);
            }
        }
    }
    for line in &antinodes {
        for &b in line {
            if b {
                eprint!("#");
            } else {
                eprint!(".");
            }
        }
        eprintln!();
    }
    antinodes
        .into_iter()
        .map(|v| v.into_iter().filter(|&b| b).count())
        .sum()
}

fn mark_antinodes(x1: usize, y1: usize, x2: usize, y2: usize, antinodes: &mut [Vec<bool>]) {
    let m = slope(x1, y1, x2, y2);
    for x in 0..antinodes[0].len() {
        let y = fromslope(x, m, x1, y1);
        if y < 0. || y.fract() != 0. || y as usize >= antinodes.len() {
            continue;
        }
        let y = y as usize;
        antinodes[y][x] = true;
    }
}

fn fromslope(x: usize, m: f32, x1: usize, y1: usize) -> f32 {
    m * (x as f32 - x1 as f32) + y1 as f32
}

fn slope(x1: usize, y1: usize, x2: usize, y2: usize) -> f32 {
    (y2 as f32 - y1 as f32) / (x2 as f32 - x1 as f32)
}
