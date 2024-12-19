use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn main() {
    let input = parse_input("input.txt");
    println!("part 1: {}", solve_part1(&input));
}

#[derive(Debug)]
struct Matrix(Vec<Vec<char>>);

impl Matrix {
    fn cell(&self, r: isize, c: isize) -> Option<char> {
        if r < 0 || r >= (*self).0.len() as isize {
            return None;
        }
        if c < 0 || c >= (*self).0[0].len() as isize {
            return None;
        }

        Some((*self).0[r as usize][c as usize])
    }

    fn iter_idx(&self) -> impl Iterator<Item = (isize, isize)> {
        isize_range(0, self.0.len() as isize)
            .cartesian_product(isize_range(0, self.0[0].len() as isize))
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn tuple(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    const VALUES: [Self; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
}

fn parse_input(filename: &str) -> Matrix {
    Matrix(
        BufReader::new(File::open(filename).unwrap())
            .lines()
            .map(Result::unwrap)
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect(),
    )
}

fn isize_range(start: isize, end: isize) -> std::ops::Range<isize> {
    std::ops::Range { start, end }
}

fn area(
    matrix: &Matrix,
    r: isize,
    c: isize,
    plant: char,
    seen: &mut HashSet<(isize, isize)>,
) -> usize {
    let current_plant = match matrix.cell(r, c) {
        Some(c) => c,
        None => return 0,
    };

    if current_plant != plant {
        return 0;
    }

    if seen.contains(&(r, c)) {
        return 0;
    }
    seen.insert((r, c));

    let mut sum = 1;
    for direction in &Direction::VALUES {
        let (dr, dc) = direction.tuple();
        sum += area(matrix, r + dr, c + dc, plant, seen)
    }
    sum
}

fn perimeter(
    matrix: &Matrix,
    r: isize,
    c: isize,
    plant: char,
    seen: &mut HashSet<(isize, isize)>,
) -> usize {
    let current_plant = match matrix.cell(r, c) {
        Some(c) => c,
        None => return 1,
    };

    if current_plant != plant {
        return 1;
    }

    if seen.contains(&(r, c)) {
        return 0;
    }
    seen.insert((r, c));

    let mut sum = 0;
    for direction in &Direction::VALUES {
        let (dr, dc) = direction.tuple();
        sum += perimeter(matrix, r + dr, c + dc, plant, seen)
    }
    sum
}

// fn solve_part1(input: &Matrix) -> usize {
//     let mut seen_area: HashSet<(isize, isize)> = HashSet::new();
//     let mut seen_perimeter: HashSet<(isize, isize)> = HashSet::new();
//     let mut areas: HashMap<char, usize> = HashMap::new();
//     let mut perimeters: HashMap<char, usize> = HashMap::new();

//     for (r, c) in input.iter_idx() {
//         let plant = input.cell(r, c).unwrap();

//         let area = area(input, r, c, plant, &mut seen_area);
//         *areas.entry(plant).or_insert(0) += area;

//         let perimeter = perimeter(input, r, c, plant, &mut seen_perimeter);
//         *perimeters.entry(plant).or_insert(0) += perimeter;
//     }
//     areas
//         .iter()
//         .map(|(plant, area)| {
//             let perimeter = perimeters.get(plant).unwrap();
//             println!("\tcharacter {} has area {} and perimeter {}", plant, area, perimeter);
//             area * perimeter
//         })
//         .sum()
// }

fn solve_part1(input: &Matrix) -> usize {
    let mut seen_area: HashSet<(isize, isize)> = HashSet::new();
    let mut seen_perimeter: HashSet<(isize, isize)> = HashSet::new();

    let mut price = 0;
    for (r, c) in input.iter_idx() {
        let plant = input.cell(r, c).unwrap();
        let area = area(input, r, c, plant, &mut seen_area);
        if area == 0 {
            continue;
        }
        let perimeter = perimeter(input, r, c, plant, &mut seen_perimeter);
        price += area * perimeter;
    }
    price
}
