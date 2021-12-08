use itertools::Itertools;

const DAY: u32 = 5;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

#[derive(Clone, Copy, Debug)]
struct Vent(usize, usize, usize, usize); // sx, sy, dx, dy

fn parse(raw: String) -> Vec<Vent> {
    let mut res = Vec::new();

    for line in raw.split("\n").filter(|l| *l != "") {
        if let [left, right] = line.split(" -> ").collect::<Vec<_>>()[0..2] {
            if let [left_x, left_y] = left.split(",").collect::<Vec<_>>()[0..2] {
                if let [right_x, right_y] = right.split(",").collect::<Vec<_>>()[0..2] {
                    res.push(Vent(
                        left_x.parse().unwrap(),
                        left_y.parse().unwrap(),
                        right_x.parse().unwrap(),
                        right_y.parse().unwrap(),
                    ));
                }
            }
        }
    }

    res
}

fn part_1(vents: impl AsRef<[Vent]>) -> usize {
    let mut image = vec![0u8; WIDTH * HEIGHT]; // Oof
    let mut res = 0;

    for vent in vents.as_ref().iter() {
        if vent.0 == vent.2 { // sx == dx, scan vertically
            let min_y = vent.1.min(vent.3);
            let max_y = vent.1.max(vent.3);
            for y in min_y..=max_y {
                let index = vent.0 + WIDTH * y;
                if image[index] == 1 {
                    res += 1;
                }
                image[index] += 1;
            }
        } else if vent.1 == vent.3 { // sy == dy, scan horizontally
            let min_x = vent.0.min(vent.2);
            let max_x = vent.0.max(vent.2);
            for x in min_x..=max_x {
                let index = x + WIDTH * vent.1;
                if image[index] == 1 {
                    res += 1;
                }
                image[index] += 1;
            }
        }
    }

    res
}

fn part_2(vents: impl AsRef<[Vent]>) -> usize {
    let mut image = vec![0u8; WIDTH * HEIGHT]; // Oof
    let mut res = 0;

    for vent in vents.as_ref().iter() {
        if vent.0 == vent.2 { // sx == dx, scan vertically
            let min_y = vent.1.min(vent.3);
            let max_y = vent.1.max(vent.3);
            for y in min_y..=max_y {
                let index = vent.0 + WIDTH * y;
                if image[index] == 1 {
                    res += 1;
                }
                image[index] += 1;
            }
        } else if vent.1 == vent.3 { // sy == dy, scan horizontally
            let min_x = vent.0.min(vent.2);
            let max_x = vent.0.max(vent.2);
            for x in min_x..=max_x {
                let index = x + WIDTH * vent.1;
                if image[index] == 1 {
                    res += 1;
                }
                image[index] += 1;
            }
        } else {
            let min_x = vent.0.min(vent.2);
            let max_x = vent.0.max(vent.2);
            let min_y = vent.1.min(vent.3);
            let max_y = vent.1.max(vent.3);

            let length = max_x - min_x;

            let dir = (vent.2 > vent.0) == (vent.3 > vent.1);

            let base_index = if dir {
                min_x + min_y * WIDTH
            } else {
                max_x + min_y * WIDTH
            };

            for n in 0..=length {
                let index = if dir {
                    base_index + n * (1 + WIDTH)
                } else {
                    base_index + n * (WIDTH - 1)
                };

                if image[index] == 1 {
                    res += 1;
                }
                image[index] += 1;
            }
        }
    }

    res
}

fn main() {
    let sample_file = format!("./sample/{:02}.txt", DAY);
    let sample = parse(std::fs::read_to_string(sample_file).unwrap());

    let input_file = format!("./input/{:02}.txt", DAY);
    let input = std::fs::read_to_string(input_file).ok().map(|str| parse(str));

    println!("[04-1] Sample: {}", part_1(sample.clone()));
    println!("[04-2] Sample: {}", part_2(sample));

    if let Some(input) = input {
        println!("[04-1] Input: {}", part_1(input.clone()));
        println!("[04-2] Input: {}", part_2(input));
    }
}
