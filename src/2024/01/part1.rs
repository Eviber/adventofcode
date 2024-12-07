pub fn solve(input: &str) -> usize {
    let (mut a, mut b): (Vec<usize>, Vec<usize>) = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (a, b) = s.split_once(' ').expect("space in input");
            let a: usize = a.trim().parse().expect("correctly formatted number");
            let b: usize = b.trim().parse().expect("correctly formatted number");
            (a, b)
        })
        .collect();
    a.sort_unstable();
    b.sort_unstable();
    a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum()
}
