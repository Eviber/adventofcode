#[derive(Clone, Copy)]
pub struct Secret {
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

