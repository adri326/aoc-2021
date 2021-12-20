use itertools::Itertools;

const DAY: u32 = 15;

#[derive(Debug, Clone, PartialEq)]
struct Matrix<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T> {
    fn init(width: usize, height: usize, fill_with: T) -> Self where T: Clone {
        Self {
            width,
            height,
            data: vec![fill_with; width * height],
        }
    }

    fn get<'b>(&'b self, x: usize, y: usize) -> Option<&'b T> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.data[x + y * self.width])
        }
    }

    fn get_mut<'b>(&'b mut self, x: usize, y: usize) -> Option<&'b mut T> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&mut self.data[x + y * self.width])
        }
    }

    fn repeat(self, repeat_x: usize, repeat_y: usize) -> Self where T: Clone {
        let mut data = Vec::with_capacity(self.width * repeat_x * self.height * repeat_y);
        for _y2 in 0..repeat_y {
            for y in 0..self.height {
                for _x2 in 0..repeat_x {
                    for x in 0..self.width {
                        data.push(self.data[x + y * self.width].clone());
                    }
                }
            }
        }

        Self {
            data,
            width: self.width * repeat_x,
            height: self.height * repeat_y,
        }
    }

    fn apply<F>(&mut self, f: F)
    where
        F: for<'c> Fn(&'c T, usize, usize) -> T
    {
        let mut data = Vec::with_capacity(self.width * self.height);
        for (index, elem) in self.data.iter().enumerate() {
            data.push(f(elem, index % self.width, index / self.width));
        }
        std::mem::swap(&mut data, &mut self.data);
    }
}

fn parse(raw: String) -> Matrix<u8> {
    let mut lines = raw.split("\n").filter(|line| *line != "");
    let mut height = 0;
    let mut width = 0;
    let mut data: Vec<u8> = Vec::new();

    for line in lines {
        height += 1;
        let mut x = 0;
        for c in line.chars() {
            x += 1;
            data.push(c.to_digit(10).unwrap() as u8);
        }
        if x > width {
            width = x;
        }
    }

    Matrix {
        data,
        width,
        height
    }
}

#[inline]
fn try_dir(x: usize, y: usize, cost: u32, matrix: &Matrix<u8>, open: &mut Vec<(usize, usize, u32)>, closed: &mut Matrix<bool>) {
    if let Some(false) = closed.get(x, y) {
        *closed.get_mut(x, y).unwrap() = true;
        open.push((x, y, cost + *matrix.get(x, y).unwrap() as u32));
    }
}

fn part_1(matrix: Matrix<u8>) -> u32 {
    let mut open = vec![(0, 0, 0)];
    let mut closed = Matrix::init(matrix.width, matrix.height, false);

    while let Some((x, y, cost)) = open.pop() {
        if x == matrix.width - 1 && y == matrix.height - 1 {
            return cost
        }

        if x < matrix.width - 1 {
            try_dir(x + 1, y, cost, &matrix, &mut open, &mut closed);
        }
        if x > 0 {
            try_dir(x - 1, y, cost, &matrix, &mut open, &mut closed);
        }
        if y < matrix.height - 1 {
            try_dir(x, y + 1, cost, &matrix, &mut open, &mut closed);
        }
        if y > 0 {
            try_dir(x, y - 1, cost, &matrix, &mut open, &mut closed);
        }
        open.sort_unstable_by_key(|x| -(x.2 as i32));
    }

    0
}

fn part_2(matrix: Matrix<u8>) -> u32 {
    let width = matrix.width;
    let height = matrix.height;
    let mut matrix = matrix.repeat(5, 5);
    matrix.apply(|&e, x, y| (e - 1 + (x / width) as u8 + (y / height) as u8) % 9 + 1);

    part_1(matrix)
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

// Took me a second to realize that it wraps back to 1 and not 0
#[test]
fn test_repeat() {
    let matrix = Matrix {
        data: vec![8],
        width: 1,
        height: 1
    };
    let mut repeated = matrix.repeat(5, 5);
    assert_eq!(repeated, Matrix::init(5, 5, 8));
    let width = 1;
    let height = 1;
    repeated.apply(|&e, x, y| (e - 1 + (x / width) as u8 + (y / height) as u8) % 9 + 1);
    assert_eq!(repeated, Matrix {
        data: vec![
            8, 9, 1, 2, 3,
            9, 1, 2, 3, 4,
            1, 2, 3, 4, 5,
            2, 3, 4, 5, 6,
            3, 4, 5, 6, 7,
        ],
        width: 5,
        height: 5,
    });
}
