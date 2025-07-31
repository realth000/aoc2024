use aoc2024::RawData;

const INPUT: RawData = include_str!("../data/01.txt");

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    // Parse into two groups of number.
    // 0    5  8
    // 55820   53096
    let mut first_group = vec![];
    let mut second_group = vec![];
    for line in INPUT.split("\n") {
        if line.trim().is_empty() {
            continue;
        }

        let first = line[0..5].parse::<i32>().unwrap();
        let second = line[8..].parse::<i32>().unwrap();
        first_group.push(first);
        second_group.push(second);
    }

    (first_group, second_group)
}

fn solve_part1() {
    let (mut first_group, mut second_group) = parse_input();
    first_group.sort();
    second_group.sort();
    let result = first_group
        .into_iter()
        .zip(second_group)
        .map(|x| (x.0 - x.1).abs())
        .reduce(|acc, x| acc + x)
        .unwrap();
    println!("PART 1: {result}");
}

fn solve_part2() {
    let (first_group, second_group) = parse_input();
    let result = first_group
        .into_iter()
        .map(|x| x * (second_group.iter().filter(|y| y == &&x).count() as i32))
        .reduce(|acc, x| acc + x)
        .unwrap();
    println!("PART 2: {result}");
}

fn main() {
    solve_part1();
    solve_part2();
}
