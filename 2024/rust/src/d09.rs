fn parse_input(input: &str) -> Vec<Option<usize>> {
    let mut disk = Vec::new();
    let mut ft = 0;

    for (i, c) in input.chars().enumerate() {
        if !c.is_digit(10) {
            continue;
        }
        match i % 2 == 0 {
            true => {
                for _ in 0..c.to_digit(10).unwrap() {
                    disk.push(Some(ft));
                }
                ft += 1;
            },
            false => {
                for _ in 0..c.to_digit(10).unwrap() {
                    disk.push(None);
                }
            }
        }
    }

    disk
}

fn compact_files(disk: &mut Vec<Option<usize>>) {
    let mut left = 0;
    let mut right = disk.len() - 1;
    while left < right {
        if disk[left].is_none() {
            while disk[right].is_none() {
                right -= 1;
            }
            disk[left] = disk[right].clone();
            disk[right] = None;
        }
        left += 1;
    }
}

fn get_free_space(disk: &Vec<Option<usize>>, start: usize) -> usize {
    let mut n_free = 0;
    let mut idx = start;
    while disk[idx].is_none() && idx < disk.len() - 1 {
        n_free += 1;
        idx += 1;
    }
    n_free
}

fn get_file_size(disk: &Vec<Option<usize>>, end: usize) -> usize {
    let mut size = 0;
    let mut idx = end;
    while disk[idx].is_some() && disk[idx] == disk[end] && idx > 0 {
        size += 1;
        idx -= 1;
    }
    size
}

fn defrag_files(disk: &mut Vec<Option<usize>>) {
    let mut right = disk.len() - 1;
    while right > 0 {
        if right % 1000 == 0 {
            dbg!(&right);
        }
        // Move left if there is no file here
        if disk[right].is_none() {
            right -= 1;
            continue;
        }
        // Check across the disk if this file may be moved
        let mut left = 0;
        while left < right {
            let free_space = get_free_space(&disk, left);
            let file_size = get_file_size(&disk, right);
            // If there is space to move the file, move it
            if free_space >= file_size {
                //dbg!(&left, &right, &free_space, &file_size, &disk[right]);
                for _ in 0..file_size {
                    disk[left] = disk[right];
                    disk[right] = None;
                    left += 1;
                    right -= 1;
                }
                break;
            } else {
                // There was no space here, move forward and check again
                left += 1.max(free_space);
            }
        }
        // Move left to check the next file
        right -= 1.max(get_file_size(&disk, right));
    }
}

fn do_p1(input: &String) -> usize {
    let mut disk = parse_input(input.as_str());
    compact_files(&mut disk);
    disk.into_iter().enumerate()
        .map(|(i, v)| match v {
            Some(n) => n * i,
            None => 0
        })
        .sum()
}

fn do_p2(input: &String) -> usize {
    let mut disk = parse_input(input.as_str());
    defrag_files(&mut disk);
    disk.into_iter().enumerate()
        .map(|(i, v)| match v {
            Some(n) => n * i,
            None => 0
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("inputs/d09.txt").unwrap();
    println!("D09P01: {}", do_p1(&input));
    println!("D09P02: {}", do_p2(&input));
}

#[cfg(test)]
mod d09_tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_parsing() {
        let disk = parse_input(INPUT);
        let expected = vec![Some(0), Some(0), None, None, None, Some(1),
            Some(1), Some(1), None, None, None, Some(2), None, None, None,
            Some(3), Some(3), Some(3), None, Some(4), Some(4), None, Some(5),
            Some(5), Some(5), Some(5), None, Some(6), Some(6), Some(6), Some(6),
            None, Some(7), Some(7), Some(7), None, Some(8), Some(8), Some(8),
            Some(8), Some(9), Some(9)];
        assert_eq!(disk, expected);
    }

    #[test]
    fn test_compacting() {
        let mut disk = parse_input(INPUT);
        let expected = vec![Some(0), Some(0), Some(9), Some(9), Some(8),
            Some(1), Some(1), Some(1), Some(8), Some(8), Some(8), Some(2),
            Some(7), Some(7), Some(7), Some(3), Some(3), Some(3), Some(6),
            Some(4), Some(4), Some(6), Some(5), Some(5), Some(5), Some(5),
            Some(6), Some(6), None, None, None, None, None, None, None, None,
            None, None, None, None, None, None];
        compact_files(&mut disk);
        assert_eq!(disk, expected);
    }

    #[test]
    fn test_defragmentation() {
        let mut disk = parse_input(INPUT);
        let expected = vec![Some(0), Some(0), Some(9), Some(9), Some(2),
            Some(1), Some(1), Some(1), Some(7), Some(7), Some(7), None, Some(4),
            Some(4), None, Some(3), Some(3), Some(3), None, None, None, None,
            Some(5), Some(5), Some(5), Some(5), None, Some(6), Some(6), Some(6),
            Some(6), None, None, None, None, None, Some(8), Some(8), Some(8),
            Some(8), None, None, ];
        defrag_files(&mut disk);
        assert_eq!(disk, expected);
    }

    #[test]
    fn test_p1() {
        assert_eq!(do_p1(&INPUT.to_string()), 1928);
    }

    #[test]
    fn test_p2() {
        assert_eq!(do_p2(&INPUT.to_string()), 2858);
    }
}
