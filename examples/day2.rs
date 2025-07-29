const INPUT: &'static str = include_str!("../data/02.txt");

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
    enum Order {
        Increase,
        Decrease,
    }

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

fn solve_part2() {
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    enum Order {
        Increase,
        Decrease,
    }

    fn check_report(xs: Vec<i32>) -> i32 {
        if xs.len() < 2 {
            return 1;
        }

        let mut already_toleranting = false;

        let mut prev: Option<i32> = None;

        fn sample_order(x: &i32, y: &i32) -> Order {
            if x < y {
                Order::Increase
            } else {
                Order::Decrease
            }
        }

        // FIXME: Equality not works here.
        let orders = vec![
            sample_order(&xs[0], &xs[1]),
            sample_order(&xs[1], &xs[2]),
            sample_order(&xs[2], &xs[3]),
        ];

        let order = if orders.into_iter().filter(|x| x == &Order::Increase).count() > 1 {
            Order::Increase
        } else {
            Order::Decrease
        };

        for x in xs.into_iter() {
            match prev {
                Some(p) => {
                    let distance = (p - x).abs();
                    if distance > 3 || distance <= 0 {
                        if already_toleranting {
                            return 0;
                        } else {
                            already_toleranting = true;
                            continue;
                        }
                    }

                    match &order {
                        Order::Increase if p > x => {
                            if already_toleranting {
                                return 0;
                            } else {
                                already_toleranting = true;
                                continue;
                            }
                        }
                        Order::Decrease if p < x => {
                            if already_toleranting {
                                return 0;
                            } else {
                                already_toleranting = true;
                                continue;
                            }
                        }
                        _ => {
                            prev = Some(x);
                        }
                    }
                }
                None => prev = Some(x),
            }
        }

        1
    }

    let data = parse_input();
    let result = data.into_iter().fold(0, |acc, x| acc + check_report(x));
    println!("PART 2: {result}");
}

fn main() {
    solve_part1();
    solve_part2();
}
