const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_starting_number(line: &str) -> Option<u32> {
    DIGITS
        .iter()
        .enumerate()
        .find(|(_, w)| line.starts_with(**w))
        .map(|(n, _)| n as u32 + 1)
}

fn get_digit(line: &str, iter: impl Iterator<Item = usize>) -> u32 {
    for i in iter {
        let c = line.chars().nth(i).expect("iter should be in bounds");
        if c.is_ascii_digit() {
            return c.to_digit(10).expect("c is an ascii digit");
        }
        let line = &line[i..];
        if let Some(n) = get_starting_number(line) {
            return n;
        }
    }
    panic!("No digit found");
}

fn extract_value(line: &str) -> u32 {
    let first = get_digit(line, 0..line.len());
    let second = get_digit(line, (0..line.len()).rev());
    first * 10 + second
}

fn main() {
    let input = include_str!("input");

    let sum: u32 = input.lines().map(extract_value).sum();
    println!("{}", sum);
    assert_eq!(sum, 55260);
}

#[cfg(test)]
mod tests {
    use super::extract_value;
    use test_case::test_case;

    #[test_case("1abc2" => 12)]
    #[test_case("pqr3stu8vwx" => 38)]
    #[test_case("a1b2c3d4e5f" => 15)]
    #[test_case("treb7uchet" => 77)]
    fn line_digit(line: &str) -> u32 {
        extract_value(line)
    }

    #[test_case("two1nine" => 29)]
    #[test_case("eightwothree" => 83)]
    #[test_case("abcone2threexyz" => 13)]
    #[test_case("xtwone3four" => 24)]
    #[test_case("4nineeightseven2" => 42)]
    #[test_case("zoneight234" => 14)]
    #[test_case("7pqrstsixteen" => 76)]
    fn line_spelled_digit(line: &str) -> u32 {
        extract_value(line)
    }
}
