use std::iter::repeat;

pub fn solve(input: &str) -> usize {
    let input = input.trim();
    let mut disk = Vec::with_capacity(
        input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .sum(),
    );
    for (i, c) in input.chars().enumerate() {
        let count = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            disk.extend(repeat(Some(i / 2)).take(count));
        } else {
            disk.extend(repeat(None).take(count));
        }
    }
    print_disk(&disk);
    for i in 0..disk.len() {
        if i >= disk.len() {
            break;
        }
        if disk[i].is_some() {
            continue;
        }
        while disk.last().unwrap().is_none() {
            disk.pop();
        }
        if i >= disk.len() {
            println!("bweeh");
            break;
        }
        disk.swap_remove(i);
    }
    print_disk(&disk);
    disk.into_iter()
        .map(Option::unwrap)
        .enumerate()
        .map(|(i, n)| i * n)
        .sum()
}

#[allow(dead_code)]
fn print_disk(disk: &[Option<usize>]) {
    for e in disk {
        match e {
            None => print!("."),
            Some(n) => print!("{}", n % 10),
        }
    }
    println!();
}
