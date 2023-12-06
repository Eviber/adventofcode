use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub start: u64,
    pub length: u64,
}

impl Range {
    #[inline]
    pub fn new(start: u64, length: u64) -> Self {
        Self { start, length }
    }

    #[inline]
    pub fn from_to(start: u64, end: u64) -> Self {
        Self {
            start,
            length: end - start,
        }
    }

    #[inline]
    pub fn end(&self) -> u64 {
        self.start + self.length
    }

    #[inline]
    pub fn contains(&self, input: u64) -> bool {
        self.start <= input && input < self.end()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rule {
    dst: Range,
    src: Range,
}

impl Rule {
    pub fn new(dst_start: u64, src_start: u64, length: u64) -> Self {
        Self {
            dst: Range::new(dst_start, length),
            src: Range::new(src_start, length),
        }
    }

    #[allow(dead_code)]
    pub fn apply(&self, input: u64) -> Option<u64> {
        if self.src.contains(input) {
            return Some(self.dst.start + (input - self.src.start));
        }
        None
    }

    pub fn apply_range(&self, input: Range) -> (Option<Range>, Option<Range>, Option<Range>) {
        if !self.src.contains(input.start) && !self.src.contains(input.end()) {
            return (None, None, None);
        }
        let before = if input.start < self.src.start {
            Some(Range::from_to(input.start, self.src.start.min(input.end())))
        } else {
            None
        };
        let after = if input.end() > self.src.end() {
            Some(Range::from_to(self.src.end().max(input.start), input.end()))
        } else {
            None
        };
        let matched = if self.src.contains(input.start) || self.src.contains(input.end()) {
            let mut range = Range::from_to(
                input.start.max(self.src.start),
                input.end().min(self.src.end()),
            );
            range.start = range.start + self.dst.start - self.src.start;
            Some(range)
        } else {
            None
        };
        (before, matched, after)
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
