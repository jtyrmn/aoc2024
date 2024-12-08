use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Tile {
    Empty,
    Explored(HashSet<&'static Location>),
    Obstacle,
}

type TileMap = Vec<Vec<Tile>>;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Location {
    row: i32,
    col: i32,
}

impl Location {
    fn add(&self, location: &Location) -> Location {
        Location {
            row: self.row + location.row,
            col: self.col + location.col,
        }
    }
}

const LOCATIONS: [Location; 4] = [
    Location { row: -1, col: 0 },
    Location { row: 0, col: 1 },
    Location { row: 1, col: 0 },
    Location { row: 0, col: -1 },
];

fn main() {
    let (tile_map, starting_location) = parse_input("input.txt");

    println!("starting at {:?}", starting_location);
    // for row in &tile_map {
    //     println!(
    //         "{:?}",
    //         row.iter()
    //             .map(|c| match c {
    //                 Tile::Empty => '.',
    //                 Tile::Explored(_) => '^',
    //                 Tile::Obstacle => '#',
    //             })
    //             .collect::<Vec<char>>()
    //     )
    // }

    println!("part 1: {}", part1(&tile_map, &starting_location));
    println!("part 2: {}", part2(&tile_map, &starting_location));
}

fn parse_input(filename: &str) -> (TileMap, Location) {
    let mut starting_pos: Location = Location { row: -1, col: -1 };
    let tile_map = BufReader::new(File::open(filename).expect("could not open file"))
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .into_iter()
                .enumerate()
                .map(|(col, character)| match character {
                    '.' => Tile::Empty,
                    '#' => Tile::Obstacle,
                    '^' => {
                        starting_pos = Location {
                            row: row as i32,
                            col: col as i32,
                        };
                        Tile::Explored({
                            let mut hash_set = HashSet::with_capacity(4);
                            hash_set.insert(&LOCATIONS[0]);
                            hash_set
                        })
                    }
                    unknown => panic!("unrecognized character \'{}\'", unknown),
                })
                .collect()
        })
        .collect::<TileMap>();

    if starting_pos.row == -1 || starting_pos.col == -1 {
        panic!("did not find starting position")
    }

    (tile_map, starting_pos)
}

fn get_tile<'a>(tile_map: &'a mut TileMap, location: Location) -> Option<&'a mut Tile> {
    if location.row < 0 || location.col < 0 {
        return None;
    }
    let row = tile_map.get_mut(location.row as usize)?;
    row.get_mut(location.col as usize)
}

fn part1(tile_map: &TileMap, starting_location: &Location) -> i32 {
    let mut count_explored = 1;
    let mut directions = LOCATIONS.iter().cycle();
    let mut location = *starting_location;
    let mut forward = (&mut directions).next().unwrap();
    let mut tile_map = tile_map.clone();

    loop {
        let next_location = location.add(forward);
        let tile = get_tile(&mut tile_map, next_location);
        match tile {
            None => {
                println!("-----------------");
                for row in &tile_map {
                    println!(
                        "{}",
                        row.iter()
                            .map(|c| match c {
                                Tile::Empty => '.',
                                Tile::Explored(_) => '^',
                                Tile::Obstacle => '#',
                            })
                            .collect::<String>()
                    )
                }
                return count_explored;
            }
            Some(t) => match t {
                Tile::Empty => {
                    *t = Tile::Explored({
                        let mut hash_set = HashSet::new();
                        hash_set.insert(forward);
                        hash_set
                    });
                    count_explored += 1;
                    location = next_location;
                }
                Tile::Explored(_) => {
                    location = next_location;
                }
                Tile::Obstacle => {
                    forward = (&mut directions).next().unwrap();
                }
            },
        }
    }
}

fn forms_loop(tile_map: TileMap, starting_location: &Location) -> bool {
    let mut directions = LOCATIONS.iter().cycle();
    let mut location = *starting_location;
    let mut forward = (&mut directions).next().unwrap();
    let mut tile_map = tile_map;

    loop {
        let next_location = location.add(forward);
        let tile = get_tile(&mut tile_map, next_location);
        match tile {
            None => {
                return false;
            }
            Some(t) => match t {
                Tile::Empty => {
                    *t = Tile::Explored({
                        let mut hash_set = HashSet::new();
                        hash_set.insert(forward);
                        hash_set
                    });
                    location = next_location;
                }
                Tile::Explored(hash_set) => {
                    if hash_set.contains(forward) {
                        return true;
                    }
                    hash_set.insert(&forward);
                    location = next_location;
                }
                Tile::Obstacle => {
                    forward = (&mut directions).next().unwrap();
                }
            },
        }
    }
}

fn part2(tile_map: &TileMap, starting_location: &Location) -> usize {
    (0..tile_map.len())
        .inspect(|r| {
            eprintln!("row {}...", r + 1);
        })
        .cartesian_product(0..tile_map[0].len())
        .filter(|(r, c)| {
            let mut tile_map = tile_map.clone();
            tile_map[*r][*c] = Tile::Obstacle;
            forms_loop(tile_map, starting_location)
        })
        .inspect(|(r, c)| {
            println!("{} {}", r, c);
        })
        .count()
}
