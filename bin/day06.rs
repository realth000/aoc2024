use std::{collections::HashSet, hash::Hash};

use aoc2024::RawData;

const INPUT: RawData = include_str!("../data/06.txt");

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: usize,
    y: usize,
}

/// Path describes the routes went before.
///
/// If the same [`Path`] occurs twice or more, then we are in a loop.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Path {
    from_pos: Position,
    to_pos: Position,
}

enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug)]
enum Item {
    Nothing,
    Something,
    Edge,
}

impl Item {
    fn from_char(ch: &char) -> Self {
        if ch == &'#' {
            return Self::Something;
        }
        Self::Nothing
    }
}

enum StopResult {
    NotStopped,
    ReachTheEdge,
    StuckInLoop,
}

mod state {
    use super::*;

    pub struct State {
        position: Position,
        direction: Direction,
        world: Vec<Vec<char>>,
        world_size: usize,
        paths: Paths,
    }

    impl State {
        pub fn new(start_position: Position, world: Vec<Vec<char>>) -> Self {
            let world_size = world.len();
            Self {
                position: start_position,
                direction: Direction::Up,
                world,
                world_size,
                paths: Paths::new(),
            }
        }

        pub fn produce_position(&mut self, p: &mut HashSet<Position>) -> StopResult {
            let pos = self.position.clone();
            p.insert(pos.clone());
            match self.get_next_position_item() {
                Item::Nothing => self.step_forward(),
                Item::Something => {
                    // Only record the path when turn direction.
                    if !self.paths.add_position(pos) {
                        // We are in a loop
                        return StopResult::StuckInLoop;
                    }
                    self.turn_right();
                    // This is the key of turning direction:
                    // No one promise we can step forward after turn right.
                    // Do the next round instead of defaultly step forward.
                    return StopResult::NotStopped;
                }
                Item::Edge => return StopResult::ReachTheEdge,
            }

            return StopResult::NotStopped;
        }

        fn turn_right(&mut self) {
            match &self.direction {
                Direction::Left => self.direction = Direction::Up,
                Direction::Up => self.direction = Direction::Right,
                Direction::Right => self.direction = Direction::Down,
                Direction::Down => self.direction = Direction::Left,
            }
        }

        /// --------> y-axis
        /// |
        /// |
        /// |
        /// |
        /// v
        ///
        /// x-axis
        fn step_forward(&mut self) {
            match &self.direction {
                Direction::Left => self.update_position(0, -1),
                Direction::Up => self.update_position(-1, 0),
                Direction::Right => self.update_position(0, 1),
                Direction::Down => self.update_position(1, 0),
            }
        }

        fn update_position(&mut self, dx: isize, dy: isize) {
            if dx >= 0 {
                self.position.x += dx as usize;
            } else {
                self.position.x -= (-dx) as usize;
            }

            if dy >= 0 {
                self.position.y += dy as usize;
            } else {
                self.position.y -= (-dy) as usize;
            }
        }

        fn get_next_position_item(&self) -> Item {
            match &self.direction {
                Direction::Left => {
                    if self.position.y < 1 {
                        Item::Edge
                    } else {
                        Item::from_char(&self.world[self.position.x][self.position.y - 1])
                    }
                }
                Direction::Up => {
                    if self.position.x < 1 {
                        Item::Edge
                    } else {
                        Item::from_char(&self.world[self.position.x - 1][self.position.y])
                    }
                }
                Direction::Right => {
                    if self.position.y > self.world_size - 2 {
                        Item::Edge
                    } else {
                        Item::from_char(&self.world[self.position.x][self.position.y + 1])
                    }
                }
                Direction::Down => {
                    if self.position.x > self.world_size - 2 {
                        Item::Edge
                    } else {
                        Item::from_char(&self.world[self.position.x + 1][self.position.y])
                    }
                }
            }
        }
    }

    #[derive(Clone)]
    struct Paths {
        /// All history paths we went through.
        paths: HashSet<Path>,

        /// The position of last step, use it when we would produce a new [`Path`].
        last_pos: Option<Position>,
    }

    impl Paths {
        fn new() -> Self {
            Self {
                paths: HashSet::new(),
                last_pos: None,
            }
        }

        /// Remember the position and update history path.
        ///
        /// Return false is already in a loop (duplicate paths occurred).
        #[must_use]
        fn add_position(&mut self, pos: Position) -> bool {
            if self.last_pos.is_none() {
                self.last_pos = Some(pos);
                return true;
            }

            let path = Path {
                from_pos: self.last_pos.clone().unwrap(),
                to_pos: pos.clone(),
            };

            if !self.paths.insert(path.clone()) {
                return false;
            }
            self.last_pos = Some(pos);

            return true;
        }
    }
}

/// Return the count of all points we ever went through.
///
/// If we got stuck in loop, return `None`.
fn solve_part1(input: &str) -> Option<HashSet<Position>> {
    let width = input.find('\n').unwrap();
    let height = input.chars().filter(|x| x == &'\n').count() + 1;

    let world = input
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start_idx = world
        .iter()
        .enumerate()
        .find_map(|(idx, line)| line.iter().position(|x| x == &'^').map(|p| p + idx * width))
        .unwrap();

    assert_eq!(width, height);

    let start_position = Position {
        x: start_idx / width,
        y: start_idx % width,
    };

    let mut all_poses = HashSet::new();
    let mut state = state::State::new(start_position, world);
    loop {
        match state.produce_position(&mut all_poses) {
            StopResult::NotStopped => continue,
            StopResult::ReachTheEdge => return Some(all_poses),
            StopResult::StuckInLoop => return None,
        }
    }
}

fn solve_part2(input: RawData) -> usize {
    let mut count = 0;

    let positions = solve_part1(input).unwrap();

    let width = input.find('\n').unwrap();
    let height = input.chars().filter(|x| x == &'\n').count() + 1;
    assert_eq!(width, height);

    let world = input
        .split('\n')
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_idx = world
        .iter()
        .enumerate()
        .find_map(|(idx, line)| line.iter().position(|x| x == &'^').map(|p| p + idx * width))
        .unwrap();

    let start_position = Position {
        x: start_idx / width,
        y: start_idx % width,
    };

    for pos in positions {
        let ch = &world[pos.x][pos.y];
        if ch == &'^' {
            continue;
        }

        let mut curr_world = world.clone();
        curr_world[pos.x][pos.y] = '#';

        let mut all_poses = &mut HashSet::new();
        let mut state = state::State::new(start_position.clone(), curr_world);
        loop {
            match state.produce_position(&mut all_poses) {
                StopResult::NotStopped => continue,
                StopResult::ReachTheEdge => break,
                StopResult::StuckInLoop => {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: RawData = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_06_01() {
        assert_eq!(solve_part1(INPUT).unwrap().len(), 41);
    }

    #[test]
    fn test_06_02() {
        assert_eq!(solve_part2(INPUT), 6);
    }
}

fn main() {
    println!("PART 1: {:?}", solve_part1(INPUT).unwrap().len());
    // 1834 too large.
    println!("PART 2: {:?}", solve_part2(INPUT));
}
