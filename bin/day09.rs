use aoc2024::RawData;

const INPUT: RawData = include_str!("../data/09.txt");

type Block = Option<usize>;

fn pretty_disk(disk: Vec<Block>) -> String {
    disk.into_iter().map(|x| if x.is_some() { x.unwrap().to_string() } else { String::from(".") }).collect::<String>()
}

fn expand_disk(input: RawData) -> Vec<Block> {
    let mut out = vec![];

    let mut scanning_file = true;

    let mut file_id = 0;

    for ch in input.chars() {
        if scanning_file {
            let mut file = vec![Some(file_id); ch.to_digit(10).unwrap() as usize];
            out.append(&mut file);
            scanning_file = false;
            file_id += 1;
        } else {
            let mut empty_block_list = vec![None; ch.to_digit(10).unwrap() as usize];
            out.append(&mut empty_block_list);
            scanning_file = true;
        }
    }

    out
}

fn solve_part1(input: RawData) -> usize {
    let mut disk = expand_disk(input);

    let mut pos = 0;
    let mut rpos = disk.len() - 1;

    loop {
        if pos >= rpos {
            break;
        }

        // pos next.
        let empty_block_idx = disk.iter().position(|x| x.is_none()).unwrap();

        // rpos next.
        let file_block_idx = disk.iter().rposition(|x| x.is_some()).unwrap();

        if empty_block_idx >= file_block_idx {
            break;
        }

        disk[empty_block_idx] = disk[file_block_idx];
        disk[file_block_idx] = None;

        pos = empty_block_idx;
        rpos = file_block_idx;
    }

    let mut sum = 0;

    // println!(">>> disk after compat: {:?}", pretty_disk(disk.clone()));

    for (idx, block) in disk.iter().enumerate() {
        if block.is_none() {
            break;
        }
        sum += idx * block.unwrap()
    }

    sum
}

fn solve_part2(input: RawData) -> usize {
    let mut disk = expand_disk(input);

    let mut rpos = 0;

    // println!(">>> disk orig  compat: {}", pretty_disk(disk.clone()));

    loop {
        // Get next file to move.
        // rpos next.
        let file_block_right_pos = disk.len() - 1 - rpos - disk.iter().rev().skip(rpos).position(|x| x.is_some()).unwrap();
        let file_id = disk[file_block_right_pos];
        let file_block_offset = match disk.iter().rev().skip(disk.len() - 1 - file_block_right_pos).position(|x| *x != file_id) {
            None => break,
            Some(v) => v,
        };
        // println!(">>> file_id={file_id:?} {}..={}", file_block_right_pos - file_block_offset + 1, file_block_right_pos);
        let file_block_left_pos = file_block_right_pos - file_block_offset + 1;

        // Searching position in current file block round.
        let mut initial_pos = 0;
        // Search next suitable space.
        // empty block start position.
        let mut empty_block_left_pos = None;
        loop {
            let p = initial_pos + disk.iter().skip(initial_pos).position(|x| x.is_none()).unwrap();
            if p >= file_block_right_pos {
                // println!(">>> MOVE no space for file {}..={}", file_block_left_pos, file_block_right_pos);
                break;
            }
            // empty block end position, exclusive.
            let empty_block_offset = match disk.iter().skip(p).position(|x| *x != None) {
                None => break,
                Some(v) => v,
            };
            if empty_block_offset < file_block_offset {
                // The position has not enough space.
                initial_pos = p + empty_block_offset + 1;
                // println!(">>> no enough space: {empty_block_offset} < {file_block_offset}");
                continue;
            }

            empty_block_left_pos = Some(p);
            break;
        };

        if empty_block_left_pos.is_none() {
            rpos = disk.len() - 1 - (file_block_left_pos - 1);
            // println!(">>> skip file_id={file_id:?}, {file_block_left_pos}..={file_block_right_pos}, rpos={rpos}");
            continue;
        }

        let empty_block_left_pos = empty_block_left_pos.unwrap();

        if empty_block_left_pos >= disk.len() - 1 - rpos {
            // No suitable space for current file.
            rpos = disk.len() - 1 - (file_block_left_pos - 1);
            // println!(">>> skip2 file_id={file_id:?}, {file_block_left_pos}..={file_block_right_pos}, rpos={rpos}");
            continue;
        }

        let empty_block_right_pos = empty_block_left_pos + file_block_offset - 1;

        // Can move.

        // println!(">>> MOVE: {}..={} <- {}..={}", empty_block_left_pos, empty_block_right_pos, file_block_left_pos, file_block_right_pos);
        disk.splice(empty_block_left_pos..=empty_block_right_pos, vec![file_id; file_block_offset]);
        disk.splice(file_block_left_pos..=file_block_right_pos, vec![None; file_block_offset]);
        // println!(">>> disk after compat: {}", pretty_disk(disk.clone()));

        rpos = disk.len() - 1 - (file_block_left_pos - 1);

        if rpos == disk.len() - 1 {
            break;
        }
    }

    let mut sum = 0;

    // println!(">>> disk after compat: {:?}", pretty_disk(disk.clone()));

    for (idx, block) in disk.iter().enumerate() {
        if block.is_none() {
            continue;
        }
        sum += idx * block.unwrap()
    }

    sum
}

#[cfg(test)]
mod test {
    use aoc2024::RawData;

    use super::*;

    const INPUT: RawData = r#"2333133121414131402"#;

    #[test]
    fn test_expand_disk() {
        assert_eq!(
            pretty_disk(expand_disk(INPUT)).as_str(),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn test_09_01() {
        assert_eq!(solve_part1(INPUT), 1928);
    }


    #[test]
    fn test_09_02() {
        assert_eq!(solve_part2(INPUT), 2858);
    }
}

fn main() {
    println!("PART 1: {}", solve_part1(INPUT));
    println!("PART 2: {}", solve_part2(INPUT));
}
