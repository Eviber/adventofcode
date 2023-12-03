fn main() {
    let input = include_str!("input");
    let sum = compute_sum(input);
    println!("{sum}");
    let ratio_sum = compute_ratio(input);
    println!("{ratio_sum}");
}

fn compute_ratio(input: &str) -> u32 {
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut debug_output = vec![vec!['.'; input[0].len()]; input.len()];

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x].is_ascii_digit() {
                debug_output[y][x] = input[y][x];
            }
        }
    }

    let mut sum = 0;
    for (y, line) in input.iter().enumerate() {
        for x in (0..line.len()).filter(|&x| line[x] == '*') {
            let product = product_if_adjacent_to_n_numbers(&input, x, y, 2);
            if product != 0 {
                debug_output[y][x] = '*';
            }
            sum += product;
        }
    }
    for line in debug_output {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
    println!();
    sum
}

fn product_if_adjacent_to_n_numbers(input: &[Vec<char>], x: usize, y: usize, n: i32) -> u32 {
    let start_y = if y > 0 { y - 1 } else { y };
    let end_y = if y + 1 < input.len() { y + 1 } else { y };
    let start_x = if x > 0 { x - 1 } else { x };
    let end_x = if x + 1 < input[0].len() { x + 1 } else { x };

    let mut count = 0;
    let mut product = 1;
    for y in start_y..=end_y {
        let mut in_number = false;
        for x in start_x..=end_x {
            if input[y][x].is_ascii_digit() {
                if !in_number {
                    in_number = true;
                    count += 1;
                    product *= extract_number(input, x, y);
                }
            } else {
                in_number = false;
            }
        }
    }
    if count != n {
        return 0;
    }
    product
}

fn extract_number(input: &[Vec<char>], mut x: usize, y: usize) -> u32 {
    while x > 0 && input[y][x - 1].is_ascii_digit() {
        x -= 1;
    }
    let mut n = 0;
    while x < input[y].len() && input[y][x].is_ascii_digit() {
        n = n * 10 + input[y][x].to_digit(10).unwrap();
        x += 1;
    }
    n
}

fn compute_sum(input: &str) -> u32 {
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut sum = 0;
    for (y, line) in input.iter().enumerate() {
        for x in (0..line.len())
            .filter(|&x| x == 0 || (!line[x - 1].is_ascii_digit() && line[x].is_ascii_digit()))
        {
            let mut end = x;
            while end < line.len() && line[end].is_ascii_digit() {
                end += 1;
            }
            if is_adjacent_to_symbol(&input, x, y, end) {
                let val: u32 = line[x..end]
                    .iter()
                    .cloned()
                    .collect::<String>()
                    .parse()
                    .unwrap();
                sum += val;
            }
        }
    }
    sum
}

fn is_adjacent_to_symbol(input: &[Vec<char>], mut x: usize, y: usize, mut end: usize) -> bool {
    if x > 0 && input[y][x - 1] != '.' {
        return true;
    }
    if end < input[y].len() && input[y][end] != '.' {
        return true;
    }
    x = x.saturating_sub(1);
    if end < input[y].len() - 1 {
        end += 1;
    }
    if y > 0 {
        for x in x..end {
            let c = input[y - 1][x];
            if c != '.' && !c.is_ascii_digit() {
                return true;
            }
        }
    }
    if y < input.len() - 1 {
        for x in x..end {
            let c = input[y + 1][x];
            if c != '.' && !c.is_ascii_digit() {
                return true;
            }
        }
    }
    false
}

#[test]
fn test() {
    assert_eq!(compute_sum(include_str!("input_test")), 4361);
}

#[test]
fn test2() {
    assert_eq!(compute_ratio(include_str!("input_test")), 467835);
}
