use std::collections::HashSet;

use aoc2024::RawData;

type World = Vec<Vec<char>>;

type Paths<'a> = Vec<PathsFromPoint<'a>>;

/// All paths toward reachable 9-points from the same point.
type PathsFromPoint<'a> = HashSet<SinglePathFromPoint<'a>>;

/// A single path.
type SinglePathFromPoint<'a> = Vec<Position<'a>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Direction {
    x: isize,
    y: isize,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position<'a> {
    x: usize,
    y: usize,
    data: &'a char,
}

impl<'a> std::fmt::Debug for Position<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

const INPUT: RawData = include_str!("../data/10.txt");
static PATHS: [&char; 10] = [&'0', &'1', &'2', &'3', &'4', &'5', &'6', &'7', &'8', &'9'];
static DIRECTIONS: [Direction; 4] = [
    Direction { x: -1, y: 0 },
    Direction { x: 0, y: 1 },
    Direction { x: 1, y: 0 },
    Direction { x: 0, y: -1 },
];

fn parse_world(input: RawData) -> World {
    input.split('\n').map(|x| x.chars().collect()).collect()
}

fn point_in_direction<'a>(
    world: &'a World,
    x: usize,
    y: usize,
    world_width: usize,
    world_height: usize,
    direction: &'static Direction,
) -> Option<Position<'a>> {
    if direction.x < 0 && x == 0 {
        // Can not move up.
        return None;
    }
    if direction.x > 0 && x >= world_height - 1 {
        // Can not move down.
        return None;
    }
    if direction.y < 0 && y == 0 {
        // Can not move left.
        return None;
    }
    if direction.y > 0 && y >= world_width - 1 {
        // Can not move right.
        return None;
    }

    let dx = direction.x;
    let dy = direction.y;

    let px = if dx < 0 {
        x - (-dx) as usize
    } else {
        x + dx as usize
    };

    let py = if dy < 0 {
        y - (-dy) as usize
    } else {
        y + dy as usize
    };

    Some(Position {
        x: px,
        y: py,
        data: &world[px][py],
    })
}

fn count_route_at_point<'a>(
    x: usize,
    y: usize,
    world: &'a World,
    world_width: usize,
    world_height: usize,
    step: usize,
    current_path: &SinglePathFromPoint<'a>,
) -> PathsFromPoint<'a> {
    let mut collected_paths = PathsFromPoint::new();
    for direction in DIRECTIONS.iter() {
        let next_point = point_in_direction(world, x, y, world_width, world_height, &direction);
        if let Some(Position { x, y, data }) = next_point {
            if data == PATHS[step] {
                let mut p = current_path.clone();
                p.push(Position { x, y, data });
                // Is a route.
                if step == PATHS.len() - 1 {
                    // Last step.
                    collected_paths.insert(p);
                } else {
                    collected_paths.extend(count_route_at_point(
                        x,
                        y,
                        world,
                        world_width,
                        world_height,
                        step + 1,
                        &p,
                    ));
                }
            }
        }
    }

    collected_paths
}

fn generate_paths<'a>(world: &'a World) -> Paths<'a> {
    let mut paths = Paths::new();

    let world_width = world[0].len();
    let world_height = world.len();

    for (x, line) in world.iter().enumerate() {
        for (y, position) in line.iter().enumerate() {
            if position != PATHS[0] {
                // Not the start point.
                continue;
            }
            paths.push(count_route_at_point(
                x,
                y,
                &world,
                world_width,
                world_height,
                1,
                &vec![Position {
                    x,
                    y,
                    data: &world[x][y],
                }],
            ));
        }
    }

    // println!(">>> ALL PATHS:");
    for target in paths.iter() {
        // let start_pos = &target.iter().next().unwrap()[0];
        // println!(">>> FROM {:?}", start_pos);
        for (_idx, path) in target.iter().enumerate() {
            if path.len() != PATHS.len() {
                panic!("invalid path length {:?}", path);
            }

            if path
                .iter()
                .enumerate()
                .any(|(idx, x)| (*x.data) as usize - 48 != idx)
            {
                panic!("invalid path {:?}", path,);
            }

            // println!(">>> {:?}: {}", start_pos, idx);
            // println!("        {:?}", path);
        }
    }

    paths
}

fn solve_part1(input: RawData) -> usize {
    let world = parse_world(input);
    let paths = generate_paths(&world);

    paths
        .into_iter()
        .map(|x| {
            let mut ends_record = vec![];
            for path in x.iter() {
                if ends_record.contains(path.last().unwrap()) {
                    continue;
                }
                ends_record.push(path.last().unwrap().clone());
            }

            ends_record.len()
        })
        .reduce(|acc, x| acc + x)
        .unwrap()
}

fn solve_part2(input: RawData) -> usize {
    let world = parse_world(input);
    let paths = generate_paths(&world);

    paths
        .into_iter()
        .map(|x| x.len())
        .reduce(|acc, x| acc + x)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[rustfmt::skip]
    const INPUT: RawData =
"\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_10_01() {
        assert_eq!(solve_part1(INPUT), 36);
    }

    #[test]
    fn test_10_02() {
        assert_eq!(solve_part2(INPUT), 81);
    }
}

fn main() {
    println!("PART 1: {}", solve_part1(INPUT));
    println!("PART 2: {}", solve_part2(INPUT));
}
