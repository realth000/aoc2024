use aoc2024::RawData;

type World = Vec<Vec<char>>;
type CostMap<'a> = Vec<Area<'a>>;

const INPUT: RawData = include_str!("../data/12.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
struct Area<'a> {
    points: Vec<Position<'a>>,
    perimeter: usize,
}

impl<'a> Area<'a> {
    fn new(pos: Position<'a>, perimeter: usize) -> Self {
        Self {
            points: vec![pos],
            perimeter,
        }
    }

    fn add_point(&mut self, pos: Position<'a>, perimeter: usize) {
        self.points.push(pos);
        self.perimeter += perimeter;
    }

    fn combine(&mut self, other: &mut Self) {
        self.points.append(&mut other.points);
        self.perimeter += other.perimeter;
    }

    fn cost(&self) -> usize {
        self.points.len() * self.perimeter
    }

    fn cost_with_side(&self, world_width: usize, world_height: usize) -> usize {
        let mut x_side = 0;
        let mut y_side = 0;

        for x in 0..world_height {
            let ys = self
                .points
                .iter()
                .filter(|pos| pos.x == x)
                .map(|pos| pos.y)
                .collect::<Vec<_>>();

            if ys.is_empty() {
                continue;
            }

            let mut counting = true;
            for y in 0..world_width {
                if ys.contains(&y) {
                    counting = true;
                } else {
                    if counting {
                        y_side += 1;
                        counting = false;
                    }
                }
            }
        }

        for y in 0..world_width {
            let xs = self
                .points
                .iter()
                .filter(|pos| pos.y == y)
                .map(|pos| pos.x)
                .collect::<Vec<_>>();

            if xs.is_empty() {
                continue;
            }

            let mut counting = true;
            for x in 0..world_height {
                if xs.contains(&x) {
                    counting = true;
                } else {
                    if counting {
                        x_side += 1;
                        counting = false;
                    }
                }
            }
        }
        println!(
            ">>> [size] {}, {} * ({} + {}) = {}",
            self.points[0].ch,
            self.points.len(),
            x_side,
            y_side,
            self.points.len() * (x_side + y_side),
        );

        if self.points[0].ch == &'S' {
            println!(">>> S: {:?}", self.points);
        }

        self.points.len() * (x_side + y_side)
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
) -> usize {
    let mut sum = 0;

    let x = position.x;
    let y = position.y;
    let ch = &world[x][y];

    if y == 0 || &world[x][y - 1] != ch {
        // Left.
        sum += 1;
    }

    if x == 0 || &world[x - 1][y] != ch {
        // Up.
        sum += 1;
    }

    if y == world_width - 1 || &world[x][y + 1] != ch {
        // Right.
        sum += 1;
    }

    if x == world_height - 1 || &world[x + 1][y] != ch {
        // Down.
        sum += 1;
    }

    sum
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
                Some(area) => area.add_point(pos, perimeter),
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

    // for area in cost_map.iter() {
    //     println!(
    //         ">>> {:?}, {} * {} = {}",
    //         area.points[0],
    //         area.points.len(),
    //         area.perimeter,
    //         area.points.len() * area.perimeter
    //     );
    // }

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

    const INPUT: RawData = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

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
