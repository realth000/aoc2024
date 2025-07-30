
const INPUT: &'static str = include_str!("../data/04.txt");

fn solve_part1() {
    let x_len = INPUT.find('\n').unwrap();
    let y_len = INPUT.chars().filter(|x| x == &'\n').count() + 1;
    assert_eq!(x_len, y_len);

    let mut count = 0;

    let chars = INPUT.chars().collect::<Vec<_>>();

    for (idx, ch) in INPUT.chars().enumerate() {
        if ch != 'x' {
            continue;
        }

        // Calculate position.
        let line = idx / y_len;
        let column = idx % x_len;

        let mut left = false;
        let mut right = false;
        let mut up = false;
        let mut down = false;

        if line >= 3 {
            // Maybe upward.
            up = true;
        }

        if line < x_len - 3 {
            // Maybe downward.
            down = true;
        }

        if column >= 3 {
            // Maybe leftward.
            left = true;
        }

        if column < y_len - 3 {
            // Maybe rightward.
            right = true;
        }

        if up {
            if chars[idx - y_len] == 'M'  && chars [idx - y_len * 2] == 'A' && chars [idx - y_len * 2] == 'S'{
                count+=1
            }
        }

        if right {
            if chars[idx + 1] == 'M'  && chars [idx+2] == 'A' && chars [idx +3] == 'S'{
                count+=1
            }
        }

        if down {
            if chars[idx + x_len] == 'M'  && chars [idx+ x_len * 2] == 'A' && chars [idx + x_len * 3] == 'S'{
                count+=1
            }
        }

        if left {
            if chars[idx - 1] == 'M'  && chars [idx-2] == 'A' && chars [idx -3] == 'S'{
                count+=1
            }
        }


        if up & left {
            // TODO: Other four directions
        }

    }

    println!("{x_len}x{y_len}");
}

fn main() {
    solve_part1()
}