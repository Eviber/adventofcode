use std::collections::VecDeque;

pub fn solve(input: &str) -> usize {
    let garden: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut visited = vec![vec![false; garden[0].len()]; garden.len()];
    let mut price = 0;
    for y in 0..garden.len() {
        for x in 0..garden[0].len() {
            if visited[y][x] {
                continue;
            }
            let (area, perimeter) = map_region(x, y, &garden, &mut visited);
            price += area * perimeter;
        }
    }
    price
}

fn map_region(
    x: usize,
    y: usize,
    garden: &[Vec<char>],
    visited: &mut [Vec<bool>],
) -> (usize, usize) {
    let mut file = VecDeque::new();
    file.push_back((x, y));
    let plant = garden[y][x];
    let mut area = 0;
    let mut perimeter = 0;
    while let Some((x, y)) = file.pop_front() {
        if y >= garden.len() || x >= garden[y].len() || visited[y][x] || garden[y][x] != plant {
            continue;
        }
        visited[y][x] = true;
        area += 1;
        if x == 0 || garden[y][x - 1] != plant {
            perimeter += 1;
        }
        if y == 0 || garden[y - 1][x] != plant {
            perimeter += 1;
        }
        if x >= garden[0].len() - 1 || garden[y][x + 1] != plant {
            perimeter += 1;
        }
        if y >= garden.len() - 1 || garden[y + 1][x] != plant {
            perimeter += 1;
        }
        file.push_back((x.wrapping_sub(1), y));
        file.push_back((x, y.wrapping_sub(1)));
        file.push_back((x + 1, y));
        file.push_back((x, y + 1));
    }
    (area, perimeter)
}
