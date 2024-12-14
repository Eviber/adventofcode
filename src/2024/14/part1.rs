use glam::IVec2;

pub fn solve<const W: i32, const H: i32>(input: &str) -> usize {
    let robots = input
        .lines()
        .map(|s| -> Robot<W, H> { Robot::from(s) })
        .map(|r| r.move_for(100))
        .map(|r| r.pos)
        .filter_map(get_quadrant::<W, H>);
    let mut quadrants = [0; 4];
    for r in robots {
        quadrants[r] += 1;
    }
    quadrants.into_iter().product()
}

fn get_quadrant<const W: i32, const H: i32>(p: IVec2) -> Option<usize> {
    if p.x < W / 2 && p.y < H / 2 {
        Some(0)
    } else if p.x > W / 2 && p.y < H / 2 {
        Some(1)
    } else if p.x < W / 2 && p.y > H / 2 {
        Some(2)
    } else if p.x > W / 2 && p.y > H / 2 {
        Some(3)
    } else {
        None
    }
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
    fn move_for(mut self, seconds: i32) -> Self {
        self.pos += self.vel * seconds;
        self.pos.x = wrap(self.pos.x, W);
        self.pos.y = wrap(self.pos.y, H);
        self
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
