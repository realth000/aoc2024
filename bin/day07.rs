use aoc2024::RawData;

const INPUT: RawData = include_str!("../data/07.txt");

#[derive(Debug)]
struct Expr {
    target: usize,
    operands: Vec<usize>,
}

impl Expr {
    fn from_line(input: &str) -> Self {
        let sep = input.find(':').unwrap();
        Self {
            target: input[..sep].parse().unwrap(),
            operands: input[(sep + 2)..]
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum FoldResult {
    /// Acc is more than target value.
    More,

    /// Acc is less than target value.
    Less,

    MoreOrLess,

    /// Acc equals target value.
    Eq,
}

fn solve_part1(input: RawData) -> usize {
    let mut sum = 0;

    fn fold_values(target: usize, acc: usize, xs: &[usize]) -> FoldResult {
        if xs.is_empty() {
            if target > acc {
                return FoldResult::Less;
            } else if target < acc {
                return FoldResult::More;
            } else {
                return FoldResult::Eq;
            }
        }

        let x = xs[0];

        let acc_a = acc + x;

        // Can not cut this branch because the operands are not sorted.
        // if target < acc_a {
        //     return FoldResult::More;
        // }

        let acc_m = if acc == 0 { 1 * x } else { acc * x };

        if xs.len() == 1 {
            if target == acc_a || target == acc_m {
                return FoldResult::Eq;
            }
            return FoldResult::MoreOrLess;
        }

        let next = fold_values(target, acc_a, &xs[1..]);
        if next == FoldResult::Eq {
            return FoldResult::Eq;
        }
        // Can not cut this branch because the operands are not sorted.
        // else if next == FoldResult::More {
        //     return FoldResult::More;
        // }

        fold_values(target, acc_m, &xs[1..])
    }

    for expr in input.split("\n").map(Expr::from_line) {
        let add_value = expr.operands.iter().fold(0, |acc, x| acc + x);
        let mul_value = expr.operands.iter().fold(1, |acc, x| acc * x);

        let max_value = std::cmp::max(add_value, mul_value);
        let min_value = std::cmp::min(add_value, mul_value);

        if expr.target == min_value || expr.target == max_value {
            sum += expr.target;
            continue;
        }

        // Can not cut this branch because value 1 as operand may break it.
        // if expr.target < min_value || expr.target > max_value {
        //     oh = true;
        // }

        if fold_values(expr.target, 0, &expr.operands.as_slice()) == FoldResult::Eq {
            sum += expr.target;
        }
    }
    sum
}

fn solve_part2(input: RawData) -> usize {
    let mut sum = 0;

    fn fold_values(target: usize, acc: usize, xs: &[usize]) -> FoldResult {
        if xs.is_empty() {
            if target > acc {
                return FoldResult::Less;
            } else if target < acc {
                return FoldResult::More;
            } else {
                return FoldResult::Eq;
            }
        }

        let x = xs[0];

        let acc_a = acc + x;

        let acc_m = if acc == 0 { 1 * x } else { acc * x };

        let mut level = 1;
        loop {
            if x / level < 10 {
                break;
            }
            level *= 10;
        }
        let acc_c = acc * level * 10 + x;

        if xs.len() == 1 {
            if target == acc_a || target == acc_m || target == acc_c {
                return FoldResult::Eq;
            }
            return FoldResult::MoreOrLess;
        }

        let next = fold_values(target, acc_a, &xs[1..]);
        if next == FoldResult::Eq {
            return FoldResult::Eq;
        }
        // Can not cut this branch because the operands are not sorted.
        // else if next == FoldResult::More {
        //     return FoldResult::More;
        // }

        if fold_values(target, acc_m, &xs[1..]) == FoldResult::Eq {
            return FoldResult::Eq;
        }

        fold_values(target, acc_c, &xs[1..])
    }

    for expr in input.split("\n").map(Expr::from_line) {
        if fold_values(expr.target, 0, &expr.operands.as_slice()) == FoldResult::Eq {
            sum += expr.target;
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use aoc2024::RawData;

    use crate::*;

    const INPUT: RawData = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test_07_01() {
        assert_eq!(solve_part1(INPUT), 3749);
    }

    #[test]
    fn test_07_02() {
        assert_eq!(solve_part2(INPUT), 11387);
    }
}

fn main() {
    println!("PART 1: {}", solve_part1(INPUT));
    println!("PART 2: {}", solve_part2(INPUT));
}
