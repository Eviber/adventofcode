pub fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(4, 6 => 2)]
    #[test_case(18, 24 => 6)]
    #[test_case(44, 55 => 11)]
    fn test_gcd(a: u64, b: u64) -> u64 {
        gcd(a, b)
    }

    #[test_case(4, 6 => 12)]
    #[test_case(18, 24 => 72)]
    #[test_case(44, 55 => 220)]
    fn test_lcm(a: u64, b: u64) -> u64 {
        lcm(a, b)
    }
}
