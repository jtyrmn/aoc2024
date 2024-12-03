use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = parse_input("input.txt");

    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    let part2 = solve_part2(input);
    println!("part 2: {}", part2);
    // println!("{:?}", debug(&input));
}

fn parse_input(filename: &str) -> Vec<Vec<i64>> {
    let file = File::open(filename).unwrap_or_else(|_| panic!("cannot open {}", filename));
    BufReader::new(file)
        .lines()
        .map(|line| {
            line.expect("cannot read line")
                .split_whitespace()
                .map(|line| {
                    line.parse::<i64>()
                        .unwrap_or_else(|_| panic!("cannot parse {}", line))
                })
                .collect()
        })
        .collect()
}

fn solve_part1(items: &Vec<Vec<i64>>) -> usize {
    // items.iter().map(|levels| {
    //     let iter1 = levels.iter();
    //     let iter2 = levels.iter().skip(1);
    //     iter1.zip(iter2).map(|(l, r)| r - l)
    // }).filter(|changes| {
    //     changes.clone().filter(|change| {let c = change.abs(); c < 1 || c > 3}).peekable().peek().is_none()
    // }).filter(|changes| {
    //     changes.clone().all(|c| c > 0) || changes.clone().all(|c| c < 0)
    // }).count()

    items.iter().filter(|item| is_safe(&item)).count()
}

fn is_safe(levels: &Vec<i64>) -> bool {
    let iter = levels.iter();
    let iter2 = levels.iter().skip(1);
    let change = iter.zip(iter2).map(|(l, r)| r - l);

    if change.clone().any(|c| {
        let abs = c.abs();
        abs < 1 || abs > 3
    }) {
        return false;
    }
    change.clone().all(|c| c > 0) || change.clone().all(|c| c < 0)
}

fn solve_part2(items: Vec<Vec<i64>>) -> usize {
    items
        .into_iter()
        .filter(|item| {
            for i in 0..item.len() {
                let mut new_item = item.clone();
                new_item.remove(i);
                if is_safe(&new_item) {
                    return true;
                }
            }
            false
        })
        .count()
}

// fn debug(items: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
//     items.iter().map(|levels| {
//         let iter1 = levels.iter();
//         let iter2 = levels.iter().skip(1);
//         iter1.zip(iter2).map(|(l, r)| r - l)
//     }).filter(|changes| {
//         changes.clone().filter(|change| {let c = change.abs(); c < 1 || c > 3}).peekable().peek().is_none()
//     }).filter(|changes| {
//         changes.clone().all(|c| c > 0) || changes.clone().all(|c| c < 0)
//     }).map(|c| c.collect()).collect()
// }
