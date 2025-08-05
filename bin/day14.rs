use aoc2024::RawData;

const INPUT: RawData = include_str!("../data/14.txt");
const WORLD_WIDTH: usize = 101;
const WORLD_HEIGHT: usize = 103;

fn safe_add(u: usize, i: isize, offset: usize) -> usize {
    let r = u as isize + i;
    if r < 0 {
        (r + offset as isize) as usize
    } else {
        r as usize % offset
    }
}

#[derive(Debug)]
struct Robot {
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    world_width: usize,
    world_height: usize,
}

impl Robot {
    fn from_line(input: &str, world_width: usize, world_height: usize) -> Self {
        let mut s = input.split(' ');
        let mut pos = s
            .next()
            .unwrap()
            .split('=')
            .skip(1)
            .next()
            .unwrap()
            .split(',');
        let x = pos.next().unwrap().parse().unwrap();
        let y = pos.next().unwrap().parse().unwrap();

        let mut v = s
            .next()
            .unwrap()
            .split('=')
            .skip(1)
            .next()
            .unwrap()
            .split(',');
        let dx = v.next().unwrap().parse().unwrap();
        let dy = v.next().unwrap().parse().unwrap();

        Self {
            x,
            y,
            dx,
            dy,
            world_width,
            world_height,
        }
    }

    fn update(&mut self) {
        self.x = safe_add(self.x, self.dx, self.world_width);
        self.y = safe_add(self.y, self.dy, self.world_height);
    }
}

fn check_tree(robots: &Vec<Robot>) -> bool {
    let has_robot = |robot: &Robot, row_offset: isize, col_offset: isize| -> bool {
        let target_x = safe_add(robot.x, row_offset, WORLD_WIDTH);
        let target_y = safe_add(robot.y, col_offset, WORLD_HEIGHT);
        robots.iter().any(|x| x.x == target_x && x.y == target_y)
    };

    for robot in robots.iter() {
        if robot.x < 2
            || robot.x > WORLD_WIDTH - 1 - 2
            || robot.y < 2
            || robot.y > WORLD_HEIGHT - 1 - 2
        {
            continue;
        }

        // Check for tree shape.
        //
        // ---> X-axis
        // |
        // |
        // V Y-axis
        //
        //    Current robot.
        //    |
        // ...#...
        // ..###..
        // .#####.
        // ...#...
        // ...#...
        if
        // 1st row.
        !has_robot(robot, -1, 0) && !has_robot(robot, 1, 0) &&
        // 2nd row
        !has_robot(robot, -2, 1) && has_robot(robot, -1, 1) && has_robot(robot, 0, 1) && has_robot(robot, 1, 1) &&
        // 3rd row
        !has_robot(robot, -3, 2) && has_robot(robot, -2, 2) && has_robot(robot, -1, 2)&& has_robot(robot, 0, 2) && has_robot(robot, 1, 2) && has_robot(robot, 2, 2) && !has_robot(robot, 3, 2) &&
        // 4th row
        has_robot(robot, 0, 3) &&
        // 5th row
        has_robot(robot, 0, 4)
        {
            return true;
        }
    }

    false
}

fn solve_part1(input: RawData, world_width: usize, world_height: usize) -> usize {
    let mut robots = input
        .split('\n')
        .map(|line| Robot::from_line(line, world_width, world_height))
        .collect::<Vec<_>>();

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.update();
        }
    }

    let c0 = robots
        .iter()
        .filter(|r| r.x < world_width / 2 && r.y < world_height / 2)
        .count();
    let c1 = robots
        .iter()
        .filter(|r| r.x > world_width / 2 && r.y < world_height / 2)
        .count();
    let c2 = robots
        .iter()
        .filter(|r| r.x < world_width / 2 && r.y > world_height / 2)
        .count();
    let c3 = robots
        .iter()
        .filter(|r| r.x > world_width / 2 && r.y > world_height / 2)
        .count();

    c0 * c1 * c2 * c3
}

fn solve_part2(input: RawData) -> usize {
    let mut robots = input
        .split('\n')
        .map(|line| Robot::from_line(line, WORLD_WIDTH, WORLD_HEIGHT))
        .collect::<Vec<_>>();

    let mut sec = 0;

    while !check_tree(&robots) {
        for robot in robots.iter_mut() {
            robot.update();
        }
        sec += 1;
    }
    println!(">>> update!");

    sec
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: RawData = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    const WORLD_WIDTH: usize = 11;
    const WORLD_HEIGHT: usize = 7;

    #[test]
    fn test_move() {
        let mut robot = Robot {
            x: 2,
            y: 4,
            dx: 2,
            dy: -3,
            world_width: WORLD_WIDTH,
            world_height: WORLD_HEIGHT,
        };
        robot.update();
        assert_eq!((robot.x, robot.y), (4, 1));
        robot.update();
        assert_eq!((robot.x, robot.y), (6, 5));
        robot.update();
        assert_eq!((robot.x, robot.y), (8, 2));
        robot.update();
        assert_eq!((robot.x, robot.y), (10, 6));
        robot.update();
        assert_eq!((robot.x, robot.y), (1, 3));
    }

    #[test]
    fn test_14_01() {
        assert_eq!(solve_part1(INPUT, WORLD_WIDTH, WORLD_HEIGHT), 12);
    }
}

fn main() {
    println!("PART 1: {}", solve_part1(INPUT, WORLD_WIDTH, WORLD_HEIGHT));
    println!("PART 2: {}", solve_part2(INPUT));
}
