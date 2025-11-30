use nalgebra::{DMatrix, DMatrixView};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Chr { A, M, S, X, NE(char) }

impl Chr {
    pub fn match_xmas_slice(chrs: &[Self]) -> Option<()> {
        match chrs {
            &[Self::X, Self::M, Self::A, Self::S]
                | &[Self::S, Self::A, Self::M, Self::X] => Some(()),
            _ => None
        }
    }

    pub fn match_mas_slice(chrs: &[Self]) -> Option<()> {
        match chrs {
            &[Self::M, Self::A, Self::S]
                | &[Self::S, Self::A, Self::M] => Some(()),
            _ => None
        }
    }
}

impl std::str::FromStr for Chr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(format!("Recieved str of len {} but expected 1.", s.len()));
        }
        match s.chars().take(1).next().unwrap().to_ascii_lowercase() {
            'a' => Ok(Self::A),
            'm' => Ok(Self::M),
            's' => Ok(Self::S),
            'x' => Ok(Self::X),
            c => Ok(Self::NE(c))
        }
    }
}

fn parse_input(input: &str) -> DMatrix<Chr> {
    let board: Vec<Vec<Chr>> = input.lines()
        .filter(|ln| ln.len() > 0)
        .map(|ln| ln.chars()
            .map(|c| c.to_string().as_str().parse().unwrap())
            .collect())
        .collect();
    let flattend = board.clone().into_iter().flatten().collect();
    DMatrix::from_vec(board.len(), board[0].len(), flattend)
}

fn search_horiz(board: &DMatrix<Chr>) -> usize {
    board.row_iter()
        .map(|r| r.iter().map(|c| c.clone()).collect())
        .collect::<Vec<Vec<_>>>()
        .into_iter()
        .map(|ln| ln.windows(4).filter_map(|cs| Chr::match_xmas_slice(cs)).count())
        .sum()
}

fn search_vert(board: &DMatrix<Chr>) -> usize {
    search_horiz(&board.clone().transpose())
}

fn search_diag(board: &DMatrix<Chr>) -> usize {
    let mut n_words = 0;
    for r in 0..=(board.nrows() - 4) {
        for c in 0..=(board.ncols() - 4) {
            let view = board.view((r, c), (4, 4));
            let diag = view.diagonal().iter().map(|c| c.clone()).collect::<Vec<_>>();
            let tdiag = mirror(&view).diagonal().iter().map(|c| c.clone()).collect::<Vec<_>>();
            if let Some(_) = Chr::match_xmas_slice(diag.as_slice()) {
                n_words += 1;
            }
            if let Some(_) = Chr::match_xmas_slice(tdiag.as_slice()) {
                n_words += 1;
            }
        }
    }
    n_words
}

fn search_x_mas(board: &DMatrix<Chr>) -> usize {
    let mut n_words = 0;
    for r in 0..=(board.nrows() - 3) {
        for c in 0..=(board.ncols() - 3) {
            let view = board.view((r, c), (3, 3));
            let diag = view.diagonal().iter().map(|c| c.clone()).collect::<Vec<_>>();
            let tdiag = mirror(&view).diagonal().iter().map(|c| c.clone()).collect::<Vec<_>>();
            let mut is_valid = false;
            if let Some(_) = Chr::match_mas_slice(diag.as_slice()) {
                is_valid = true;
            }
            if let Some(_) = Chr::match_mas_slice(tdiag.as_slice()) {
                if is_valid {
                    n_words += 1;
                }
            }
        }
    }
    n_words
}

fn mirror(mtx: &DMatrixView<Chr>) -> DMatrix<Chr> {
    let mut v_mirror = mtx.row_iter()
        .map(|r| r.iter().map(|c| c.clone().to_owned()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    v_mirror.iter_mut().for_each(|r| r.reverse());
    let flattened = v_mirror.clone().into_iter().flatten().collect();
    DMatrix::from_vec(v_mirror.len(), v_mirror[0].len(), flattened)
}

fn do_p1(input: &str) -> usize {
    let b = parse_input(input);
    search_horiz(&b) + search_vert(&b) + search_diag(&b)
}

fn do_p2(input: &str) -> usize {
    search_x_mas(&parse_input(input))
}

fn main() {
    let input = std::fs::read_to_string("inputs/d04.txt").unwrap();
    println!("D04P01: {}", do_p1(input.as_str()));
    println!("D04P02: {}", do_p2(input.as_str()));
}

#[cfg(test)]
mod d04_tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(do_p1(input), 18);
    }

    #[test]
    fn test_p2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(do_p2(input), 9);
    }
}
