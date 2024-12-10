use std::{collections::{HashSet, VecDeque}, iter::repeat};

pub fn solve(input: &str) -> usize {
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    map.iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter_map(|(x, &h)| (h == 0).then_some(x))
                .zip(repeat(y))
                .map(|(x, y)| trailhead_rating(&map, x, y))
        })
        .sum()
}

fn trailhead_rating(map: &[Vec<u32>], x: usize, y: usize) -> usize {
    let mut file = VecDeque::new();
    file.push_back((x, y));
    let mut visited = HashSet::new();
    let mut summits = vec![];

    while let Some((x, y)) = file.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        if map[y][x] == 9 {
            summits.push((x, y));
            continue;
        }
        if x > 0 && map[y][x-1] == map[y][x] + 1 {
            file.push_back((x - 1, y));
        }
        if x < map[y].len() - 1 && map[y][x + 1] == map[y][x] + 1 {
            file.push_back((x + 1, y));
        }
        if y > 0 && map[y-1][x] == map[y][x] + 1 {
            file.push_back((x, y - 1));
        }
        if y < map.len() - 1 && map[y + 1][x] == map[y][x] + 1 {
            file.push_back((x, y + 1));
        }
    }
    if summits.is_empty() {
        return 0;
    }
    let mut ratings = vec![vec![0; map[0].len()]; map.len()];
    for (x, y) in summits {
        ratings[y][x] = 1;
    }
    for n in (0..=8).rev() {
        for &(x, y) in &visited {
            if map[y][x] != n {
                continue;
            }
            let mut rating = 0;
            if x > 0 && map[y][x-1] == map[y][x] + 1 {
                rating += ratings[y][x - 1];
            }
            if x < map[y].len() - 1 && map[y][x + 1] == map[y][x] + 1 {
                rating += ratings[y][x + 1];
            }
            if y > 0 && map[y-1][x] == map[y][x] + 1 {
                rating += ratings[y - 1][x];
            }
            if y < map.len() - 1 && map[y + 1][x] == map[y][x] + 1 {
                rating += ratings[y + 1][x];
            }
            ratings[y][x] = rating;
        }
    }
    ratings[y][x]
}
