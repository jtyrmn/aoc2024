use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Matrix = Vec<Vec<char>>;
type Direction = (i32, i32);

const LATERAL: [Direction; 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

const DIAGONAL: [Direction; 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

fn main() {
    let input = get_input("input.txt");
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn get_input(filename: &str) -> Matrix {
    let file = File::open(filename).unwrap();

    BufReader::new(file)
        .lines()
        .map(|l| l.expect("cannot read line"))
        .fold(Vec::new(), |mut acc, next| {
            acc.push(next.chars().collect());
            acc
        })
}

fn get_char<'a>(matrix: &'a Matrix, row: i32, col: i32) -> Option<&'a char> {
    if row < 0 || col < 0 {
        return None;
    }
    matrix.get(row as usize)?.get(col as usize)
}

fn is_xmas(matrix: &Matrix, row: i32, col: i32, direction: &Direction, string: &str) -> bool {
    if string.len() == 0 {
        return true;
    }

    match get_char(matrix, row, col) {
        None => false,
        Some(c) => {
            *c == string.chars().nth(0).unwrap()
                && is_xmas(
                    matrix,
                    row + direction.0,
                    col + direction.1,
                    direction,
                    &string[1..],
                )
        }
    }
}

fn part1(matrix: &Matrix) -> i32 {
    let mut sum = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            for direction in LATERAL.iter().chain(DIAGONAL.iter()) {
                if is_xmas(&matrix, row as i32, col as i32, &direction, "XMAS") {
                    sum += 1;
                }
            }
        }
    }
    sum
}

fn is_cross_mas(matrix: &Matrix, row: i32, col: i32) -> bool {
    if let Some(c) = get_char(matrix, row, col) {
        if *c != 'A' {
            return false;
        }
    } else {
        return false;
    }

    let mut sum = 0;
    for direction in DIAGONAL {
        match (
            get_char(matrix, row + direction.0, col + direction.1),
            get_char(matrix, row - direction.0, col - direction.1),
        ) {
            (Some(m), Some(s)) => {
                if *m == 'M' && *s == 'S' {
                    sum += 1;
                }
            }
            (_, _) => continue,
        }
    }
    if sum >= 2 {
        // println!("{} {}", row, col);
        return true;
    }
    false
}

fn part2(matrix: &Matrix) -> i32 {
    let mut sum = 0;
    for row in 1..matrix.len() - 1 {
        for col in 1..matrix[row].len() - 1 {
            if is_cross_mas(matrix, row as i32, col as i32) {
                sum += 1
            }
        }
    }
    sum
}

// 2070 - too high
// 2000 - too low (random guess)
// 2035 - too low (random guess)
// 2045 - doesn't
