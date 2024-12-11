use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let input: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut memo = HashMap::new();
    blink(&input, 75, &mut memo)
}

fn blink(stones: &[u64], count: u64, memo: &mut HashMap<(u64, u64), usize>) -> usize {
    if count == 0 {
        return stones.len();
    }
    if stones.is_empty() {
        return 0;
    }
    if stones.len() > 1 {
        return blink(&stones[0..(stones.len() - 1)], count, memo)
            + blink(&stones[(stones.len() - 1)..stones.len()], count, memo);
    }
    let stone = stones[0];
    if let Some(&res) = memo.get(&(stone, count)) {
        return res;
    }
    let stones = blink_one(stone);
    let res = blink(&stones, count - 1, memo);
    memo.insert((stone, count), res);
    res
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
