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
        .filter(|(test_value, equation)| can_be_solved(equation[0], &equation[1..], *test_value))
        .map(|(value, _)| value)
        .sum()
}

fn can_be_solved(current: usize, equation: &[usize], test_value: usize) -> bool {
    if equation.is_empty() && current == test_value {
        return true;
    }
    if equation.is_empty() || current > test_value {
        return false;
    }
    let sum = current + equation[0];
    let product = current * equation[0];
    can_be_solved(sum, &equation[1..], test_value)
        || can_be_solved(product, &equation[1..], test_value)
}
