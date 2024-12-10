use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Coord {
    r: isize,
    c: isize
}

impl Coord {
    pub fn new(r: isize, c: isize) -> Self {
        Self { r, c }
    }

    pub fn new_from_usize(r: usize, c: usize) -> Self {
        Self { r: r.try_into().unwrap(), c: c.try_into().unwrap() }
    }
}

#[derive(Clone, Copy, Eq, Debug, PartialEq, Hash)]
enum Direction {
    North, South, East, West
}

impl Direction {
    pub fn rot_clock(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MapChar {
    Empty, Blockage, Guard
}

impl std::str::FromStr for MapChar {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Empty),
            "#" => Ok(Self::Blockage),
            ">" | "<" | "^" | "v" => Ok(Self::Guard),
            _ => Err(format!("Unkown value {s}"))
        }
    }
}

fn parse_input(input: &str) -> (HashMap<Coord, MapChar>, (usize, usize)) {
    let mut board = HashMap::new();
    let input: Vec<Vec<_>> = input.lines()
        .map(|ln| ln.chars().map(|c| c.to_string()).collect())
        .collect();
    let mut r = 0;
    let mut c = 0;
    let mut n_cols = 0;
    for ln in input.iter() {
        for ch in ln.iter() {
            board.insert(Coord::new_from_usize(r, c), ch.as_str().parse().unwrap());
            c += 1;
        }
        n_cols = n_cols.max(c);
        c = 0;
        r += 1;
    }
    (board, (r, n_cols))
}

fn find_guard(board: &HashMap<Coord, MapChar>) -> Coord {
    board.iter()
        .filter_map(|(k, v)| match *v {
            MapChar::Guard => Some(k.clone()),
            _ => None
        })
        .collect::<Vec<Coord>>()
        .first()
        .unwrap()
        .to_owned()
}

fn get_guard_path(board: &HashMap<Coord, MapChar>, max_path_len: Option<usize>) -> Option<HashSet<Coord>> {
    let guard = find_guard(&board);
    let trans = HashMap::from([
        (Direction::West, (0, -1)),
        (Direction::East, (0, 1)),
        (Direction::North, (-1, 0)),
        (Direction::South, (1, 0)),
    ]);
    let mut pos = guard;
    let mut dir = Direction::North;
    let peek = |dir: &Direction, pos: &Coord| {
        let t = trans.get(dir).unwrap();
        Coord::new(pos.r + t.0, pos.c + t.1)
    };
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut path_len = 0;
    while let Some(mc) = board.get(&peek(&dir, &pos)) {
        path_len += 1;
        if let Some(n) = max_path_len {
            if path_len >= n {
                return None;
            }
        }
        visited.insert(pos);
        if mc == &MapChar::Blockage {
            dir = dir.rot_clock();
        }
        pos = peek(&dir, &pos);
    }
    Some(visited)
}

fn do_p1(input: &str) -> usize {
    let board = parse_input(input).0;
    match get_guard_path(&board, None) {
        Some(p) => p.len() + 1,
        None => 0
    }
}

fn do_p2(input: &str) -> usize {
    let (board, (n_rows, n_cols)) = parse_input(input);
    dbg!(n_rows, n_cols);
    let guard_pos = find_guard(&board);
    let guard_path = get_guard_path(&board, None).unwrap();
    let max_path_len = guard_path.len() * 2;
    (0..n_rows)
        .map(|r| {
            (0..n_cols)
                .map(|c| {
                    let mut new_board = board.clone();
                    let curr = Coord::new_from_usize(r, c);
                    if curr != guard_pos && guard_path.contains(&curr) {
                        new_board.entry(curr).and_modify(|ch| *ch = MapChar::Blockage);
                    } else {
                        return false;
                    }
                    match get_guard_path(&new_board, Some(max_path_len)) {
                        Some(_) => false,
                        None => true
                    }
                })
                .filter(|&b| b)
                .count()
        })
        .sum::<usize>() + 1
}

fn main() {
    let input = std::fs::read_to_string("inputs/d06.txt").unwrap();
    println!("D06P01: {}", do_p1(input.as_str()));
    println!("D06P02: {}", do_p2(input.as_str()));
}

#[cfg(test)]
mod d06_tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_p1() {
        assert_eq!(do_p1(INPUT), 41);
    }

    #[test]
    fn test_p2() {
        assert_eq!(do_p2(INPUT), 6);
    }
}
