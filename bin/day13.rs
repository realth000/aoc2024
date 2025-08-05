use aoc2024::RawData;

const INPUT: RawData = include_str!("../data/13.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
struct Offset {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Button {
    offset: Offset,
    price: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ClawMachine {
    button_a: Button,

    button_b: Button,

    target: Offset,
}

impl ClawMachine {
    fn calculate_cost(&self) -> Option<usize> {
        let mut costs = vec![];

        for a_times in 0..=100 {
            for b_times in 0..=100 {
                if self.button_a.offset.x * a_times + self.button_b.offset.x * b_times
                    == self.target.x
                    && self.button_a.offset.y * a_times + self.button_b.offset.y * b_times
                        == self.target.y
                {
                    costs.push(self.button_a.price * a_times + self.button_b.price * b_times);
                }
            }
        }

        costs.iter().min().map(|x| x.to_owned())
    }

    fn calculate_cost_ex(&self) -> Option<usize> {
        let x = (self.target.x * self.button_b.offset.y - self.button_b.offset.x * self.target.y)
            / (self.button_b.offset.y * self.button_a.offset.x
                - self.button_b.offset.x * self.button_a.offset.y);
        let y = (self.target.y - self.button_a.offset.y * x) / (self.button_b.offset.y);

        if (self.button_a.offset.x * x + self.button_b.offset.x * y == self.target.x)
            && (self.button_a.offset.y * x + self.button_b.offset.y * y == self.target.y)
        {
            println!(">>> {x} {y}");
            Some(3 * x + y)
        } else {
            println!(">>> {x} {y} !");
            None
        }
    }

    fn from_text_block(block: &str) -> Self {
        let lines = block.split('\n').collect::<Vec<_>>();
        if lines.len() != 3 {
            panic!("invalid text block");
        }

        let mut a = lines[0].split(':').skip(1).next().unwrap().split(',');
        let ax = a
            .next()
            .map(|x| x.split('+').last().unwrap().parse::<usize>().unwrap())
            .unwrap();
        let ay = a
            .next()
            .map(|x| x.split('+').last().unwrap().parse::<usize>().unwrap())
            .unwrap();

        let mut b = lines[1].split(':').skip(1).next().unwrap().split(',');
        let bx = b
            .next()
            .map(|x| x.split('+').last().unwrap().parse::<usize>().unwrap())
            .unwrap();
        let by = b
            .next()
            .map(|x| x.split('+').last().unwrap().parse::<usize>().unwrap())
            .unwrap();

        let mut t = lines[2].split(':').skip(1).next().unwrap().split(',');
        let target_x = t
            .next()
            .unwrap()
            .split('=')
            .skip(1)
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let target_y = t
            .next()
            .unwrap()
            .split('=')
            .skip(1)
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Self {
            button_a: Button {
                offset: Offset { x: ax, y: ay },
                price: 3,
            },
            button_b: Button {
                offset: Offset { x: bx, y: by },
                price: 1,
            },
            target: Offset {
                x: target_x,
                y: target_y,
            },
        }
    }

    fn far_more(&mut self) -> &mut Self {
        self.target.x += 10000000000000;
        self.target.y += 10000000000000;
        self
    }
}

fn parse_machines(input: RawData) -> Vec<ClawMachine> {
    input
        .split("\n\n")
        .map(|block| ClawMachine::from_text_block(block))
        .collect()
}

fn solve_part1(input: RawData) -> usize {
    parse_machines(input)
        .iter()
        .filter_map(|x| x.calculate_cost())
        .reduce(|acc, x| acc + x)
        .unwrap_or(0)
}

fn solve_part2(input: RawData) -> usize {
    parse_machines(input)
        .iter_mut()
        .filter_map(|x| x.far_more().calculate_cost_ex())
        .reduce(|acc, x| acc + x)
        .unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: RawData = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    #[test]
    fn test_13_01() {
        assert_eq!(solve_part1(INPUT), 480);
    }
}

fn main() {
    println!("PART 1: {}", solve_part1(INPUT));
    println!("PART 2: {}", solve_part2(INPUT));
}
