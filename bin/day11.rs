use std::collections::HashMap;

use aoc2024::RawData;

const INPUT: RawData = include_str!("../data/11.txt");

type Stone = usize;

type StoneLine = Vec<Stone>;

fn should_split(stone: &Stone) -> Option<(Stone, Stone)> {
    if *stone < 10 {
        return None;
    }

    let digits = stone
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    if digits.len() % 2 == 0 {
        let (left, right) = digits.split_at(digits.len() / 2);
        let left = left
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, x)| 10_u32.pow(idx as u32) * x)
            .reduce(|acc, x| acc + x)
            .unwrap();
        let right = right
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, x)| 10_u32.pow(idx as u32) * x)
            .reduce(|acc, x| acc + x)
            .unwrap();
        Some((left as usize, right as usize))
    } else {
        None
    }
}

fn blink(stone_line: &mut StoneLine) {
    let mut pos = 0;
    loop {
        if pos > stone_line.len() - 1 {
            break;
        }

        let stone = &mut stone_line[pos];

        if *stone == 0 {
            *stone = 1;
            pos += 1;
            continue;
        }

        if let Some((left, right)) = should_split(stone) {
            *stone = left;
            stone_line.insert(pos + 1, right);
            pos += 2;
            continue;
        }

        *stone = (*stone) * 2024;
        pos += 1;
    }
}

/// This solution not works.
///
/// The complexity is too large because we still save the result of each round.
///
/// It's only 1 times faster than the original [blink] solution, not enough for 75 rounds.
#[allow(dead_code)]
fn blink_ex(stone_line: StoneLine, round: usize) -> usize {
    #[derive(Debug)]
    struct StoneEx {
        data: usize,
        round: usize,
    }

    impl StoneEx {
        fn still_blinking(&self) -> bool {
            self.round > 0
        }

        fn blink(&mut self) -> Option<StoneEx> {
            if !self.still_blinking() {
                return None;
            }

            self.round -= 1;

            if self.data == 0 {
                self.data = 1;
                None
            } else if let Some((left, right)) = should_split(&self.data) {
                self.data = left;
                Some(StoneEx {
                    data: right,
                    round: self.round,
                })
            } else {
                self.data *= 2024;
                None
            }
        }
    }

    let mut blinking_stones = stone_line
        .into_iter()
        .map(|data| StoneEx { data, round })
        .collect::<Vec<_>>();

    let mut count = 0;

    while !blinking_stones.is_empty() {
        let mut birthed_stones = vec![];

        for stone in blinking_stones.iter_mut() {
            if let Some(s) = stone.blink() {
                birthed_stones.push(s);
            }
        }

        let c1 = blinking_stones.len();
        blinking_stones.retain(|x| x.still_blinking());
        let c2 = blinking_stones.len();
        count += c1 - c2;

        blinking_stones.extend(birthed_stones);
    }

    count
}

/// This solution works.
///
/// Same numbers are calculated once, use a hash map to store the counts of each number.
fn blink_ex2(stone_line: StoneLine, round: usize) -> usize {
    fn add_or_insert(m: &mut HashMap<Stone, usize>, stone: Stone, value: usize) {
        match m.get_mut(&stone) {
            Some(v) => *v += value,
            None => {
                m.insert(stone, value);
            }
        }
    }

    let mut round_result = HashMap::<Stone, usize>::new();
    for stone in stone_line {
        add_or_insert(&mut round_result, stone, 1);
    }

    fn blink_round(stones: &HashMap<Stone, usize>) -> HashMap<Stone, usize> {
        let stone_keys = stones.keys().map(|x| x.to_owned()).collect::<Vec<_>>();
        let mut calculated = HashMap::new();
        for stone in stone_keys {
            let stone_count = stones[&stone];
            if stone == 0 {
                add_or_insert(&mut calculated, 1, stone_count);
                continue;
            }

            if let Some((left, right)) = should_split(&stone) {
                add_or_insert(&mut calculated, left, stone_count);
                add_or_insert(&mut calculated, right, stone_count);
                continue;
            }

            add_or_insert(&mut calculated, stone * 2024, stone_count);
        }

        calculated
    }

    for _round in 0..round {
        round_result = blink_round(&round_result);
    }
    round_result.values().fold(0, |acc, x| acc + x.to_owned())
}

fn solve_01(input: RawData) -> usize {
    let mut stone_line: StoneLine = input.split(" ").map(|x| x.parse().unwrap()).collect();

    for _round in 0..25 {
        blink(&mut stone_line);
    }

    stone_line.len()
}

fn solve_02(input: RawData) -> usize {
    let stone_line: StoneLine = input.split(" ").map(|x| x.parse().unwrap()).collect();
    // blink_ex(stone_line, 25)
    blink_ex2(stone_line, 75)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: RawData = "125 17";

    #[test]
    fn test_split() {
        assert_eq!(should_split(&0), None);
        assert_eq!(should_split(&1), None);
        assert_eq!(should_split(&2), None);
        assert_eq!(should_split(&10), Some((1, 0)));
        assert_eq!(should_split(&100), None);
        assert_eq!(should_split(&1000), Some((10, 0)));
        assert_eq!(should_split(&123456), Some((123, 456)));
    }

    #[test]
    fn test_11_01() {
        assert_eq!(solve_01(INPUT), 55312);
    }

    #[test]
    fn test_blink_ex() {
        let stone_line: StoneLine = INPUT.split(" ").map(|x| x.parse().unwrap()).collect();
        // assert_eq!(blink_ex(stone_line.clone(), 25), 55312);
        assert_eq!(blink_ex2(stone_line, 25), 55312);
    }
}

fn main() {
    println!("PART 1: {}", solve_01(INPUT));
    println!("PART 2: {}", solve_02(INPUT));
}
