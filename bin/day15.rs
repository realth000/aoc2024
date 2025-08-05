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
    fn apply_movement(
        &self,
        mov: &Movement,
        world_width: usize,
        world_height: usize,
    ) -> Option<Position> {
        match mov {
            Movement::Left => {
                if self.y == 0 {
                    None
                } else {
                    Some(Position {
                        x: self.x,
                        y: self.y - 1,
                    })
                }
            }
            Movement::Up => {
                if self.x == 0 {
                    None
                } else {
                    Some(Position {
                        x: self.x - 1,
                        y: self.y,
                    })
                }
            }
            Movement::Right => {
                if self.y == world_width - 2 {
                    None
                } else {
                    Some(Position {
                        x: self.x,
                        y: self.y + 1,
                    })
                }
            }
            Movement::Down => {
                if self.x == world_height - 2 {
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
    Movable { to: Position },

    /// Move the boxes, fiannly the point moves to `from` and the box originally at `from` is moved to `to`.
    Swap { from: Position, to: Position },

    /// Boexes is adjacent till the wall.
    NotMovable,
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
        next_point = match next_point.apply_movement(movement, world_width, world_height) {
            Some(v) => v,
            None => return Movable::NotMovable,
        };
        // println!(">>> next: {:?} ({:?})", next_point, movement);

        let ch = &world[next_point.x][next_point.y];
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
        }
        // for line in world.iter() {
        //     println!("{}", line.iter().collect::<String>());
        // }
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
}
