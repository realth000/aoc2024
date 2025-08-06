use aoc2024::RawData;

const INPUT: RawData = include_str!("../data/16.txt");

type World = Vec<Vec<char>>;

fn parse_world(input: RawData) -> World {
    input.split('\n').map(|x| x.chars().collect()).collect()
}

fn solve_part1(input: RawData) -> usize {
    let world = parse_world(input);

    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: RawData = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    const EXAMPLE_2: RawData = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    #[test]
    fn test_example() {
        assert_eq!(solve_part1(EXAMPLE_1), 7036);
        assert_eq!(solve_part1(EXAMPLE_2), 11048);
    }
}

fn main() {
    println!("PART 1: {}", solve_part1(INPUT));
}
