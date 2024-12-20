use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Iter,
};

use regex::Regex;

fn main() {
    let problem_set = parse_input("input.txt");
    println!("part 1: {}", solve_part1(&problem_set));
    println!("part 2: {}", solve_part2(&problem_set));
}

#[derive(Clone, Copy)]
struct Pair {
    x: i64,
    y: i64,
}

struct ProblemSet {
    a: Pair,
    b: Pair,
    prize: Pair,
}

struct SplitByEmptyLines<I: Iterator<Item = String>> {
    iterator: I,
    buffer: Vec<String>,
}

impl<I: Iterator<Item = String>> Iterator for SplitByEmptyLines<I> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iterator.next() {
                Some(item) => {
                    if item.trim().is_empty() {
                        if self.buffer.is_empty() {
                            continue;
                        }
                        let buffered = self.buffer.join(" ");
                        self.buffer.clear();
                        return Some(buffered);
                    } else {
                        self.buffer.push(item);
                    }
                }
                None => {
                    if !self.buffer.is_empty() {
                        let buffered = self.buffer.join(" ");
                        self.buffer.clear();
                        return Some(buffered);
                    }
                    return None;
                }
            }
        }
    }
}

trait SplitByEmptyLinesEnumerable {
    // fn split_by_empty_lines<Self : Iterator<Item = String>>(&self) -> SplitByEmptyLines<Self> {
    //     SplitByEmptyLines { iterator: self, buffer: Vec::with_capacity(3) }
    // }
    fn split_by_empty_lines(self) -> SplitByEmptyLines<Self>
    where
        Self: Iterator<Item = String>,
        Self: Sized;
}
impl<I> SplitByEmptyLinesEnumerable for I
where
    I: Iterator<Item = String>,
{
    fn split_by_empty_lines(self) -> SplitByEmptyLines<Self>
    where
        Self: Iterator<Item = String>,
        Self: Sized,
    {
        SplitByEmptyLines {
            iterator: self,
            buffer: Vec::with_capacity(3),
        }
    }
}

fn parse_input(filename: &str) -> Vec<ProblemSet> {
    let pattern = Regex::new(
        r"Button A: X\+(\d*), Y\+(\d*).*Button B: X\+(\d*), Y\+(\d*).*Prize: X=(\d*), Y=(\d*)",
    )
    .unwrap();
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(Result::unwrap)
        .split_by_empty_lines()
        .map(|l| {
            let captures = pattern.captures(l.as_str()).unwrap();
            let a = Pair {
                x: captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                y: captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            };
            let b = Pair {
                x: captures.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                y: captures.get(4).unwrap().as_str().parse::<i64>().unwrap(),
            };
            let prize = Pair {
                x: captures.get(5).unwrap().as_str().parse::<i64>().unwrap(),
                y: captures.get(6).unwrap().as_str().parse::<i64>().unwrap(),
            };
            ProblemSet { a, b, prize }
        })
        .collect::<Vec<ProblemSet>>()
}

fn calculate_inverses(problem_set: &ProblemSet) -> Option<(i64, i64)> {
    let (a, b, p) = (problem_set.a, problem_set.b, problem_set.prize);
    let det = a.x * b.y - b.x * a.y;
    if det == 0 {
        return None;
    }

    let A = p.x * b.y - p.y * b.x;
    let B = -p.x * a.y + p.y * a.x;
    Some((A / det, B / det))
}

fn solve_part1(problem_set: &Vec<ProblemSet>) -> i64 {
    problem_set
        .iter()
        .filter_map(|p| {
            let (A, B) = calculate_inverses(p)?;
            if (A * p.a.x + B * p.b.x, A * p.a.y + B * p.b.y) != (p.prize.x, p.prize.y) {
                return None;
            }
            Some(3 * A + B)
        })
        .sum()
}

fn solve_part2(problem_set: &Vec<ProblemSet>) -> i64 {
    problem_set
        .iter()
        .filter_map(|p| {
            let p = ProblemSet {
                prize: Pair {
                    x: p.prize.x + 10000000000000,
                    y: p.prize.y + 10000000000000,
                },
                ..*p
            };
            let (A, B) = calculate_inverses(&p)?;
            if (A * p.a.x + B * p.b.x, A * p.a.y + B * p.b.y) != (p.prize.x, p.prize.y) {
                return None;
            }
            Some(3 * A + B)
        })
        .sum()
}
