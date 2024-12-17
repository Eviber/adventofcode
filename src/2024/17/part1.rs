pub fn solve(input: &str) -> Vec<i32> {
    let mut vm = VM::from(input);
    vm.run()
}

#[derive(Debug)]
struct VM {
    instructions: Vec<i32>,
    ip: usize,
    a: i32,
    b: i32,
    c: i32,
}

impl VM {
    fn run(&mut self) -> Vec<i32> {
        let mut output = vec![];
        while self.step(&mut output) {}
        output
    }

    fn step(&mut self, output: &mut Vec<i32>) -> bool {
        if self.ip + 1 > self.instructions.len() {
            return false;
        }
        let instruction = self.instructions[self.ip];
        let operand = self.instructions[self.ip + 1];
        assert!((0..=7).contains(&instruction));
        assert!((0..=7).contains(&operand));
        match instruction {
            0 => self.a /= 2_i32.pow(self.combo(operand) as u32),
            1 => self.b ^= operand,
            2 => self.b = self.combo(operand) % 8,
            3 => {
                if self.a != 0 {
                    self.ip = operand as usize;
                    return true;
                }
            }
            4 => self.b ^= self.c,
            5 => output.push(self.combo(operand) % 8),
            6 => self.b = self.a / 2_i32.pow(self.combo(operand) as u32),
            7 => self.c = self.a / 2_i32.pow(self.combo(operand) as u32),
            _ => unreachable!(),
        }
        self.ip += 2;
        true
    }

    fn combo(&self, operand: i32) -> i32 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("7 should not appear as a combo operand"),
            _ => unreachable!(),
        }
    }
}

impl From<&str> for VM {
    fn from(s: &str) -> Self {
        let (reg, program) = s.trim().split_once("\n\n").expect("empty line");
        let mut reg = reg.lines();
        let a = get_reg(&mut reg);
        let b = get_reg(&mut reg);
        let c = get_reg(&mut reg);
        let instructions = program[9..]
            .split(',')
            .map(|s| s.parse::<i32>().expect("instruction"))
            .collect();
        VM {
            instructions,
            ip: 0,
            a,
            b,
            c,
        }
    }
}

fn get_reg(reg: &mut std::str::Lines<'_>) -> i32 {
    reg.next().expect("line")[12..].parse().expect("number")
}

#[test]
fn test_1() {
    let input = "Register A: 0
Register B: 0
Register C: 9

Program: 2,6
";
    let mut vm = VM::from(input);
    vm.run();
    assert_eq!(vm.b, 1)
}

#[test]
fn test_3() {
    let input = "Register A: 1024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
    let mut vm = VM::from(input);
    vm.run();
    assert_eq!(vm.a, 0)
}

#[test]
fn test_4() {
    let input = "Register A: 0
Register B: 29
Register C: 0

Program: 1,7
";
    let mut vm = VM::from(input);
    vm.run();
    assert_eq!(vm.b, 26)
}

#[test]
fn test_5() {
    let input = "Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0
";
    let mut vm = VM::from(input);
    vm.run();
    assert_eq!(vm.b, 44354)
}
