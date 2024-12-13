use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = parse_input("input.txt");
    println!("part 1: {}", solve_part1(&input));
    println!("part 2: {}", solve_part2(&input))
}

type Map = Vec<Vec<usize>>;

fn parse_input(filename: &str) -> Map {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            l.chars()
                .map(|c| {
                    c.to_digit(10)
                        .unwrap_or_else(|| panic!("cannot unwrap {c}")) as usize
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Map>()
}

type Location = (isize, isize);

fn traverse_peaks(level: usize, location: &Location, map: &Map, peaks: &mut HashSet<Location>) {
    if location.0 < 0
        || location.0 >= map.len() as isize
        || location.1 < 0
        || location.1 >= map[0].len() as isize
    {
        return;
    }

    let new_level = map[location.0 as usize][location.1 as usize] + 1;
    if new_level != level + 1 {
        return;
    }

    if level == 9 {
        peaks.insert(*location);
        return;
    }

    let directions: [Location; 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for direction in directions {
        let next_location = (location.0 + direction.0, location.1 + direction.1);
        traverse_peaks(new_level, &next_location, map, peaks);
    }
}

fn solve_part1(input: &Map) -> usize {
    let trailheads = input
        .iter()
        .enumerate()
        .flat_map(|(r_idx, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, v)| **v == 0)
                .map(move |(c_idx, _)| (r_idx as isize, c_idx as isize))
        })
        .collect::<Vec<Location>>();

    trailheads
        .iter()
        .map(|trailhead| {
            let mut peaks: HashSet<Location> = HashSet::new();
            traverse_peaks(0, &trailhead, input, &mut peaks);
            let score = peaks.len();
            println!("{trailhead:?}: {score} <- {peaks:?}");
            score
        })
        // .inspect(|score| println!("{score}"))
        .sum()
}

fn traverse_peaks_part2(level: usize, location: &Location, map: &Map, peaks: &mut usize) {
    if location.0 < 0
        || location.0 >= map.len() as isize
        || location.1 < 0
        || location.1 >= map[0].len() as isize
    {
        return;
    }

    let new_level = map[location.0 as usize][location.1 as usize] + 1;
    if new_level != level + 1 {
        return;
    }

    if level == 9 {
        *peaks += 1;
        return;
    }

    let directions: [Location; 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for direction in directions {
        let next_location = (location.0 + direction.0, location.1 + direction.1);
        traverse_peaks_part2(new_level, &next_location, map, peaks);
    }
}

fn solve_part2(input: &Map) -> usize {
    let trailheads = input
        .iter()
        .enumerate()
        .flat_map(|(r_idx, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, v)| **v == 0)
                .map(move |(c_idx, _)| (r_idx as isize, c_idx as isize))
        })
        .collect::<Vec<Location>>();

    trailheads
        .iter()
        .map(|trailhead| {
            let mut peaks = 0;
            traverse_peaks_part2(0, &trailhead, input, &mut peaks);
            println!("{trailhead:?}: {peaks}");
            peaks
        })
        // .inspect(|score| println!("{score}"))
        .sum()
}