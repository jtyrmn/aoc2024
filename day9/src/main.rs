use std::{fs::File, io::{Empty, Read}, iter::repeat_n};

fn main() {
    let input = parse_input("input.txt");

    println!("part 1: {}", solve_part1(&input));
    println!("part 2: {}", solve_part2(&input));
}

fn parse_input(filename: &str) -> String {
    let mut buf = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut buf).unwrap();
    buf
}

#[derive(Debug, Clone, Copy)]
enum DiskBlock {
    File { size: usize, id: usize },
    Empty { gap: usize },
}

type DiskMap = Vec<DiskBlock>;

fn get_diskmap(input: &str) -> DiskMap {
    input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let size = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                DiskBlock::File { size, id: i / 2 }
            } else {
                DiskBlock::Empty { gap: size }
            }
        })
        .collect::<DiskMap>()
}

fn print_diskmap(diskmap: &DiskMap) {
    let output = diskmap
        .iter()
        .map(|b| match b {
            DiskBlock::File { size, id } => repeat_n(
                char::from_digit((*id % 10).try_into().unwrap(), 10).unwrap(),
                *size,
            )
            .collect::<String>(),
            DiskBlock::Empty { gap } => repeat_n('.', *gap).collect::<String>(),
        })
        .collect::<Vec<String>>()
        .join("|");
    println!("{output}");
}

fn diskmap_checksum(diskmap: &DiskMap) -> usize {
    diskmap
        .iter()
        .map(|b| match b {
            DiskBlock::File { size, id } => (*size, *id),
            DiskBlock::Empty { gap } => (*gap, 0),
        })
        .flat_map(|(size, id)| repeat_n(id, size))
        .enumerate()
        // .inspect(|v| println!("\t{v:?}"))
        .fold(0, |acc, (seq, id)| acc + seq * id)
}

fn solve_part1(input: &str) -> usize {
    let mut diskmap = get_diskmap(input);
    // print_diskmap(&diskmap);

    let mut leftmost_empty = 0;
    let mut rightmost_occupied = diskmap.len() - 1;
    diskmap.push(DiskBlock::Empty { gap: 0 });

    while leftmost_empty < rightmost_occupied {
        let size_gap_to_fill = match diskmap[leftmost_empty] {
            DiskBlock::File { size: _, id: _ } => {
                leftmost_empty += 1;
                continue;
            }
            DiskBlock::Empty { gap } => gap,
        };
        let (num_blocks_to_move, id) = match diskmap[rightmost_occupied] {
            DiskBlock::File { size, id } => (size, id),
            DiskBlock::Empty { gap: _ } => {
                rightmost_occupied -= 1;
                continue;
            }
        };

        if size_gap_to_fill <= num_blocks_to_move {
            diskmap[leftmost_empty] = DiskBlock::File {
                size: size_gap_to_fill,
                id,
            };
            if size_gap_to_fill == num_blocks_to_move {
                diskmap.remove(rightmost_occupied);
            } else {
                diskmap[rightmost_occupied] = DiskBlock::File {
                    size: num_blocks_to_move - size_gap_to_fill,
                    id,
                };
            }
        } else {
            diskmap.insert(
                leftmost_empty,
                DiskBlock::File {
                    size: num_blocks_to_move,
                    id,
                },
            );
            diskmap[leftmost_empty + 1] = DiskBlock::Empty {
                gap: size_gap_to_fill - num_blocks_to_move,
            };
            diskmap.remove(rightmost_occupied + 1);
        }

        // print_diskmap(&diskmap);
    }

    diskmap_checksum(&diskmap)
}

fn next_gap_with_size(diskmap: &DiskMap, min_size: usize) -> Option<(usize, usize)> {
    for i in 0..diskmap.len() {
        match diskmap[i] {
            DiskBlock::File { size: _, id: _ } => {
                continue;
            }
            DiskBlock::Empty { gap } => {
                if gap >= min_size {
                    return Option::Some((i, gap));
                }
                continue;
            }
        }
    }
    None
}

fn merge_empty_blocks(diskmap: &mut DiskMap) -> usize {
    let mut num_merges = 0;
    let mut l = 0;
    while l < diskmap.len() - 1 {
        if let DiskBlock::Empty { gap } = diskmap[l] {
            if gap == 0 {
                diskmap.remove(l);
                num_merges += 1;
                l -= 1;
            }
        }

        let r = l + 1;
        match (diskmap[l], diskmap[r]) {
            (DiskBlock::Empty { gap: l_gap }, DiskBlock::Empty { gap: r_gap }) => {
                diskmap.remove(r);
                diskmap[l] = DiskBlock::Empty { gap: l_gap + r_gap };
                num_merges += 1;
                l -= 1;
            }
            _ => {
                l += 1;
                continue;
            }
        }
        l += 1;
    }
    num_merges
}

fn solve_part2(input: &str) -> usize {
    let mut diskmap = get_diskmap(input);
    // print_diskmap(&diskmap);

    let mut file_to_move: isize = diskmap.len() as isize - 1;
    while file_to_move >= 0 {
        let (file_size, id) = match diskmap[file_to_move as usize] {
            DiskBlock::File { size, id } => (size, id),
            DiskBlock::Empty { gap: _ } => {
                file_to_move -= 1;
                continue;
            }
        };

        let (empty_space_to_fill, gap_size) = match next_gap_with_size(&diskmap, file_size) {
            Some(v) => v,
            None => {
                file_to_move -= 1;
                continue;
            }
        };

        if empty_space_to_fill >= file_to_move as usize {
            file_to_move -= 1;
            continue;
        }

        if file_size == empty_space_to_fill {
            diskmap[empty_space_to_fill] = DiskBlock::File {
                size: file_size,
                id,
            };
        } else {
            diskmap[empty_space_to_fill] = DiskBlock::File {
                size: file_size,
                id,
            };
            diskmap.insert(
                empty_space_to_fill + 1,
                DiskBlock::Empty {
                    gap: gap_size - file_size,
                },
            );
            file_to_move += 1;
        }
        diskmap.remove(file_to_move as usize);
        diskmap.insert(file_to_move as usize, DiskBlock::Empty { gap: file_size });
        file_to_move -= 1;

        // print_diskmap(&diskmap);

        // let num_merges = merge_empty_blocks(&mut diskmap);
        // file_to_move -= num_merges as isize;
        // print_diskmap(&diskmap);
    }

    diskmap_checksum(&diskmap)
}

// 9685220703964 -- too high
// 6398424798063 -- too high