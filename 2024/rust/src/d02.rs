fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .filter(|ln| ln.len() > 0)
        .map(|ln| ln.split_ascii_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect())
        .collect()
}

fn check_line(line: Vec<u32>) -> bool {
    let is_incr = line.first() < line.last();
    let unidirectional = line.windows(2)
        .all(|ns| match is_incr {
            true => ns[0] < ns[1],
            false => ns[0] > ns[1]
        });
    if !unidirectional {
        return false;
    }
    line.windows(2)
        .all(|ns| {
            let d = ns[0].abs_diff(ns[1]);
            1 <= d && d <= 3
        })
}

fn do_p1(input: &str) -> usize {
    parse_input(input).into_iter()
        .map(|v| check_line(v))
        .filter(|&b| b)
        .count()
}

fn do_p2(input: &str) -> usize {
    let lines = parse_input(input);
    let mut safe_lines = std::collections::HashSet::new();
    for i in 0..lines.len() {
        let line = lines[i].clone();
        for j in 0..line.len() {
            let mut ln = line.clone();
            let _ = ln.remove(j);
            if check_line(ln) {
                safe_lines.insert(i);
            }
        }
    }
    safe_lines.len()
}

fn main() {
    let input = std::fs::read_to_string("inputs/d02.txt").unwrap();
    println!("D02P01: {}", do_p1(&input));
    println!("D02P02: {}", do_p2(&input));
}

#[cfg(test)]
mod d02_tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_p1() {
        assert_eq!(do_p1(&INPUT), 2);
    }

    #[test]
    fn test_p2() {
        assert_eq!(do_p2(&INPUT), 4);
    }
}
