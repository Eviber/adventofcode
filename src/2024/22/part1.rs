pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(Secret::from)
        .map(|mut secret| secret.nth(1999).unwrap())
        .sum()
}

#[derive(Clone, Copy)]
struct Secret {
    value: usize,
}

impl Iterator for Secret {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        *self = self.mix(*self * 64).prune();
        *self = self.mix(*self / 32).prune();
        *self = self.mix(*self * 2048).prune();
        Some(self.value)
    }
}

impl Secret {
    fn mix(self, n: Secret) -> Secret {
        (self.value ^ n.value).into()
    }

    fn prune(self) -> Secret {
        (self.value % 16777216).into()
    }
}

impl std::ops::Mul<usize> for Secret {
    type Output = Secret;

    fn mul(self, rhs: usize) -> Self::Output {
        Secret {
            value: self.value * rhs,
        }
    }
}

impl std::ops::Div<usize> for Secret {
    type Output = Secret;

    fn div(self, rhs: usize) -> Self::Output {
        Secret {
            value: self.value / rhs,
        }
    }
}

impl From<usize> for Secret {
    fn from(value: usize) -> Self {
        Secret { value }
    }
}

impl From<&str> for Secret {
    fn from(s: &str) -> Self {
        s.trim().parse::<usize>().unwrap().into()
    }
}

#[test]
fn part1() {
    let input = "1
10
100
2024";
    assert_eq!(solve(input), 37327623);
}

#[test]
fn part1_1() {
    let input = "1";
    assert_eq!(solve(input), 8685429);
}

#[test]
fn part1_10() {
    let input = "10";
    assert_eq!(solve(input), 4700978);
}

#[test]
fn part1_100() {
    let input = "100";
    assert_eq!(solve(input), 15273692);
}

#[test]
fn part1_2024() {
    let input = "2024";
    assert_eq!(solve(input), 8667524);
}
