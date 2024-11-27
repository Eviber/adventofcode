pub fn solve(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    let steps = input.trim_end().split(',').map(Step::new);
    for step in steps {
        let b = &mut boxes[hash(step.label)];
        match step.operation {
            Operation::Remove => {
                if let Some(i) = b.iter().position(|(s, _)| *s == step.label) {
                    b.remove(i);
                };
            }
            Operation::Add(lens) => {
                if let Some(i) = b.iter().position(|(s, _)| *s == step.label) {
                    b[i] = (step.label, lens);
                } else {
                    b.push((step.label, lens));
                }
            }
        }
    }
    boxes
        .into_iter()
        .enumerate()
        .filter(|(_, b)| !b.is_empty())
        .flat_map(|(i, b)| {
            b.into_iter()
                .enumerate()
                .map(move |(j, (_, lens))| (i + 1) * (j + 1) * lens)
        })
        .sum()
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

struct Step<'a> {
    label: &'a str,
    operation: Operation,
}
enum Operation {
    Remove,
    Add(usize),
}

impl<'a> Step<'a> {
    fn new(s: &'a str) -> Self {
        let n = s.find(['-', '=']).expect("operator");
        let label = &s[..n];
        let operation = match &s[n..n + 1] {
            "-" => Operation::Remove,
            "=" => Operation::Add(s[n + 1..].parse().expect("lens value")),
            _ => unreachable!(),
        };
        Step { label, operation }
    }
}
