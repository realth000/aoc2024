const INPUT: &'static str = include_str!("../data/02.txt");

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Order {
    Increase,
    Decrease,
}

fn parse_input() -> Vec<Vec<i32>> {
    INPUT
        .trim()
        .split("\n")
        .into_iter()
        .map(|x| {
            x.split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn solve_part1() {
    fn check_report(xs: Vec<i32>) -> i32 {
        if xs.len() < 2 {
            return 1;
        }

        let order = if xs[0] < xs[1] {
            Order::Increase
        } else {
            Order::Decrease
        };

        let mut prev: Option<i32> = None;

        for x in xs.into_iter() {
            match prev {
                Some(v) => {
                    let distance = (v - x).abs();
                    if distance > 3 || distance <= 0 {
                        return 0;
                    }

                    match order {
                        Order::Increase if v > x => return 0,
                        Order::Decrease if v < x => return 0,
                        _ => prev = Some(x),
                    }
                }
                None => prev = Some(x),
            }
        }

        1
    }

    let data = parse_input();
    let result = data.into_iter().fold(0, |acc, x| acc + check_report(x));
    println!("PART 1: {result}");
}

fn check_report2(xs: Vec<i32>) -> i32 {
    if xs.len() < 2 {
        return 1;
    }

    let mut already_toleranting = false;

    let mut backup: Option<(&i32, &i32)> = None;
    let mut prev: Option<&i32> = None;

    fn sample_order(x: &i32, y: &i32) -> Option<Order> {
        if x < y {
            Some(Order::Increase)
        } else if x > y {
            Some(Order::Decrease)
        } else {
            None
        }
    }

    fn work_with(x: &i32, y: &i32, order: &Order) -> bool {
        let distance = (x - y).abs();
        if distance > 3 || distance <= 0 {
            return false;
        }

        match order {
            Order::Increase if x > y => false,
            Order::Decrease if x < y => false,
            _ => true,
        }
    }

    // FIXME: Equality not works here.
    // 0 1 4 4 5
    //  i i e i   -> remove '4'
    //
    // 0 1 4 4 4
    //  i i e e   -> NOT PASSED
    //
    // 1 2 1 2 1
    //  i d i d   -> NOT PASSED
    let orders = vec![
        sample_order(&xs[0], &xs[1]),
        sample_order(&xs[1], &xs[2]),
        sample_order(&xs[2], &xs[3]),
        sample_order(&xs[3], &xs[4]),
    ];

    if orders.iter().filter(|x| x.is_none()).count() >= 2 {
        // To many equals.
        return 0;
    }

    let inc_count = orders
        .into_iter()
        .filter_map(|x| x)
        .filter(|x| x == &Order::Increase)
        .count();

    let order = if inc_count > 2 {
        Order::Increase
    } else if inc_count < 2 {
        Order::Decrease
    } else {
        // i i d d
        return 0;
    };

    let xs = if sample_order(&xs[0], &xs[1]) != Some(order.clone()) {
        already_toleranting = true;
        backup = Some((&xs[0], &xs[1]));
        prev = Some(&xs[0]);
        &xs[1..]
    } else {
        &xs[0..]
    };

    for x in xs.into_iter() {
        if let Some((p2, p)) = &backup {
            // Already tolerating.
            if work_with(p, x, &order) || work_with(p2, x, &order) {
                backup = None;
                prev = Some(&x);
                continue;
            }
            return 0;
        }

        match prev {
            Some(p) => {
                if work_with(p, x, &order) {
                    prev = Some(&x);
                } else {
                    if already_toleranting {
                        return 0;
                    } else {
                        already_toleranting = true;
                        backup = Some((&p, &x));
                        prev = Some(&x);
                        continue;
                    }
                }
            }
            None => prev = Some(&x),
        }
    }

    1
}

fn solve_part2() {
    let data = parse_input();
    let result = data.into_iter().fold(0, |acc, x| {
        let ret = check_report2(x.clone());
        if ret == 0 {
            println!("{x:?}");
        }

        acc + ret
    });
    println!("PART 2: {result}");
}

// Not works for:
//
// [18, 22, 23, 25, 26]
// [52, 59, 60, 61, 62]
// [81, 77, 76, 75, 74, 71]
// [18, 13, 12, 9, 7, 6]
// [88, 91, 90, 91, 93]
// [81, 85, 88, 89, 91, 93]
// [56, 61, 63, 65, 68, 71, 73]
// [53, 49, 47, 44, 42, 40, 38, 35]
// [83, 78, 76, 74, 72, 71, 69]
// [42, 46, 48, 51, 52]
// [53, 60, 62, 64, 67, 69, 72, 75]
// [33, 29, 27, 24, 21, 19]
// [77, 70, 67, 65, 62, 60, 57]
// [46, 39, 38, 36, 35, 34, 31]
// [60, 64, 66, 69, 72, 74, 76, 77]

fn main() {
    // println!("{}", check_report2(vec![18, 22, 23, 25, 26]));
    // return;
    solve_part1();
    solve_part2();
}
