const INPUT: &'static str = include_str!("../data/04.txt");

fn solve_part1(input: &'static str) -> i32 {
    let width = input.find('\n').unwrap();
    let height = input.chars().filter(|x| x == &'\n').count() + 1;

    // 140x140
    assert_eq!(width, height);

    let mut count = 0;

    let chars = input.chars().filter(|x| x != &'\n').collect::<Vec<_>>();

    for (idx, ch) in input.chars().filter(|x| x != &'\n').enumerate() {
        if ch != 'X' {
            continue;
        }

        // Calculate position.
        let line = idx / height;
        let column = idx % width;

        let mut left = false;
        let mut right = false;
        let mut up = false;
        let mut down = false;

        if line >= 3 {
            // Maybe upward.
            up = true;
        }

        if line < height - 3 {
            // Maybe downward.
            down = true;
        }

        if column >= 3 {
            // Maybe leftward.
            left = true;
        }

        if column < width - 3 {
            // Maybe rightward.
            right = true;
        }

        if up {
            if chars[idx - width] == 'M'
                && chars[idx - width * 2] == 'A'
                && chars[idx - width * 3] == 'S'
            {
                count += 1
            }
        }

        if right {
            if chars[idx + 1] == 'M' && chars[idx + 2] == 'A' && chars[idx + 3] == 'S' {
                count += 1
            }
        }

        if down {
            if chars[idx + width] == 'M'
                && chars[idx + width * 2] == 'A'
                && chars[idx + width * 3] == 'S'
            {
                count += 1
            }
        }

        if left {
            if chars[idx - 1] == 'M' && chars[idx - 2] == 'A' && chars[idx - 3] == 'S' {
                count += 1
            }
        }

        if up & left {
            if chars[idx - (width + 1)] == 'M'
                && chars[idx - (width + 1) * 2] == 'A'
                && chars[idx - (width + 1) * 3] == 'S'
            {
                count += 1
            }
        }

        if up & right {
            if chars[idx - (width - 1)] == 'M'
                && chars[idx - (width - 1) * 2] == 'A'
                && chars[idx - (width - 1) * 3] == 'S'
            {
                count += 1
            }
        }

        if down & left {
            if chars[idx + (width - 1)] == 'M'
                && chars[idx + (width - 1) * 2] == 'A'
                && chars[idx + (width - 1) * 3] == 'S'
            {
                count += 1
            }
        }

        if down & right {
            if chars[idx + (width + 1)] == 'M'
                && chars[idx + (width + 1) * 2] == 'A'
                && chars[idx + (width + 1) * 3] == 'S'
            {
                count += 1
            }
        }
    }

    count
}

fn solve_part2(input: &'static str) -> i32 {
    let width = input.find('\n').unwrap();
    let height = input.chars().filter(|x| x == &'\n').count() + 1;

    // 140x140
    assert_eq!(width, height);

    let mut count = 0;

    let chars = input.chars().filter(|x| x != &'\n').collect::<Vec<_>>();

    for (idx, ch) in input.chars().filter(|x| x != &'\n').enumerate() {
        if ch != 'A' {
            continue;
        }

        // Calculate position.
        let line = idx / height;
        let column = idx % width;

        if line < 1 || line > height - 2 || column < 1 || column > width - 2 {
            // Near the edge, not works.
            continue;
        }

        let top_left = chars[idx - (width + 1)];
        let top_right = chars[idx - (width - 1)];
        let bottom_left = chars[idx + (width - 1)];
        let bottom_right = chars[idx + (width + 1)];

        if ((top_left == 'M' && bottom_right == 'S') || (top_left == 'S' && bottom_right == 'M'))
            && ((top_right == 'M' && bottom_left == 'S')
                || (top_right == 'S' && bottom_left == 'M'))
        {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test_04_01() {
        assert_eq!(solve_part1(INPUT), 18);
    }

    #[test]
    fn test_04_02() {
        assert_eq!(solve_part2(INPUT), 9);
    }
}

fn main() {
    let part1 = solve_part1(INPUT);
    println!("PART 1: {part1}");

    let part2 = solve_part2(INPUT);
    println!("PART 2: {part2}");
}
