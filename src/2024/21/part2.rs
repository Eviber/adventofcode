use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let mut memo =  HashMap::new();
    input
        .lines()
        .map(|s| {
            let n: usize = s[..(s.len() - 1)].parse().unwrap();
            let len = {
                let kp = Keypad::from(s);
                let commands = kp.into_commands();
                commands_len('A', &commands, 25, &mut memo)
            };
            n * len
        })
        .sum()
}

fn commands_len(origin: char, commands: &str, depth: i32, memo: &mut HashMap<(char, String, i32), usize>) -> usize {
    if commands.is_empty() {
        return 0;
    }
    if depth == 0 {
        return commands.len();
    }
    if let Some(&count) = memo.get(&(origin, commands.to_string(), depth)) {
        return count;
    }
    if commands.len() > 1 {
        let len = commands_len(origin, &commands[0..1], depth, memo) + commands_len(commands.chars().next().unwrap(), &commands[1..], depth, memo);
        memo.insert((origin, commands.to_string(), depth), len);
        return len;
    }
    let commands2 = Keypad::new(origin, commands, KeypadType::Directional).into_commands();
    let len = commands_len('A', &commands2, depth - 1, memo);
    memo.insert((origin, commands.to_string(), depth), len);
    len
}

struct Keypad {
    x: usize,
    y: usize,
    code: Vec<char>,
    kind: KeypadType,
}

#[derive(Clone, Copy)]
enum KeypadType {
    Digital,
    Directional,
}

fn pos_digital(c: char) -> (usize, usize) {
    if c == ' ' {
        return (0, 3);
    }
    if c == 'A' {
        return (2, 3);
    }
    let n = c.to_digit(10).expect("c should be a digit") as usize;
    match n {
        7..=9 => (n - 7, 0),
        4..=6 => (n - 4, 1),
        1..=3 => (n - 1, 2),
        0 => (1, 3),
        _ => unreachable!(),
    }
}

fn pos_directional(c: char) -> (usize, usize) {
    match c {
        ' ' => (0, 0),
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!("invalid direction"),
    }
}

fn pos(kind: KeypadType, c: char) -> (usize, usize) {
    match kind {
        KeypadType::Digital => pos_digital(c),
        KeypadType::Directional => pos_directional(c),
    }
}

impl Keypad {
    fn new(origin: char, command: &str, kind: KeypadType) -> Self {
        let code: Vec<char> = command.chars().collect();
        let (x, y) = pos(kind, origin);
        Keypad { x, y, code, kind }
    }

    fn pos(&self, c: char) -> (usize, usize) {
        pos(self.kind, c)
    }

    fn into_commands(mut self) -> String {
        let blank = self.pos(' ');
        let mut commands = Vec::new();
        for &c in &self.code {
            let (target_x, target_y) = self.pos(c);
            while self.x != target_x || self.y != target_y {
                while self.x > target_x
                    && (self.x < blank.0
                        || target_x > blank.0
                        || self.y == target_y
                        || self.y != blank.1)
                {
                    self.x -= 1;
                    assert!((self.x, self.y) != blank);
                    commands.push('<');
                }
                while self.y > target_y
                    && (self.y < blank.1
                        || target_y > blank.1
                        || self.x == target_x
                        || self.x != blank.0)
                {
                    self.y -= 1;
                    assert!((self.x, self.y) != blank);
                    commands.push('^');
                }
                while self.y < target_y
                    && (self.y > blank.1
                        || target_y < blank.1
                        || self.x == target_x
                        || self.x != blank.0)
                {
                    self.y += 1;
                    assert!((self.x, self.y) != blank);
                    commands.push('v');
                }
                while self.x < target_x
                    && (self.x > blank.0
                        || target_x < blank.0
                        || self.y == target_y
                        || self.y != blank.1)
                {
                    self.x += 1;
                    assert!((self.x, self.y) != blank);
                    commands.push('>');
                }
            }
            commands.push('A');
        }
        commands.into_iter().collect()
    }
}

impl From<&str> for Keypad {
    fn from(s: &str) -> Self {
        let code: Vec<char> = s.chars().collect();
        let kind = if code[0].is_ascii_digit() || code[1].is_ascii_digit() {
            KeypadType::Digital
        } else {
            KeypadType::Directional
        };
        let (x, y) = pos(kind, 'A');
        Keypad { x, y, code, kind }
    }
}

impl From<String> for Keypad {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}
