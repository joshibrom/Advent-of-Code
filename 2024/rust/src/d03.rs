use regex::Regex;

#[derive(Debug)]
struct MulCmd {
    pub n1: isize,
    pub n2: isize
}

impl MulCmd {
    pub fn mul(&self) -> isize {
        self.n1 * self.n2
    }
}

impl std::str::FromStr for MulCmd {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ns = s.trim_start_matches("mul(").trim_end_matches(")").split(",")
            .map(|n| n.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        Ok(Self {
            n1: ns.first().unwrap().to_owned(),
            n2: ns.last().unwrap().to_owned()
        })
    }
}

fn parse_input(input: &str, enable_do_dont: bool) -> Vec<MulCmd> {
    let mut to_parse = input.to_string();
    if enable_do_dont {
        // Thanks to https://github.com/bluescreen/aoc2024/blob/master/src/p3.ts
        // for the regex on this part... Actually coulda done that myself, I
        // think.
        let re = Regex::new(r"mul\(\d+{1,3},\d+{1,3}\)|(?:do\([^)]*\)|don't\([^)]*\))")
            .unwrap();
        let caps = re.captures_iter(to_parse.as_str())
            .map(|c| c.extract::<0>().0)
            .collect::<Vec<_>>();
        let mut parsable: Vec<&str> = Vec::new();
        let mut add = true;
        for s in &caps {
            match s {
                &"do()" => add = true,
                &"don't()" => add = false,
                _ => if add {
                    parsable.push(s);
                }
            }
        }
        to_parse = parsable.into_iter().collect();
    }
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    re.captures_iter(to_parse.as_str())
        .map(|c| c.extract::<0>())
        .map(|c| c.0.parse().unwrap())
        .collect()
}

fn do_p1(input: &str) -> isize {
    parse_input(input, false).into_iter()
        .map(|mc| mc.mul())
        .sum()
}

fn do_p2(input: &str) -> isize {
    parse_input(input, true).into_iter()
        .map(|mc| mc.mul())
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("inputs/d03.txt").unwrap();
    println!("D03P01: {}", do_p1(input.as_str()));
    println!("D03P02: {}", do_p2(input.as_str()));
}

#[cfg(test)]
mod d03_tests {
    use super::*;


    #[test]
    fn test_p1() {
        const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(do_p1(INPUT), 161);
    }

    #[test]
    fn test_p2() {
        const INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(do_p2(INPUT), 48);
    }
}
