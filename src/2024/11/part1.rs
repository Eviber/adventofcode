pub fn solve(input: &str) -> usize {
    let input: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    blink(&input, 25)
}

fn blink(stones: &[u64], count: i32) -> usize {
    if count == 0 {
        return stones.len();
    }
    if stones.is_empty() {
        return 0;
    }
    if stones.len() == 1 {
        let stones = blink_one(stones[0]);
        return blink(&stones, count - 1);
    }
    blink(&stones[0..(stones.len() - 1)], count) + blink(&stones[(stones.len() - 1)..stones.len()], count)
}

fn blink_one(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }
    let len = stone.ilog10() + 1;
    if len % 2 == 0 {
        let half_pow = 10_u64.pow(len / 2);
        vec![stone / half_pow, stone % half_pow]
    } else {
        vec![stone * 2024]
    }
}
