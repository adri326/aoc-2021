use itertools::Itertools;

const DAY: u32 = 4;

#[derive(Clone, Debug)]
struct Board {
    pub numbers: [u32; 25],
    pub checked: [bool; 25],
}

impl Board {
    pub fn mark(&mut self, value: u32) -> bool {
        for (index, &num) in self.numbers.iter().enumerate() {
            if num != value {
                continue;
            }

            self.checked[index] = true;

            // Check for a win and return if yes
            return self.won();
        }

        false
    }

    pub fn score(&self) -> u64 {
        let mut res = 0;
        for n in 0..25 {
            if !self.checked[n] {
                res += self.numbers[n] as u64;
            }
        }
        res
    }

    pub fn won(&self) -> bool {
        for n in 0..5 {
            // Check row n
            let mut wins = true;
            for o in 0..5 {
                wins = wins && self.checked[n * 5 + o];
            }
            if wins {
                return true;
            }

            // Check column n
            wins = true;
            for o in 0..5 {
                wins = wins && self.checked[n + o * 5];
            }
            if wins {
                return true;
            }
        }

        false
    }
}

fn parse(file: String) -> (Vec<u32>, Vec<Board>) {
    let mut iter = file.split("\n");
    let first_line = iter.next().unwrap();

    let mut numbers: Vec<u32> = Vec::new();
    for raw_number in first_line.split(",") {
        numbers.push(raw_number.parse().unwrap());
    }

    let mut boards: Vec<Board> = Vec::new();
    for lines in iter.chunks(6).into_iter() {
        let lines = lines.filter(|l| *l != "").collect::<Vec<_>>();
        if lines.len() == 0 {
            break
        }
        assert_eq!(lines.len(), 5);

        let mut board = Board {
            numbers: [0; 25],
            checked: [false; 25],
        };

        for (y, line) in lines.into_iter().enumerate() {
            for (x, raw_number) in line.split_whitespace().filter(|x| *x != "").enumerate() {
                board.numbers[x + y * 5] = raw_number.parse().unwrap();
            }
        }

        boards.push(board);
    }

    (numbers, boards)
}

fn part_1((numbers, mut boards): (Vec<u32>, Vec<Board>)) -> u64 {
    for number in numbers {
        for board in boards.iter_mut() {
            if board.mark(number) {
                return board.score() * number as u64;
            }
        }
    }

    0
}

fn part_2((numbers, mut boards): (Vec<u32>, Vec<Board>)) -> u64 {
    for number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);
        }
        if boards.len() == 1 {
            if boards[0].won() {
                return boards[0].score() * number as u64;
            }
        } else {
            boards.retain(|board| !board.won());
        }
    }

    0
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
