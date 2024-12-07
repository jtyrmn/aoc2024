use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

type Page = i32;

#[derive(Debug)]
struct Problem {
    ordering_rules: Vec<(Page, Page)>,
    pages: Vec<Vec<i32>>,
}

fn main() {
    let problem_set = parse_input("input.txt");
    println!("part 1: {}", solve_part1(&problem_set));
    println!("part 2: {}", solve_part2(&problem_set));
}

fn parse_input(filename: &str) -> Problem {
    let mut file_reader = BufReader::new(File::open(filename).expect("cannot open file"));
    let ordering_rules = (&mut file_reader)
        .lines()
        .map(Result::unwrap)
        .take_while(|l| l.contains("|"))
        .map(|l| {
            let parsed: Vec<&str> = l.trim().split("|").collect();
            let parse = |s: &str| {
                s.parse::<Page>()
                    .unwrap_or_else(|_| panic!("cannot parse {}", s))
            };
            (parse(parsed[0]), parse(parsed[1]))
        })
        .fold(Vec::new(), |mut acc: Vec<(Page, Page)>, next| {
            acc.push(next);
            acc
        });

    let pages = file_reader
        .lines()
        .map(Result::unwrap)
        .filter(|l| l.contains(","))
        .map(|l| {
            l.trim()
                .split(",")
                .map(|p| {
                    p.parse::<Page>()
                        .unwrap_or_else(|_| panic!("cannot parse {}", p))
                })
                .collect::<Vec<Page>>()
        })
        .collect::<Vec<Vec<Page>>>();

    Problem {
        ordering_rules,
        pages,
    }
}

fn is_correct_order(pages: &Vec<Page>, cannot_occur_after: &HashMap<Page, HashSet<Page>>) -> bool {
    let mut seen: HashSet<Page> = HashSet::new();
    for p in pages {
        if let Some(forbidden) = cannot_occur_after.get(&p) {
            if forbidden.intersection(&seen).next().is_some() {
                return false;
            }
        }
        seen.insert(*p);
    }
    true
}

fn solve_part1(problem_set: &Problem) -> i32 {
    let cannot_occur_after = problem_set.ordering_rules.iter().fold(
        HashMap::new(),
        |mut acc: HashMap<Page, HashSet<Page>>, next| {
            acc.entry(next.0)
                .and_modify(|v| {
                    v.insert(next.1);
                })
                .or_insert({
                    let mut v = HashSet::new();
                    v.insert(next.1);
                    v
                });
            acc
        },
    );

    problem_set
        .pages
        .iter()
        .filter(|pages| is_correct_order(*pages, &cannot_occur_after))
        // .inspect(|pages| println!("{:?}", pages))
        .map(|pages| {
            let len = pages.len();
            if len % 2 == 0 || len == 0 {
                println!("warning: zero or even number of items: {:?}", pages);
            }
            pages[len/2]
        })
        .sum()
}

fn solve_part2(problem_set: &Problem) -> i32 {
    let cannot_occur_after = problem_set.ordering_rules.iter().fold(
        HashMap::new(),
        |mut acc: HashMap<Page, HashSet<Page>>, next| {
            acc.entry(next.0)
                .and_modify(|v| {
                    v.insert(next.1);
                })
                .or_insert({
                    let mut v = HashSet::new();
                    v.insert(next.1);
                    v
                });
            acc
        },
    );

    problem_set
        .pages
        .iter()
        .filter(|pages| !is_correct_order(*pages, &cannot_occur_after))
        .map(|pages| {
            let mut pages = pages.clone();
            reorder_pages(&mut pages, &cannot_occur_after);
            pages
        })
        // .inspect(|pages| println!("{:?}", pages))
        .map(|pages| {
            let len = pages.len();
            if len % 2 == 0 || len == 0 {
                println!("warning: zero or even number of items: {:?}", pages);
            }
            pages[len/2]
        })
        .sum()
}

fn reorder_pages( pages: &mut Vec<Page>, cannot_occur_after: &HashMap<Page, HashSet<Page>>) {
    for _ in 0..pages.len() {
        for (left, right) in (0..pages.len()).zip(1..pages.len()) {
            match cannot_occur_after.get(&pages[left]) {
                None => {},
                Some(forbidden) => {
                    if forbidden.contains(&pages[right]) {
                        pages.swap(left, right);
                    }
                }
            }
        }
    }

    pages.reverse();
}
