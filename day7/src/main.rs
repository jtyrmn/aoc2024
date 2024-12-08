use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    let input = parse_input("input.txt");
    println!("part 1: {}", solve_part1(&input));
    println!("part 2: {}", solve_part2(&input));
}

type Calibration = (i128, Vec<i128>);

fn parse_input(filename: &str) -> Vec<Calibration> {
    let pattern = Regex::new(r"(\d+): (.+)").unwrap();
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            let captures = pattern.captures(&l).unwrap();
            let test_value = captures.get(1).unwrap().as_str().parse::<i128>().unwrap();
            let equation = captures
                .get(2)
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|s| s.parse::<i128>().unwrap())
                .collect::<Vec<i128>>();
            (test_value, equation)
        })
        .collect::<Vec<Calibration>>()
}

fn can_sum_to(target: i128, current_value: i128, items: &[i128]) -> bool {
    if current_value > target {
        return false;
    }
    if items.len() == 0 {
        return target == current_value;
    }
    can_sum_to(target, current_value + items[0], &items[1..])
        || can_sum_to(target, current_value * items[0], &items[1..])
}

fn solve_part1(calibrations: &Vec<Calibration>) -> i128 {
    calibrations
        .iter()
        .filter(|c| can_sum_to(c.0, c.1[0], &c.1[1..]))
        .inspect(|c| println!("{:?}", c))
        .map(|c| c.0)
        .sum()
}

fn concatenate(a: i128, b: i128) -> i128 {
    let mut result = a.to_string();
    result.push_str(b.to_string().as_str());
    result.parse::<i128>().unwrap()
}

fn can_sum_to_with_concat(target: i128, current_value: i128, items: &[i128]) -> bool {
    if current_value > target {
        return false;
    }
    if items.len() == 0 {
        return target == current_value;
    }
    can_sum_to_with_concat(target, current_value + items[0], &items[1..])
        || can_sum_to_with_concat(target, current_value * items[0], &items[1..])
        || can_sum_to_with_concat(target, concatenate(current_value, items[0]), &items[1..])
}

fn solve_part2(calibrations: &Vec<Calibration>) -> i128 {
    calibrations
        .iter()
        .filter(|c| can_sum_to_with_concat(c.0, c.1[0], &c.1[1..]))
        .inspect(|c| println!("{:?}", c))
        .map(|c| c.0)
        .sum()
}
