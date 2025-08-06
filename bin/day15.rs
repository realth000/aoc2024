use std::collections::HashMap;

use aoc2024::RawData;

const INPUT_WORLD: RawData = include_str!("../data/15_01.txt");
const INPUT_MOVES: RawData = include_str!("../data/15_02.txt");

type World = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq)]
enum Movement {
    Left,
    Up,
    Right,
    Down,
}

impl Movement {
    fn from_char(c: &char) -> Self {
        match c {
            '<' => Movement::Left,
            '^' => Movement::Up,
            '>' => Movement::Right,
            'v' => Movement::Down,
            v => panic!("invalid move \"{v}\""),
        }
    }
}

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn add_offset(
        &self,
        mov: &Movement,
        world_width: usize,
        world_height: usize,
    ) -> Option<Position> {
        match mov {
            Movement::Left => {
                if self.y <= 0 {
                    None
                } else {
                    Some(Position {
                        x: self.x,
                        y: self.y - 1,
                    })
                }
            }
            Movement::Up => {
                if self.x <= 0 {
                    None
                } else {
                    Some(Position {
                        x: self.x - 1,
                        y: self.y,
                    })
                }
            }
            Movement::Right => {
                if self.y >= world_width - 2 {
                    None
                } else {
                    Some(Position {
                        x: self.x,
                        y: self.y + 1,
                    })
                }
            }
            Movement::Down => {
                if self.x >= world_height - 2 {
                    None
                } else {
                    Some(Position {
                        x: self.x + 1,
                        y: self.y,
                    })
                }
            }
        }
    }
}

#[derive(Debug)]
enum Movable {
    /// Just move the point to `to`.
    Movable {
        to: Position,
    },

    Swap {
        from: Position,
        to: Position,
    },

    Push {
        to: Position,
        points: Vec<Position>,
    },

    /// Boexes is adjacent till the wall.
    NotMovable,
}

