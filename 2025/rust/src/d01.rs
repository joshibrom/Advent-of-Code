#[derive(Debug)]
enum Direction {
    Left(usize),
    Right(usize),
}

impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Get first character
        let d = s.chars().nth(0).map_or_else(
            || Err(String::from("Could not get first character")),
            |c| Ok(c),
        )?;
        // Get rest (the number)
        let n = (&s[1..])
            .parse()
            .map_err(|e| format!("Could not parse number: {e}"))?;
        match d {
            'L' => Ok(Self::Left(n)),
            'R' => Ok(Self::Right(n)),
            c => Err(format!("Invalid character: {c}")),
        }
    }
}

fn parse_input(input: &str) -> Vec<Direction> {
    input.lines().map(|ln| ln.parse().unwrap()).collect()
}

fn process_p1(input: &str) -> usize {
    let mut rb = std::collections::VecDeque::<usize>::with_capacity(100);
    for i in 0..100 {
        rb.push_back(i);
    }
    rb.rotate_right(50);
    parse_input(input)
        .into_iter()
        .map(|d| {
            let n1fn = |n: usize| n % 100;
            let n2fn = |n: usize, n1: usize| (n - n1) % 100;
            match d {
                Direction::Left(n) => {
                    let n1 = n1fn(n);
                    let n2 = n2fn(n, n1);
                    rb.rotate_left(n1);
                    rb.rotate_left(n2);
                }
                Direction::Right(n) => {
                    let n1 = n1fn(n);
                    let n2 = n2fn(n, n1);
                    rb.rotate_right(n1);
                    rb.rotate_right(n2);
                }
            };
            rb.front().unwrap().clone()
        })
        .filter(|n| n == &0)
        .count()
}

fn process_p2(input: &str) -> usize {
    let mut rb = std::collections::VecDeque::<usize>::with_capacity(100);
    for i in 0..100 {
        rb.push_back(i);
    }
    rb.rotate_right(50);
    parse_input(input)
        .into_iter()
        .flat_map(|d| match d {
            Direction::Left(n) => (0..n)
                .map(|_| {
                    rb.rotate_left(1);
                    rb.front().unwrap().clone()
                })
                .filter(|n| n == &0)
                .collect::<Vec<usize>>(),
            Direction::Right(n) => (0..n)
                .map(|_| {
                    rb.rotate_right(1);
                    rb.front().unwrap().clone()
                })
                .filter(|n| n == &0)
                .collect(),
        })
        .count()
}

fn main() {
    let input = std::fs::read_to_string("input/d01.txt").unwrap();
    println!("D01P01: {}", process_p1(&input));
    println!("D01P02: {}", process_p2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_works() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(process_p1(input), 3);
    }

    #[test]
    fn p2_works() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(process_p2(input), 6);
    }
}
