const INPUT: &'static str = include_str!("../data/03.txt");

#[derive(Debug, Clone)]
struct MulStmt {
    lhs: u32,
    rhs: u32,
}

fn solve_part1() {
    let mut stmts = vec![];

    #[derive(Debug, Clone)]
    enum StmtState {
        None,
        M,
        Mu,
        Mul,
        LPar,
        Lhs(i32),
        Peroid,
        Rhs(i32),
        RPar,
    }

    struct State {
        state: StmtState,
        lhs: Option<u32>,
        rhs: Option<u32>,
    }

    impl State {
        fn new() -> Self {
            Self {
                state: StmtState::None,
                lhs: None,
                rhs: None,
            }
        }

        fn produce_stmt(&self) -> MulStmt {
            MulStmt {
                lhs: self.lhs.clone().unwrap(),
                rhs: self.rhs.clone().unwrap(),
            }
        }

        fn update_state(&mut self, state: StmtState) {
            self.state = state
        }

        fn push_lhs(&mut self, ch: char) {
            self.lhs = Some(self.lhs.unwrap_or_default() * 10 + ch.to_digit(10).unwrap());
        }

        fn push_rhs(&mut self, ch: char) {
            self.rhs = Some(self.rhs.unwrap_or_default() * 10 + ch.to_digit(10).unwrap());
        }

        fn reset(&mut self) {
            self.state = StmtState::None;
            self.lhs = None;
            self.rhs = None;
        }
    }

    let mut state = State::new();

    for ch in INPUT.chars() {
        match state.state.clone() {
            StmtState::None if ch == 'm' => state.update_state(StmtState::M),
            StmtState::M if ch == 'u' => state.update_state(StmtState::Mu),
            StmtState::Mu if ch == 'l' => state.update_state(StmtState::Mul),
            StmtState::Mul if ch == '(' => state.update_state(StmtState::LPar),
            StmtState::LPar if ch.is_digit(10) => {
                state.push_lhs(ch);
                state.update_state(StmtState::Lhs(1));
            }
            StmtState::Lhs(v) => {
                if ch.is_digit(10) {
                    if v == 3 {
                        state.reset();
                        continue;
                    }
                    state.push_lhs(ch);
                    state.update_state(StmtState::Lhs(v + 1));
                } else {
                    if v == 0 {
                        state.reset();
                        continue;
                    }

                    if ch == ',' {
                        state.update_state(StmtState::Peroid);
                        continue;
                    }

                    state.reset();
                }
            }
            StmtState::Peroid if ch.is_digit(10) => {
                state.push_rhs(ch);
                state.update_state(StmtState::Rhs(1));
            }
            StmtState::Rhs(v) => {
                if ch.is_digit(10) {
                    if v == 3 {
                        state.reset();
                        continue;
                    }

                    state.push_rhs(ch);
                    state.update_state(StmtState::Rhs(v + 1));
                } else {
                    if v == 0 {
                        state.reset();
                    }
                    if ch == ')' {
                        state.update_state(StmtState::RPar);
                        continue;
                    }
                    state.reset();
                }
            }
            StmtState::RPar => {
                stmts.push(state.produce_stmt());
                state.reset();
            }
            _ => state.reset(),
        }
    }

    let result = stmts.into_iter().fold(0, |acc, x| acc + x.lhs * x.rhs);
    println!("PART 1: {result}");
}

fn solve_part1_with_regex() {
    use regex::Regex;

    let mut stmts = vec![];
    let re = Regex::new(r#"mul\((?<lhs>(\d+){1,3}),(?<rhs>(\d+){1,3})\)"#).unwrap();
    for cap in re.captures_iter(INPUT) {
        let lhs = cap.name("lhs").unwrap().as_str().parse::<u32>().unwrap();
        let rhs = cap.name("lhs").unwrap().as_str().parse::<u32>().unwrap();
        stmts.push(MulStmt { lhs, rhs });
    }

    let result = stmts.into_iter().fold(0, |acc, x| acc + x.lhs * x.rhs);
    println!("PART 1 (with regex) : {result}");
}

fn main() {
    solve_part1();
    solve_part1_with_regex();
}
