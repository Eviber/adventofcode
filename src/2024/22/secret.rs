pub use i64 as int;

#[derive(Clone, Copy)]
pub struct Secret {
    value: int,
}

impl Iterator for Secret {
    type Item = int;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.value;
        *self = self.mix(*self * 64).prune();
        *self = self.mix(*self / 32).prune();
        *self = self.mix(*self * 2048).prune();
        Some(value)
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

impl std::ops::Mul<int> for Secret {
    type Output = Secret;

    fn mul(self, rhs: int) -> Self::Output {
        Secret {
            value: self.value * rhs,
        }
    }
}

impl std::ops::Div<int> for Secret {
    type Output = Secret;

    fn div(self, rhs: int) -> Self::Output {
        Secret {
            value: self.value / rhs,
        }
    }
}

impl From<int> for Secret {
    fn from(value: int) -> Self {
        Secret { value }
    }
}

impl From<&str> for Secret {
    fn from(s: &str) -> Self {
        s.trim().parse::<int>().unwrap().into()
    }
}

