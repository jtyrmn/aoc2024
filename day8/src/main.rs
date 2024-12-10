use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

type Matrix = Vec<Vec<char>>;

fn main() {
    let input = parse_input("input.txt");
    // print(&input);
    println!("part 1: {}", solve_part1(&input));
    println!("part 2: {}", solve_part2(&input));
}

fn parse_input(filename: &str) -> Matrix {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Matrix>()
}

fn print(matrix: &Matrix) {
    for line in matrix {
        println!("{}", line.iter().collect::<String>());
    }
}

fn get_antennas(matrix: &Matrix) -> HashMap<char, Vec<(i32, i32)>> {
    (0..matrix.len())
        .cartesian_product(0..matrix[0].len())
        .map(|(r, c)| (matrix[r][c], r as i32, c as i32))
        .filter(|(v, _, _)| *v != '.')
        .fold(
            HashMap::new(),
            |mut acc: HashMap<char, Vec<(i32, i32)>>, (v, r, c)| {
                acc.entry(v).and_modify(|e| e.push((r, c))).or_insert({
                    let mut vec = Vec::new();
                    vec.push((r, c));
                    vec
                });
                acc
            },
        )
}

fn distance(r1: i32, c1: i32, r2: i32, c2: i32) -> f32 {
    let dx = (c2 - c1) as f32;
    let dy = (r1 - r2) as f32;
    (dx.powi(2) + dy.powi(2)).sqrt()
}

fn floats_approx_eq(f1: f32, f2: f32) -> bool {
    (f1 - f2).abs() < 1e-6
    // -6: 594
    // -5: 865
    // -4: 949
    // -3: 949
    // -2: 957
}

fn direction_vec(ra: i32, ca: i32, rb: i32, cb: i32) -> (f32, f32) {
    let d = distance(ra, ca, rb, cb);
    (((cb - ca) as f32) / d, ((rb - ra) as f32) / d)
}

fn same_directions(r: i32, c: i32, r1: i32, c1: i32, r2: i32, c2: i32) -> bool {
    let d1 = direction_vec(r, c, r1, c1);
    let d2 = direction_vec(r, c, r2, c2);
    floats_approx_eq(d1.0, d2.0) && floats_approx_eq(d1.1, d2.1)
}

fn solve_part1(matrix: &Matrix) -> i32 {
    let antennas = get_antennas(matrix);
    // println!("{:?}", antennas);

    let mut num_antinode_spots: i32 = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            'is_antinode: for wavelength in &antennas {
                if wavelength.1.len() <= 1 {
                    continue;
                }
                for a1 in 0..wavelength.1.len() {
                    for a2 in (a1 + 1)..wavelength.1.len() {
                        let a1 = wavelength.1[a1];
                        let a2 = wavelength.1[a2];
                        if !same_directions(row as i32, col as i32, a1.0, a1.1, a2.0, a2.1) {
                            continue;
                        }
                        let d1 = distance(row as i32, col as i32, a1.0, a1.1);
                        let d2 = distance(row as i32, col as i32, a2.0, a2.1);
                        let ratio = d1 / d2;
                        if floats_approx_eq(ratio, 2_f32) || floats_approx_eq(ratio, 0.5_f32) {
                            num_antinode_spots += 1;
                            // println!("({row},{col}) antinode from {} between {a1:?} and {a2:?}", wavelength.0);
                            break 'is_antinode;
                        }
                    }
                }
            }
        }
    }

    num_antinode_spots
}

fn is_multiple(big: f32, small: f32) -> bool {
    let mut big = big;
    while big > 0_f32 {
        if floats_approx_eq(big, small) {
            return true;
        }
        big -= small;
    }
    false
}

fn solve_part2(matrix: &Matrix) -> i32 {
    let antennas = get_antennas(matrix);
    // println!("{:?}", antennas);

    let mut num_antinode_spots: i32 = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] != '.' {
                num_antinode_spots += 1;
                println!("({row},{col}) antinode from antenna");
                continue;
            }
            'is_antinode: for wavelength in &antennas {
                if wavelength.1.len() <= 1 {
                    continue;
                }
                for a1 in 0..wavelength.1.len() {
                    for a2 in (a1 + 1)..wavelength.1.len() {
                        let a1 = wavelength.1[a1];
                        let a2 = wavelength.1[a2];
                        if !same_directions(row as i32, col as i32, a1.0, a1.1, a2.0, a2.1) {
                            continue;
                        }
                        let d1 = distance(row as i32, col as i32, a1.0, a1.1);
                        let d2 = distance(row as i32, col as i32, a2.0, a2.1);

                        let delta = (d1 - d2).abs();
                        
                        if is_multiple(d1, delta) || is_multiple(d2, delta) {
                            num_antinode_spots += 1;
                            println!("({row},{col}) antinode from {} between {a1:?} and {a2:?}", wavelength.0);
                            break 'is_antinode;
                        }
                    }
                }
            }
        }
    }

    num_antinode_spots
}

// 865 -- too low
// 949 -- correct