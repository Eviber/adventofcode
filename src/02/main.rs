const RED_LIMIT: u32 = 12;
const GREEN_LIMIT: u32 = 13;
const BLUE_LIMIT: u32 = 14;

fn main() {
    let input = include_str!("input");
    let mut sum_possible = 0;
    let mut sum_minimum_powers = 0;
    for line in input.lines() {
        let mut split = line.split(':');
        let id: u32 = (&(split.next().expect("no id"))[5..])
            .parse()
            .expect("parsing failed");
        let split = split.next().unwrap().split(';');
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for handful in split {
            let colors = handful.split(',');
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for color in colors {
                let mut split = color.trim().split(' ');
                let amount: u32 = split.next().unwrap().parse().unwrap();
                let color = split.next().unwrap();
                match color {
                    "red" => red = amount,
                    "green" => green = amount,
                    "blue" => blue = amount,
                    _ => panic!("unknown color"),
                }
            }
            max_red = red.max(max_red);
            max_green = green.max(max_green);
            max_blue = blue.max(max_blue);
        }
        sum_minimum_powers += max_red * max_green * max_blue;
        if max_red <= RED_LIMIT && max_green <= GREEN_LIMIT && max_blue <= BLUE_LIMIT {
            sum_possible += id;
        }
    }
    println!("sum of possible ids: {}", sum_possible);
    println!("sum of minimum powers: {}", sum_minimum_powers);
}