fn world_to_string(world: &World) -> String {
    world
        .iter()
        .map(|x| x.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Not recursive.
///
/// Moveing horizontally is simple: searching for the next position of '.' in the line
/// of `pos` in direction of `movement`.
///
/// * If we find one, move here.
/// * If not found, not movable.
///
/// Only returns [`Movable::Movable`] or [`Movable::NotMovable`].
fn check_movable_horizontal(world: &World, pos: &Position, movement: &Movement) -> Movable {
    let ch = &world[pos.x][pos.y];
    if ch != &'@' {
        panic!("invalid horizontal movable check.");
    }
    let empty_space = match movement {
        Movement::Right | Movement::Down => panic!("invalid horizontal movement"),
        Movement::Left => world[pos.x]
            .iter()
            .skip(pos.x)
            .rev()
            .position(|c| c == &'.'),
        Movement::Up => world[pos.x].iter().skip(pos.x).position(|c| c == &'.'),
    };

    match empty_space {
        Some(y) => Movable::Movable {
            to: Position { x: pos.x, y },
        },
        None => Movable::NotMovable,
    }
}

/// A vertical movement.
///
/// This case is complicated, because each box occupies two horizontal spaces, when pushing boxes in
/// up or down, more boxes may be pushed together.
///
/// ```console
/// .....
/// .[]..
/// ..[].
/// ...@.
/// ```
///
/// When '@' moves up:
///
/// ```console
/// .[]..
/// ..[].
/// ...@.
/// .....
/// ```
///
/// A box pushes another box in the same direction.
///
/// But like a horizontal move, we still looking for an empty space '.'.
fn check_movable_vertically(
    world: &World,
    world_width: usize,
    world_height: usize,
    pos: &Position,
    movement: &Movement,
) -> Movable {
    let ch = &world[pos.x][pos.y];
    if ch == &'#' {
        return Movable::NotMovable;
    } else if ch == &'.' {
        return Movable::Movable { to: pos.clone() };
    }

    let mut poses = vec![];

    let mut pushed_points = Vec::<Position>::new();

    let next_pos = match pos.add_offset(movement, world_width, world_height) {
        Some(v) => v,
        None => return Movable::NotMovable,
    };

    let next_ch = &world[next_pos.x][next_pos.y];

    // .....
    // .[]..
    // ..[].
    // ...@.
    //
    // []...
    // .[]..
    // .[]..
    // ..@..
    if next_ch == &'[' {
        poses.push(next_pos.clone());
        poses.push(Position {
            x: next_pos.x,
            y: next_pos.y + 1,
        });
    } else if next_ch == &']' {
        poses.push(Position {
            x: next_pos.x,
            y: next_pos.y - 1,
        });
    } else if next_ch == &'#' {
        return Movable::NotMovable;
    }

    for pos_in_check in poses.iter() {
        match check_movable_vertically(world, world_width, world_height, pos_in_check, movement) {
            Movable::Movable { .. } => pushed_points.push(pos_in_check.clone()),
            Movable::Swap { .. } => panic!("invalid e"),
            Movable::Push { mut points, .. } => {
                pushed_points.push(pos_in_check.clone());
                pushed_points.append(&mut points);
            }
            Movable::NotMovable => return Movable::NotMovable,
        }
    }

    if pushed_points.is_empty() {
        Movable::Movable { to: next_pos }
    } else {
        Movable::Push {
            to: next_pos.clone(),
            points: pushed_points,
        }
    }
}

fn check_movable(
    world: &World,
    world_width: usize,
    world_height: usize,
    pos: &Position,
    movement: &Movement,
) -> Movable {
    let curr = &world[pos.x][pos.y];
    if curr != &'@' {
        // Unreachable.
        panic!("invalid move target");
    }

    let mut next_point = pos.to_owned();
    let mut from_pos: Option<Position> = None;
    loop {
        next_point = match next_point.add_offset(movement, world_width, world_height) {
            Some(v) => v,
            None => return Movable::NotMovable,
        };
        // println!(">>> next: {:?} ({:?})", next_point, movement);

        let ch = &world[next_point.x][next_point.y];
        if ch == &'[' || ch == &']' {
            panic!("not available check");
        }

        if ch == &'O' {
            // Need move.
            if from_pos.is_none() {
                from_pos = Some(next_point.clone());
            }
            continue;
        }

        if ch == &'#' {
            return Movable::NotMovable;
        }

        if from_pos.is_some() {
            // We are pushing box.
            // Push here.
            return Movable::Swap {
                from: from_pos.unwrap(),
                to: next_point.clone(),
            };
        } else {
            return Movable::Movable { to: next_point };
        }
    }
}

fn scale_world(world: &World) -> World {
    let mut scaled_world = World::new();
    for line in world.iter() {
        let mut scaled_line = Vec::<char>::new();
        for pos in line.iter() {
            match pos {
                &'#' => {
                    scaled_line.push('#');
                    scaled_line.push('#');
                }
                &'O' => {
                    scaled_line.push('[');
                    scaled_line.push(']');
                }
                &'.' => {
                    scaled_line.push('.');
                    scaled_line.push('.');
                }
                &'@' => {
                    scaled_line.push('@');
                    scaled_line.push('.');
                }
                v => panic!("invalid world element {}", v),
            }
        }
        scaled_world.push(scaled_line);
    }

    scaled_world
}

fn solve_part1(input_world: RawData, input_moves: RawData) -> usize {
    let mut world: World = input_world
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();
    let moves: Vec<Movement> = input_moves
        .replace('\n', "")
        .chars()
        .map(|c| Movement::from_char(&c))
        .collect();

    let world_width = world[0].len();
    let world_height = world.len();

    let initial_x = world.iter().position(|row| row.contains(&'@')).unwrap();
    let initial_y = world[initial_x].iter().position(|pos| pos == &'@').unwrap();

    let mut pos = Position {
        x: initial_x,
        y: initial_y,
    };

    for mov in moves.iter() {
        match check_movable(&world, world_width, world_height, &pos, mov) {
            Movable::Movable { to } => {
                world[pos.x][pos.y] = '.';
                pos = to;
                world[pos.x][pos.y] = '@';
            }
            Movable::Swap { from, to } => {
                world[from.x][from.y] = '.';
                world[to.x][to.y] = 'O';
                world[pos.x][pos.y] = '.';
                pos = from;
                world[pos.x][pos.y] = '@';
            }
            Movable::NotMovable => continue,
            Movable::Push { .. } => panic!("Push is not allowed in PART 1"),
        }
    }

    let mut sum = 0;

    for (row, line) in world.iter().enumerate() {
        for (col, pos) in line.iter().enumerate() {
            if pos == &'O' {
                sum += 100 * row + col;
            }
        }
    }

    sum
}

fn solve_part2(input_world: RawData, input_moves: RawData) -> usize {
    let world: World = input_world
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();
    let mut world = scale_world(&world);
    let moves: Vec<Movement> = input_moves
        .replace('\n', "")
        .chars()
        .map(|c| Movement::from_char(&c))
        .collect();

    let world_width = world[0].len();
    let world_height = world.len();

    let initial_x = world.iter().position(|row| row.contains(&'@')).unwrap();
    let initial_y = world[initial_x].iter().position(|pos| pos == &'@').unwrap();

    let mut pos = Position {
        x: initial_x,
        y: initial_y,
    };

    for mov in moves.iter() {
        let mut horizontal = false;
        let movable = if mov == &Movement::Left || mov == &Movement::Right {
            horizontal = true;
            check_movable_horizontal(&world, &pos, mov)
        } else {
            check_movable_vertically(&world, world_width, world_height, &pos, mov)
        };

        match movable {
            Movable::Movable { to } => {
                if horizontal {
                    let line = &mut world[to.x];
                    if to.y > pos.y {
                        // Move right.
                        for i in ((pos.y + 1)..=to.y).rev() {
                            line[i] = line[i - 1];
                        }
                        line[pos.y] = '.';
                        pos = to;
                    } else {
                        // Move left.
                        for i in to.y..(pos.y - 1) {
                            line[i] = line[i + 1];
                        }
                        line[pos.y] = '.';
                        pos = to;
                    }
                } else {
                    world[pos.x][pos.y] = '.';
                    pos = to;
                    world[pos.x][pos.y] = '@';
                }
            }
            Movable::Swap { .. } => panic!("invalid move result"),
            Movable::Push { to, points } => {
                // Always a vertical move.
                // TODO: Upword or downword.
                world[pos.x][pos.y] = '.';
                pos = to;
                world[pos.x][pos.y] = '@';
            }
            Movable::NotMovable => continue,
        }
    }

    let mut sum = 0;

    for (row, line) in world.iter().enumerate() {
        for (col, pos) in line.iter().enumerate() {
            if pos == &'O' {
                sum += 100 * row + col;
            }
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const WORLD_0: RawData = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########"#;

    const MOVES_0: RawData = r#"<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    const WORLD_1: RawData = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#;

    const MOVES_1: RawData = r#"<^^>>>vv<v>>v<<"#;

    const WORLD_0_SCALED: RawData = r#"####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################"#;

    const WORLD_0_SCALED_AFTER_MOVE: RawData = r#"####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################"#;

    #[test]
    fn test_scale_world() {
        assert_eq!(
            world_to_string(&scale_world(
                &WORLD_0
                    .split('\n')
                    .map(|x| x.chars().collect::<Vec<char>>())
                    .collect::<Vec<Vec<char>>>()
            )),
            WORLD_0_SCALED
        );
    }

    #[test]
    fn test_15_example_0() {
        assert_eq!(solve_part1(WORLD_0, MOVES_0), 10092);
    }

    #[test]
    fn test_15_example_1() {
        assert_eq!(solve_part1(WORLD_1, MOVES_1), 2028);
    }
}

fn main() {
    println!("PART 1: {}", solve_part1(INPUT_WORLD, INPUT_MOVES));
    println!("PART 2: {}", solve_part2(INPUT_WORLD, INPUT_MOVES));
}
