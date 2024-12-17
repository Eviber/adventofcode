pub fn solve(input: &str) -> u64 {
    let vm = VM::from(input);
    let mut a = 1;
    loop {
        let mut vm = vm.clone();
        vm.a = a;
        let res = vm.run();
        if res == vm.instructions {
            return a;
        }
        assert!(res.len() <= vm.instructions.len());
        if res.len() < vm.instructions.len() {
            a *= 8;
            continue;
        }
        if res == vm.instructions {
            return a;
        }
        for i in (0..res.len()).rev() {
            if res[i] != vm.instructions[i] {
                a += 8u64.pow(i as u32);
                break;
            }
        }
    }
}

#[derive(Clone, Debug)]
struct VM {
    instructions: Vec<u64>,
    ip: usize,
    a: u64,
    b: u64,
    c: u64,
}

impl VM {
    fn run(&mut self) -> Vec<u64> {
        let mut output = vec![];
        while self.step(&mut output) {}
        output
    }

    fn step(&mut self, output: &mut Vec<u64>) -> bool {
        if self.ip + 1 >= self.instructions.len() {
            return false;
        }
        let instruction = self.instructions[self.ip];
        let operand = self.instructions[self.ip + 1];
        assert!((0..=7).contains(&instruction));
        assert!((0..=7).contains(&operand));
        match instruction {
            0 => self.a /= 2_u64.pow(self.combo(operand) as u32),
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
            6 => self.b = self.a / 2_u64.pow(self.combo(operand) as u32),
            7 => self.c = self.a / 2_u64.pow(self.combo(operand) as u32),
            _ => unreachable!(),
        }
        self.ip += 2;
        true
    }

    fn combo(&self, operand: u64) -> u64 {
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
            .map(|s| s.parse::<u64>().expect("instruction"))
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

fn get_reg(reg: &mut std::str::Lines<'_>) -> u64 {
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
