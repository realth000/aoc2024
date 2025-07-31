use aoc2024::RawData;

const INPUT_RULES: RawData = include_str!("../data/05_01.txt");
const INPUT_UPDATES: RawData = include_str!("../data/05_02.txt");

type Update = Vec<usize>;

#[derive(Debug, Clone)]
struct Rule {
    before: usize,
    after: usize,
}

fn parse_rules(rules: RawData) -> Vec<Rule> {
    let mut all_rules = Vec::with_capacity(rules.len());
    for rule in rules.split('\n') {
        let sep_pos = rule.find('|').unwrap();
        all_rules.push(Rule {
            before: rule[..sep_pos].parse::<usize>().unwrap(),
            after: rule[sep_pos + 1..].parse::<usize>().unwrap(),
        });
    }

    all_rules
}

fn parse_upates(updates: RawData) -> Vec<Update> {
    let mut all_updates = Vec::with_capacity(updates.len());
    for update in updates.split('\n') {
        let pages = update
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        all_updates.push(pages);
    }
    all_updates
}

fn find_mid(update: &Update) -> usize {
    update[update.len() / 2]
}

fn solve_part1(rules: RawData, updates: RawData) -> usize {
    let rules = parse_rules(rules);
    let updates = parse_upates(updates);

    fn check_update(update: &Update, rules: &Vec<Rule>) -> bool {
        for rule in rules {
            if let (Some(first_idx), Some(second_idx)) = (
                update.iter().position(|x| *x == rule.before),
                update.iter().position(|x| *x == rule.after),
            ) {
                if first_idx > second_idx {
                    return false;
                }
            }
        }

        return true;
    }

    let result = updates
        .iter()
        .filter_map(|x| {
            if check_update(x, &rules) {
                Some(find_mid(x))
            } else {
                None
            }
        })
        .fold(0, |acc, x| acc + x);

    result
}

fn solve_part2(rules: RawData, updates: RawData) -> usize {
    let rules = parse_rules(rules);
    let mut updates = parse_upates(updates);

    fn fix_updates(update: &mut Update, rules: &Vec<Rule>) -> bool {
        let mut have_fix = false;
        // JUST DO IT
        for round in 1..=20 {
            for rule in rules {
                if let (Some(first_idx), Some(second_idx)) = (
                    update.iter().position(|x| *x == rule.before),
                    update.iter().position(|x| *x == rule.after),
                ) {
                    if first_idx > second_idx {
                        update.swap(first_idx, second_idx);
                        if round == 1 {
                            have_fix = true;
                        }
                    }
                }
            }
        }

        return have_fix;
    }

    let result = updates
        .iter_mut()
        .filter_map(|x| {
            if fix_updates(x, &rules) {
                Some(find_mid(x))
            } else {
                None
            }
        })
        .fold(0, |acc, x| acc + x);

    result
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_RULES: RawData = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13"#;

    const INPUT_UPDATES: RawData = r#"75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_05_01() {
        assert_eq!(solve_part1(INPUT_RULES, INPUT_UPDATES), 143);
    }

    #[test]
    fn test_05_02() {
        assert_eq!(solve_part2(INPUT_RULES, INPUT_UPDATES), 123);
    }
}

fn main() {
    println!("PART 1: {}", solve_part1(INPUT_RULES, INPUT_UPDATES));
    println!("PART 2: {}", solve_part2(INPUT_RULES, INPUT_UPDATES));
}
