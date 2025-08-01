use std::collections::{HashMap, HashSet};

use aoc2024::RawData;

const INPUT: RawData = include_str!("../data/08.txt");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

type Freq = char;

fn pos_inside_map(pos: &Position, width: usize, height: usize) -> bool {
    if pos.x < 0 || pos.y < 0 {
        return false;
    }

    if pos.x >= (width as isize) || pos.y >= (height as isize) {
        return false;
    }

    true
}

/// Axis directions:
///
/// | ------> y-axis
/// |
/// |
/// v
///  x-axis
fn get_aninodes(
    pos1: &Position,
    pos2: &Position,
    width: usize,
    height: usize,
    extend: bool,
) -> Vec<Position> {
    let mut nodes = vec![];

    {
        let mut level = if extend { 0 } else { 1 };
        loop {
            let pos = Position {
                x: pos1.x + (pos1.x - pos2.x) * level,
                y: pos1.y + (pos1.y - pos2.y) * level,
            };

            if pos_inside_map(&pos, width, height) {
                nodes.push(pos);
                level += 1;
            } else {
                break;
            }

            if !extend {
                break;
            }
        }
    }

    {
        let mut level = if extend { 0 } else { 1 };
        loop {
            let pos = Position {
                x: pos2.x + (pos2.x - pos1.x) * level,
                y: pos2.y + (pos2.y - pos1.y) * level,
            };

            if pos_inside_map(&pos, width, height) {
                nodes.push(pos);
                level += 1;
            } else {
                break;
            }

            if !extend {
                break;
            }
        }
    }

    nodes
}

fn solve(input: RawData, extend: bool) -> usize {
    let world = input
        .split("\n")
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>();

    let width = world[0].len();
    let height = world.len();

    assert_eq!(width, height);

    let mut freq_map: HashMap<Freq, Vec<Position>> = HashMap::new();

    for (x, line) in world.iter().enumerate() {
        for (y, freq) in line.iter().enumerate() {
            if freq == &'.' || freq == &'#' {
                // Empty
                continue;
            }

            match freq_map.get_mut(freq) {
                Some(v) => v.push(Position {
                    x: x as isize,
                    y: y as isize,
                }),
                None => {
                    let _ = freq_map.insert(
                        freq.to_owned(),
                        vec![Position {
                            x: x as isize,
                            y: y as isize,
                        }],
                    );
                }
            }
        }
    }

    let mut nodes = HashSet::<Position>::new();

    for (_, poses) in freq_map.iter() {
        for pos1 in poses {
            for pos2 in poses {
                if pos1 == pos2 {
                    continue;
                }
                nodes.extend(get_aninodes(pos1, pos2, width, height, extend));
            }
        }
    }

    nodes.len()
}

fn solve_part1(input: RawData) -> usize {
    solve(input, false)
}

fn solve_part2(input: RawData) -> usize {
    solve(input, true)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: RawData = r#"......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#."#;

    #[test]
    fn test_08_01() {
        assert_eq!(solve_part1(INPUT), 14);
    }

    #[test]
    fn test_08_02() {
        assert_eq!(solve_part2(INPUT), 34);
    }
}

fn main() {
    println!("PART 1: {}", solve_part1(INPUT));
    println!("PART 2: {}", solve_part2(INPUT));
}
