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
    let mid = (WORLD_WIDTH as isize) / 2;

    for x in 0..(WORLD_WIDTH as isize) {
        for y in 0..(WORLD_HEIGHT as isize) {
            if (x - mid).abs() <= y {
                if !robots
                    .iter()
                    .any(|r| r.x == x as usize && r.y == y as usize)
                {
                    return false;
                }
            }
        }
    }

    true
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

    while check_tree(&robots) {
        for robot in robots.iter_mut() {
            robot.update();
        }
        sec += 1;
        println!(">>> update!");
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
