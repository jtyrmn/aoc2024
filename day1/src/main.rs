use std::{collections::HashMap, fs::File, io::{self, BufRead}};

const LIST_LEN: usize = 1000;

fn main() {
    let (left, right) = get_input("input.txt");

    println!("part 1: {}", calculate_distances(left.clone(), right.clone()));
    println!("part 2: {}", calculate_similarity(left, right));
} 

fn get_input(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let file = File::open(filename).expect("cannot open file");
    
    let mut left: Vec<i32> = Vec::with_capacity(LIST_LEN);
    let mut right: Vec<i32> = Vec::with_capacity(LIST_LEN);

    for line in io::BufReader::new(file).lines() {
        match line {
            Ok(line) => {
                let columns: Vec<i32> = line.split_whitespace().into_iter()
                    .map(|s| s.parse::<i32>().expect(&format!("cannot parse {s}")))
                    .collect();
                left.push(columns[0]);
                right.push(columns[1]);
            },
            Err(err) => panic!("error in loop: {}", err),
        }
    }
    
    (left, right)
}

fn calculate_distances(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    left.sort();
    right.sort();

    left.iter().zip(right.iter()).map(|(l, r)| i32::abs(l - r)).sum()
}

fn calculate_similarity(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut occurrences = right.iter()
        .fold(HashMap::new(), |mut map, v| {
            map.entry(v).and_modify(|item| *item += 1).or_insert(1);
            map
        }
    );

    left.iter().map(|i| *i * *occurrences.entry(i).or_insert(0)).sum()
}