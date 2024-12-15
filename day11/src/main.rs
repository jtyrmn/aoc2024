use std::collections::HashMap;

fn main() {
    const input: &str = "4022724 951333 0 21633 5857 97 702 6";
    const input_test: &str = "125 17";

    println!("part 1: {}", solve_part1(&input));
    println!("part 2: {}", solve_part2(&input));
}

fn process_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
}

enum StoneResult {
    SingleStone(usize),
    DoubleStone(usize, usize),
}

fn process_single_stone(v: usize) -> StoneResult {
    if v == 0 {
        return StoneResult::SingleStone(1);
    }
    let v_str = v.to_string();
    if v_str.len() % 2 == 0 {
        let (left, right) = (&v_str[0..v_str.len() / 2], &v_str[v_str.len() / 2..]);
        let (left, right) = (left.parse().unwrap(), right.parse().unwrap());
        return StoneResult::DoubleStone(left, right);
    }

    StoneResult::SingleStone(v * 2024)
}

fn calculate_blinks(input: &str, blinks: usize) -> usize {
    let mut stones = process_input(input);
    for i in 0..blinks {
        stones = stones
            .iter()
            .map(|stone| process_single_stone(*stone))
            .fold(Vec::new(), |mut acc, stone| match stone {
                StoneResult::SingleStone(v) => {
                    acc.push(v);
                    acc
                }
                StoneResult::DoubleStone(v1, v2) => {
                    acc.push(v1);
                    acc.push(v2);
                    acc
                }
            });
        println!("{i}: {}", stones.len());
    }
    stones.len()
}

fn calculate_blinks_fast(input: &str, blinks: usize) -> usize {
    let mut stones = process_input(input).iter().fold(
        HashMap::new(),
        |mut acc: HashMap<usize, usize>, &next| {
            acc.entry(next).and_modify(|c| *c += 1).or_insert(1);
            acc
        },
    );

    println!("0: {} stones -> {:?}", stones.len(), stones);
    for i in 0..blinks {
        stones = stones.iter().fold(HashMap::new(), |mut acc: HashMap<usize, usize>, (stone, count)| {
            match process_single_stone(*stone) {
                StoneResult::SingleStone(v) => {
                    acc.entry(v).and_modify(|c| *c += *count).or_insert(*count);
                },
                StoneResult::DoubleStone(v1, v2) => {
                    acc.entry(v1).and_modify(|c| *c += *count).or_insert(*count);
                    acc.entry(v2).and_modify(|c| *c += *count).or_insert(*count);
                },
            };
            acc
        });
        // println!("{}: {} stones -> {:?}", i + 1, stones.len(), stones);
    }
    stones.iter().fold(0, |acc, (_, count)| acc + count)
}

fn solve_part1(input: &str) -> usize {
    calculate_blinks_fast(input, 25)
}

fn solve_part2(input: &str) -> usize {
    calculate_blinks_fast(input, 75)
}
