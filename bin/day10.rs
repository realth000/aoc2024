use std::collections::HashSet;

use aoc2024::RawData;

type World = Vec<Vec<char>>;

type Targets = Vec<(usize, usize)>;

struct Direction {
    x: isize,
    y: isize,
}

struct Position<'a> {
    x: usize,
    y: usize,
    data: &'a char,
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

fn count_route_at_point(
    x: usize,
    y: usize,
    world: &World,
    world_width: usize,
    world_height: usize,
    step: usize,
) -> Targets {
    let mut targets = Targets::new();
    for direction in DIRECTIONS.iter() {
        let next_point = point_in_direction(world, x, y, world_width, world_height, &direction);
        if let Some(Position { x, y, data }) = next_point {
            if data == PATHS[step] {
                if step == 9 {
                    println!(">>> step {step} {x},{y}: {data}");
                }
                // Is a route.
                if step == PATHS.len() - 1 {
                    // Last step.
                    targets.push((x, y));
                } else {
                    // println!(">>> p >>>>> {step} {x},{y}: {data}");
                    targets.extend(count_route_at_point(
                        x,
                        y,
                        world,
                        world_width,
                        world_height,
                        step + 1,
                    ));
                }
            }
        }
    }
    // println!(">>> count at {x},{y} ={count}");
    targets
}

fn solve_part1(input: RawData) -> usize {
    let mut targets = Targets::new();

    let world = parse_world(input);

    let world_width = world[0].len();
    let world_height = world.len();

    for (x, line) in world.iter().enumerate() {
        for (y, position) in line.iter().enumerate() {
            if position != PATHS[0] {
                // Not the start point.
                continue;
            }
            targets.extend(count_route_at_point(
                x,
                y,
                &world,
                world_width,
                world_height,
                1,
            ));
        }
    }

    targets.len()
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
}

fn main() {
    println!("PART 1: {}", solve_part1(INPUT));
}
