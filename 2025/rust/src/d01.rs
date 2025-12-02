#[derive(Debug, Clone, Copy)]
enum Direction {
    Left(usize),
    Right(usize),
}

impl Direction {
    fn apply(self: Self, click: usize) -> usize {
        let left = |n: usize, click: usize| match n > click {
            true => 100 - (n - click),
            false => click - n,
        };
        let right = |n: usize, click: usize| (click + n) % 100;
        match self {
            Self::Left(n) => left(n % 100, click),
            Self::Right(n) => right(n % 100, click),
        }
    }
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

fn perform_rotations(rots: Vec<Direction>) -> usize {
    rots.into_iter()
        .fold((50, 0), |(click, mut zero_count), d| {
            let new_click = d.apply(click.clone());
            zero_count += (new_click == 0) as usize;
            (new_click, zero_count)
        })
        .1
}

fn parse_input(input: &str) -> Vec<Direction> {
    input.lines().map(|ln| ln.parse().unwrap()).collect()
}

fn process_p1(input: &str) -> usize {
    perform_rotations(parse_input(input))
}

fn process_p2(input: &str) -> usize {
    let mut rots = Vec::new();
    // Expand to single-click rotations
    parse_input(input).into_iter().for_each(|d| match d {
        Direction::Left(n) => (0..n).for_each(|_| rots.push(Direction::Left(1))),
        Direction::Right(n) => (0..n).for_each(|_| rots.push(Direction::Right(1))),
    });
    perform_rotations(rots)
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
