use aoc2024::RawData;

const INPUT: RawData = include_str!("../data/03.txt");

#[derive(Debug, Clone)]
struct MulStmt {
    lhs: u32,
    rhs: u32,
}

#[derive(Debug, Clone)]
enum StmtState {
    None,
    M,
    Mu,
    Mul,
    LPar,
    Lhs(i32),
    Period,
    Rhs(i32),
}

#[derive(Debug, Clone)]
enum DoState {
    None,
    D,
    Do,
    LPar,
}

#[derive(Debug, Clone)]
enum DoNotState {
    None,
    D,
    Do,
    Don,
    DonQuote,
    DonQuoteT,
    LPar,
}

struct State {
    state: StmtState,
    do_state: DoState,
    do_not_state: DoNotState,
    enabled: bool,
    lhs: Option<u32>,
    rhs: Option<u32>,
}

impl State {
    fn new() -> Self {
        Self {
            state: StmtState::None,
            do_state: DoState::None,
            do_not_state: DoNotState::None,
            enabled: true,
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

    fn update_do_state(&mut self, do_state: DoState) {
        self.do_state = do_state
    }

    fn update_do_not_state(&mut self, do_not_state: DoNotState) {
        self.do_not_state = do_not_state
    }

    fn push_lhs(&mut self, ch: char) {
        self.lhs = Some(self.lhs.unwrap_or_default() * 10 + ch.to_digit(10).unwrap());
    }

    fn push_rhs(&mut self, ch: char) {
        self.rhs = Some(self.rhs.unwrap_or_default() * 10 + ch.to_digit(10).unwrap());
    }

    fn reset_do(&mut self) {
        self.do_state = DoState::None
    }

    fn reset_do_not(&mut self) {
        self.do_not_state = DoNotState::None
    }

    fn reset(&mut self) {
        self.state = StmtState::None;
        self.lhs = None;
        self.rhs = None;
    }
}

fn solve() {
    let mut stmts = vec![];

    let mut state = State::new();

    for ch in INPUT.chars() {
        match state.do_state.clone() {
            DoState::None if ch == 'd' => state.update_do_state(DoState::D),
            DoState::D if ch == 'o' => state.update_do_state(DoState::Do),
            DoState::Do if ch == '(' => state.update_do_state(DoState::LPar),
            DoState::LPar if ch == ')' => {
                state.enabled = true;
                state.reset_do();
            }
            _ => state.reset_do(),
        }

        match state.do_not_state.clone() {
            DoNotState::None if ch == 'd' => state.update_do_not_state(DoNotState::D),
            DoNotState::D if ch == 'o' => state.update_do_not_state(DoNotState::Do),
            DoNotState::Do if ch == 'n' => state.update_do_not_state(DoNotState::Don),
            DoNotState::Don if ch == '\'' => state.update_do_not_state(DoNotState::DonQuote),
            DoNotState::DonQuote if ch == 't' => state.update_do_not_state(DoNotState::DonQuoteT),
            DoNotState::DonQuoteT if ch == '(' => state.update_do_not_state(DoNotState::LPar),
            DoNotState::LPar if ch == ')' => {
                state.enabled = false;
                state.reset_do_not()
            }
            _ => state.reset_do_not(),
        }

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
                        state.update_state(StmtState::Period);
                        continue;
                    }

                    state.reset();
                }
            }
            StmtState::Period if ch.is_digit(10) => {
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
                        continue;
                    }
                    if ch == ')' {
                        if state.enabled {
                            stmts.push(state.produce_stmt());
                        }
                        state.reset();
                        continue;
                    }
                    state.reset();
                }
            }
            _ => state.reset(),
        }
    }

    let result = stmts.into_iter().fold(0, |acc, x| acc + x.lhs * x.rhs);
    println!("PART 1: {result}");
}

// fn solve_part1_with_regex() {
//     use regex::Regex;
//
//     let mut stmts = vec![];
//     let re = Regex::new(r#"mul\((?<lhs>(\d+){1,3}),(?<rhs>(\d+){1,3})\)"#).unwrap();
//     for cap in re.captures_iter(INPUT) {
//         let lhs = cap.name("lhs").unwrap().as_str().parse::<u32>().unwrap();
//         let rhs = cap.name("rhs").unwrap().as_str().parse::<u32>().unwrap();
//         stmts.push(MulStmt { lhs, rhs });
//     }
//
//     let result = stmts.into_iter().fold(0, |acc, x| acc + x.lhs * x.rhs);
//     println!("PART 1 (with regex) : {result}");
// }

fn main() {
    solve();
}
