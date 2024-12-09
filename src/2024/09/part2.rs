use std::iter::repeat;

pub fn solve(input: &str) -> usize {
    let input = input.trim();
    let mut disk = Vec::new();
    for (i, c) in input.chars().enumerate() {
        let count = c.to_digit(10).unwrap() as usize;
        if count == 0 {
            continue;
        }
        if i % 2 == 0 {
            disk.push((count, Some(i / 2)));
        } else {
            disk.push((count, None));
        }
    }
    let mut current = disk.last().unwrap().1.unwrap();
    loop {
        let i = disk.iter().take_while(|&&(_, id)| id != Some(current)).count();
        let size = disk[i].0;
        let mut space = 0;
        let mut j = 0;
        while space < size {
            while j < i && disk[j].1.is_some() {
                j += 1;
            }
            if j >= i {
                break;
            }
            space = disk[j].0;
            if space < size  {
                j += 1;
            }
        }
        if size <= space && j < i {
            let element = disk[i];
            disk[i].1 = None;
            disk[j].0 -= element.0;
            disk.insert(j, element);
        }
        if current == 0 {
            break;
        }
        current -= 1;
    }
    disk.into_iter()
        .flat_map(|e| repeat(e.1).take(e.0))
        .enumerate()
        .map(|(i, o)| (i, o.unwrap_or(0)))
        .map(|(i, n)| i * n)
        .sum()
}

#[allow(dead_code)]
fn print_disk(disk: &[(usize, Option<usize>)]) {
    for &(count, o) in disk {
        for _ in 0..count {
            match o {
                None => print!("."),
                Some(n) => print!("{}", n % 10),
            }
        }
    }
    println!();
}
