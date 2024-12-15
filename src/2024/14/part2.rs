use glam::IVec2;

pub fn solve<const W: i32, const H: i32>(input: &str) -> usize {
    let mut robots: Vec<Robot<W, H>> = input
        .lines()
        .map(|s| -> Robot<W, H> { Robot::from(s) })
        .collect();

    let mut i = 0;
    loop {
        let mut map = vec![vec![false; W as usize]; H as usize];
        for robot in &mut robots {
            let x = robot.pos.x as usize;
            let y = robot.pos.y as usize;
            map[y][x] = true;
            robot.move_for(1);
        }
        if all_linked(&map) {
            for line in &map {
                for &cell in line {
                    if cell {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
            println!();
            break i;
        }
        i += 1
    }
}

fn all_linked(map: &[Vec<bool>]) -> bool {
    const R: usize = 3;
    for lines in map.windows(R) {
        for x in 0..map[0].len() {
            let mut test = true;
            for line in lines {
                for b in line.iter().take(x).skip(x-R) {
                    test &= b;
                }
            }
            if test {
                return true;
            }
        }
    }
    false
}

fn wrap(n: i32, max: i32) -> i32 {
    ((n % max) + max) % max
}

#[derive(Clone, Copy)]
struct Robot<const W: i32, const H: i32> {
    pos: IVec2,
    vel: IVec2,
}

impl<const W: i32, const H: i32> Robot<W, H> {
    fn move_for(&mut self, seconds: i32) {
        self.pos += self.vel * seconds;
        self.pos.x = wrap(self.pos.x, W);
        self.pos.y = wrap(self.pos.y, H);
    }
}

impl<const W: i32, const H: i32> From<&str> for Robot<W, H> {
    fn from(s: &str) -> Self {
        let (p, v) = s.split_once(' ').expect("line shoud have a space");
        let p = p.strip_prefix("p=").expect("line shoud start with 'p='");
        let (x, y) = p.split_once(',').expect("p should have a ','");
        let x: i32 = x.parse().expect("p.x should be a valid number");
        let y: i32 = y.parse().expect("p.y should be a valid number");
        let pos = IVec2::new(x, y);
        let v = v.strip_prefix("v=").expect("line shoud have 'v='");
        let (x, y) = v.split_once(',').expect("v should have a ','");
        let x: i32 = x.parse().expect("v.x should be a valid number");
        let y: i32 = y.parse().expect("v.y should be a valid number");
        let vel = IVec2::new(x, y);
        Self { pos, vel }
    }
}
