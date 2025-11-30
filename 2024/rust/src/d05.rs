use std::collections::{HashMap, HashSet, VecDeque};

type Rules = HashMap<u16, Vec<u16>>;
type Updates = Vec<VecDeque<u16>>;

fn parse_input(input: &str) -> (Rules, Updates) {
    let (rule_str, update_str) = input.split_once("\n\n").unwrap();
    let rules = rule_str.lines()
        .fold(HashMap::new(), |mut hm, ln| {
            let (t, r) = ln.split_once("|").unwrap();
            let t = t.parse().unwrap();
            let r = r.parse().unwrap();
            hm.entry(t).and_modify(|rs: &mut Vec<_>| rs.push(r)).or_insert(vec![r]);
            hm
        });
    let updates = update_str.lines()
        .map(|ln| ln.split(",")
            .map(|n| n.parse().unwrap())
            .collect())
        .collect();
    (rules, updates)
}

fn check_update(update: &VecDeque<u16>, rules: &Rules) -> Option<VecDeque<u16>> {
    let mut vals_to_check = HashSet::new();
    let is_ordered = update.iter()
        .map(|n| {
            let valid = match rules.get(n) {
                Some(rs) => rs.iter().map(|r| vals_to_check.contains(r)).all(|b| !b),
                None => true
            };
            vals_to_check.insert(n);
            valid
        })
        .all(|b| b);
    if is_ordered {
        return Some(update.clone());
    } else {
        return None;
    }
}

fn do_p1(input: &str) -> u16 {
    let (rules, updates) = parse_input(input);
    let ordered = updates.into_iter()
        .filter_map(|update| check_update(&update, &rules))
        .collect::<Vec<_>>();
    ordered.into_iter()
        .map(|update| {
            let mid = f32::ceil(update.len() as f32 / 2f32) as usize - 1;
            update[mid]
        })
        .sum()
}

fn do_p2(input: &str) -> u16 {
    let (rules, updates) = parse_input(input);
    todo!()
}

fn main() {
    let input = std::fs::read_to_string("inputs/d05.txt").unwrap();
    println!("D05P01: {}", do_p1(input.as_str()));
}

#[cfg(test)]
mod d05_tests {
    use super::*;

    const INPUT: &str = "47|53
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
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_p1() {
        assert_eq!(do_p1(INPUT), 143);
    }

    #[test]
    fn test_p2() {
        assert_eq!(do_p2(INPUT), 123);
    }
}
