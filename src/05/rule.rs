use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Rule {
    dst_start: u64,
    src_start: u64,
    length: u64,
}

impl Rule {
    pub fn new(dst_start: u64, src_start: u64, length: u64) -> Self {
        Self {
            dst_start,
            src_start,
            length,
        }
    }

    pub fn apply(&self, input: u64) -> Option<u64> {
        if self.src_start <= input && input < self.src_start + self.length {
            return Some(self.dst_start + (input - self.src_start));
        }
        None
    }
}

impl FromStr for Rule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.trim().split_whitespace().map(|s| s.parse::<u64>());
        Ok(Self::new(
            values
                .next()
                .ok_or("no dst")?
                .map_err(|_| "dst parse error")?,
            values
                .next()
                .ok_or("no src")?
                .map_err(|_| "src parse error")?,
            values
                .next()
                .ok_or("no len")?
                .map_err(|_| "len parse error")?,
        ))
    }
}
