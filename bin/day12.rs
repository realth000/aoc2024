use aoc2024::RawData;

type World = Vec<Vec<char>>;
type CostMap<'a> = Vec<Area<'a>>;

const INPUT: RawData = include_str!("../data/12.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BorderPoint {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Area<'a> {
    points: Vec<Position<'a>>,
    borders: Vec<BorderPoint>,
}

impl<'a> Area<'a> {
    fn new(pos: Position<'a>, borders: Vec<BorderPoint>) -> Self {
        Self {
            points: vec![pos],
            borders,
        }
    }

    fn add_point(&mut self, pos: Position<'a>, borders: &Vec<BorderPoint>) {
        self.points.push(pos);
        self.borders.append(&mut borders.clone());
    }

    fn combine(&mut self, other: &mut Self) {
        self.points.append(&mut other.points);
        self.borders.append(&mut other.borders);
    }

    fn cost(&self) -> usize {
        self.points.len() * self.borders.len()
    }

    fn cost_with_side(&self, world_width: usize, world_height: usize) -> usize {
        let mut sum = 0;

        for y in 0..world_width {
            let mut test_line = " ".repeat(world_width).chars().collect::<Vec<_>>();
            for xs in self
                .borders
                .iter()
                .filter(|pos| pos.y == y)
                .map(|pos| pos.x)
            {
                test_line[xs] = '1';
            }

            test_line.dedup();
            println!(">>> {} x-axis({}): {:?}", self.points[0].ch, y, test_line);
            sum += test_line
                .iter()
                .collect::<String>()
                .trim()
                .split(' ')
                .count();
        }
        let w = sum.clone();

        for x in 0..world_height {
            let mut test_line = " ".repeat(world_height).chars().collect::<Vec<_>>();
            for ys in self
                .borders
                .iter()
                .filter(|pos| pos.x == x)
                .map(|pos| pos.y)
            {
                test_line[ys] = '1';
            }

            test_line.dedup();
            println!(">>> {} y-axis({}): {:?}", self.points[0].ch, x, test_line);
            sum += test_line
                .iter()
                .collect::<String>()
                .trim()
                .split(' ')
                .count();
        }

        println!(
            ">>> {}, {} * ({} + {}) = {}",
            self.points[0].ch,
            self.points.len(),
            w,
            sum,
            w * sum
        );

        self.points.len() * sum
    }

    fn is_area_adjacent(&self, other: &Self) -> bool {
        self.points
            .iter()
            .any(|x| other.points.iter().any(|y| x.is_adjacent(y)))
    }

    fn is_pos_adjacent(&self, pos: &'a Position) -> bool {
        self.points.iter().any(|x| x.is_adjacent(pos))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Position<'a> {
    x: usize,
    y: usize,
    ch: &'a char,
}

impl<'a> Position<'a> {
    fn is_adjacent(&self, other: &Self) -> bool {
        if self.ch != other.ch {
            return false;
        }

        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);

        dx <= 1 && dy <= 1 && !(dx == 1 && dy == 1)
    }
}

fn parse_world(input: RawData) -> World {
    input.split('\n').map(|x| x.chars().collect()).collect()
}

fn calculate_point_fence_length(
    position: &Position,
    world: &World,
    world_width: usize,
    world_height: usize,
) -> Vec<BorderPoint> {
    let mut borders = vec![];

    let x = position.x;
    let y = position.y;
    let ch = &world[x][y];

    if y == 0 || &world[x][y - 1] != ch {
        // Left.
        borders.push(BorderPoint {
            x,
            y,
            direction: Direction::Left,
        });
    }

    if x == 0 || &world[x - 1][y] != ch {
        // Up.
        borders.push(BorderPoint {
            x,
            y,
            direction: Direction::Up,
        });
    }

    if y == world_width - 1 || &world[x][y + 1] != ch {
        // Right.
        borders.push(BorderPoint {
            x,
            y,
            direction: Direction::Right,
        });
    }

    if x == world_height - 1 || &world[x + 1][y] != ch {
        // Down.
        borders.push(BorderPoint {
            x,
            y,
            direction: Direction::Down,
        });
    }

    borders
}

fn generate_cost_map<'a>(world: &'a World) -> CostMap<'a> {
    let mut tmp_map = CostMap::new();

    let world_width = world[0].len();
    let world_height = world.len();

    for (x, line) in world.iter().enumerate() {
        for (y, ch) in line.iter().enumerate() {
            let pos = Position { x, y, ch };

            let perimeter = calculate_point_fence_length(&pos, &world, world_width, world_height);

            match tmp_map.iter_mut().find(|x| x.is_pos_adjacent(&pos)) {
                Some(area) => area.add_point(pos, &perimeter),
                None => tmp_map.push(Area::new(pos, perimeter)),
            }
        }
    }

    fn combine_map<'a>(m2: &mut CostMap<'a>) -> CostMap<'a> {
        let mut m1 = CostMap::new();
        for area in m2.iter_mut() {
            match m1.iter_mut().find(|x| x.is_area_adjacent(area)) {
                Some(v) => v.combine(area),
                None => m1.push(area.clone()),
            }
        }

        m1
    }

    let cost_map = {
        let mut last_map = tmp_map.to_owned();
        let mut round = 0;
        loop {
            round += 1;
            println!(">>> combine round={round}");
            let tmp_map2 = combine_map(&mut tmp_map);
            tmp_map = tmp_map2;
            if tmp_map.iter().any(|x| x.points.is_empty()) {
                panic!("invalid combine: {tmp_map:?}");
            }
            if tmp_map == last_map {
                break tmp_map;
            }
            last_map = tmp_map.clone();
        }
    };

    cost_map
}

fn solve_part1(input: RawData) -> usize {
    let world = parse_world(input);
    let cost_map = generate_cost_map(&world);
    cost_map
        .into_iter()
        .map(|x| x.cost())
        .fold(0, |acc, x| acc + x)
}

fn solve_part2(input: RawData) -> usize {
    let world = parse_world(input);
    let world_width = world[0].len();
    let world_height = world.len();
    let cost_map = generate_cost_map(&world);
    cost_map
        .into_iter()
        .map(|x| x.cost_with_side(world_width, world_height))
        .fold(0, |acc, x| acc + x)
}

#[cfg(test)]
mod test {
    use super::*;

    #[rustfmt::skip]
    const INPUT: RawData = //
"\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_12_01() {
        assert_eq!(solve_part1(INPUT), 1930);
    }

    #[test]
    fn test_12_02() {
        assert_eq!(solve_part2(INPUT), 1206);
    }
}

fn main() {
    println!("PART 1: {}", solve_part1(INPUT));
    println!("PART 2: {}", solve_part2(INPUT));
}
