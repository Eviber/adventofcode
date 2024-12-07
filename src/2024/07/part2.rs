pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.split_once(": ").unwrap())
        .map(|(s_test, s_eq)| {
            let test_value: usize = s_test.parse().unwrap();
            let equation: Vec<usize> = s_eq
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            (test_value, equation)
        })
        .filter(|(test_value, equation)| can_be_solved(0, equation, *test_value))
        .map(|(value, _)| value)
        .sum()
}

fn can_be_solved(mut current: usize, mut equation: &[usize], test_value: usize) -> bool {
    if current == 0 {
        current = equation[0];
        equation = &equation[1..];
    }
    if equation.is_empty() && current == test_value {
        return true;
    }
    if equation.is_empty() || current > test_value {
        return false;
    }
    let sum = current + equation[0];
    let product = current * equation[0];
    let concat = concatenate(current, equation[0]);
    let equation = &equation[1..];
    can_be_solved(sum, equation, test_value)
        || can_be_solved(product, equation, test_value)
        || can_be_solved(concat, equation, test_value)
}

fn concatenate(a: usize, b: usize) -> usize {
    a * 10usize.pow(b.ilog10() + 1) + b
}
