fn do_p1(input: &str) -> u32 {
    let (mut col1, mut col2) = str_to_col_vecs(input);
    col1.sort();
    col2.sort();
    // This could be done with a tuple but a struct is used for readbility
    struct FoldState {idx: usize, sum: u32}
    col1.into_iter().fold(FoldState {idx: 0, sum: 0}, |acc, c1| {
        FoldState {idx: acc.idx + 1, sum: acc.sum + c1.abs_diff(col2[acc.idx])}
    }).sum
}

fn do_p2(input: &str) -> u32 {
    let (col1, col2) = str_to_col_vecs(input);
    col1.into_iter()
        .map(|c1| col2.iter()
            .filter_map(|c2| match c1 == *c2 {
                true => Some(true),
                false => None
            })
            .count() as u32 * c1)
        .sum()
}

fn str_to_col_vecs(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.lines()
        .filter(|ln| ln.len() > 0)
        .fold((Vec::new(), Vec::new()), |mut acc, ln| {
            let [c1, c2] = ln.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap();

            acc.0.push(c1);
            acc.1.push(c2);

            acc
        })
}

fn main() {
    let input = std::fs::read_to_string("inputs/d01.txt").unwrap();
    println!("D01P01: {}", do_p1(&input));
    println!("D01P02: {}", do_p2(&input));
}

#[cfg(test)]
mod d1_test {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_p1() {
        assert_eq!(do_p1(INPUT), 11);
    }

    #[test]
    fn test_p2() {
        assert_eq!(do_p2(INPUT), 31);
    }
}
