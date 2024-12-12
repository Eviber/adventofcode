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
            let (area, sides) = map_region(x, y, &garden, &mut visited);
            price += area * sides;
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
    let mut in_region = vec![vec![false; garden[0].len()]; garden.len()];
    let mut file = VecDeque::new();
    file.push_back((x, y));
    let plant = garden[y][x];
    let mut area = 0;
    while let Some((x, y)) = file.pop_front() {
        if y >= garden.len() || x >= garden[y].len() || in_region[y][x] || garden[y][x] != plant {
            continue;
        }
        in_region[y][x] = true;
        area += 1;
        file.push_back((x.wrapping_sub(1), y));
        file.push_back((x, y.wrapping_sub(1)));
        file.push_back((x + 1, y));
        file.push_back((x, y + 1));
    }
    let sides = calc_sides(&in_region);
    for (y, line) in in_region.into_iter().enumerate() {
        for (x, b) in line.into_iter().enumerate() {
            if b {
                visited[y][x] = true;
            }
        }
    }
    (area, sides)
}

fn calc_sides(in_region: &[Vec<bool>]) -> usize {
    let mut sides = 0;
    for y in 0..in_region.len() {
        let mut up_border = false;
        let mut down_border = false;
        for x in 0..in_region[0].len() {
            if (in_region[y][x] && (y == 0 || !in_region[y - 1][x])) != up_border {
                up_border = !up_border;
                if up_border {
                    sides += 1;
                }
            }
            if (in_region[y][x] && (y >= in_region.len() - 1 || !in_region[y + 1][x]))
                != down_border
            {
                down_border = !down_border;
                if down_border {
                    sides += 1;
                }
            }
        }
    }
    for x in 0..in_region[0].len() {
        let mut left_border = false;
        let mut right_border = false;
        for line in in_region {
            if (line[x] && (x == 0 || !line[x - 1])) != left_border {
                left_border = !left_border;
                if left_border {
                    sides += 1;
                }
            }
            if (line[x] && (x >= line.len() - 1 || !line[x + 1])) != right_border {
                right_border = !right_border;
                if right_border {
                    sides += 1;
                }
            }
        }
    }
    sides
}
