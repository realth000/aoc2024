use aoc2024::RawData;

const INPUT: RawData = include_str!("../data/09.txt");

fn expand_disk(input: RawData) -> String {
    let mut out = String::new();

    let mut scanning_file = true;

    let mut file_id = -1;

    for ch in input.chars() {
        if scanning_file {
            file_id += 1;
            out += file_id
                .to_string()
                .repeat(ch.to_digit(10).unwrap() as usize)
                .as_str();
            scanning_file = false;
        } else {
            out += ".".repeat(ch.to_digit(10).unwrap() as usize).as_str();
            scanning_file = true;
        }
    }

    out
}

fn solve_part1(input: RawData) -> usize {
    let mut disk = expand_disk(input).chars().collect::<Vec<_>>();

    let mut pos = 0;
    let mut rpos = disk.len() - 1;

    loop {
        if pos >= rpos {
            break;
        }

        // pos next.
        let empty_block_idx = disk.iter().position(|x| x == &'.').unwrap();

        // rpos next.
        let file_block_idx = disk.iter().rposition(|x| x != &'.').unwrap();

        if empty_block_idx >= file_block_idx {
            break;
        }

        disk[empty_block_idx] = disk[file_block_idx];
        disk[file_block_idx] = '.';

        pos = empty_block_idx;
        rpos = file_block_idx;
    }

    let mut sum = 0;

    println!(">>> disk after compat: {}", disk.iter().collect::<String>());

    for (idx, block) in disk.iter().enumerate() {
        match block.to_digit(10) {
            Some(v) => sum += idx * (v as usize),
            None => break,
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use aoc2024::RawData;

    use super::*;

    const INPUT: RawData = r#"2333133121414131402"#;

    #[test]
    fn test_09_01() {
        assert_eq!(
            expand_disk(INPUT),
            "00...111...2...333.44.5555.6666.777.888899"
        );
        assert_eq!(solve_part1(INPUT), 1928);
    }
}

fn main() {
    println!("PART 1: {}", solve_part1(INPUT));
}
