use std::{fs::File, io::Read};

use regex::Regex;

fn main() {
    let input = get_input("input.txt");
    println!("part 1: {}", part1(&input.as_str()));

    let disabled = disable(&input);
    println!("{}", disabled);
    println!("part 2: {}", part1(&disabled.as_str()));
}

fn get_input(filename: &str) -> String {
    let mut file = match File::open(filename) {
        Err(e) => panic!("cannot open {}: {}", filename, e),
        Ok(file) => file,
    };

    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("cannot read file");
    buf
}

fn part1(input: &str) -> i32 {
    let pattern = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();
    pattern
        .captures_iter(input)
        .map(|capture| {
            let a = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let b = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
            a * b
        })
        .sum()
}

fn disable(input: &str) -> String {
    let pattern = Regex::new(r"(?s)don't\(\)(.*?)(do\(\))").unwrap();
    // for capture in pattern.captures_iter(input) {
    //     let start = capture.get(0).unwrap().start();
    //     let end = capture.get(0).unwrap().end();

    //     for i in [start..=end] {
    //         input.
    //     }
    //     input.replace(from, to)
    // }

    pattern.replace_all(input, "<DISABLED>").to_string()
}

// 48810620 - too low
// 94785627 - too high
// 85879953 - likely too high, some don't() tokens at the end
// 179834255 - too high
// 80570939 - right answer, cheaped out and just added a do() to the end of the input
