use std::{fs::File, io::Read};

type Board = Vec<Vec<char>>;

pub fn parse_file(file: &str) -> Board {
    let mut file = File::open(file).unwrap();
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents).unwrap();

    file_contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part1(file: &str) -> usize {
    let board = parse_file(file);
    let mut beam_board: Vec<Vec<bool>> = board
        .iter()
        .map(|row| row.iter().map(|c| *c == 'S').collect())
        .collect();

    let mut split = 0;
    for row in 0..beam_board.len() - 1 {
        let (current, next) = beam_board.split_at_mut(row + 1);
        let current_row = current.last().unwrap();
        let next_row = next.first_mut().unwrap();
        for (col, beam) in current_row.iter().copied().enumerate() {
            if beam {
                if board[row + 1][col] == '^' {
                    next_row[col - 1] = true;
                    next_row[col + 1] = true;
                    split += 1;
                } else {
                    next_row[col] = true;
                }
            }
        }
    }

    split
}

pub fn part2(file: &str) -> u64 {
    let board = parse_file(file);
    let mut beam_board: Vec<Vec<u64>> = board
        .iter()
        .map(|row| row.iter().map(|c| if *c == 'S' { 1 } else { 0 }).collect())
        .collect();

    for row in 0..beam_board.len() - 1 {
        let (current, next) = beam_board.split_at_mut(row + 1);
        let current_row = current.last().unwrap();
        let next_row = next.first_mut().unwrap();
        for (col, beam) in current_row.iter().copied().enumerate() {
            if beam > 0 && board[row + 1][col] == '^' {
                next_row[col - 1] += current_row[col];
                next_row[col + 1] += current_row[col];
            } else {
                next_row[col] += current_row[col];
            }
        }
    }

    beam_board[board.len() - 1].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let result = part1("input-sample");
        assert_eq!(result, 21);
    }

    #[test]
    fn part1_test() {
        let result = part1("input");
        assert_eq!(result, 1635);
    }

    #[test]
    fn sample_part2() {
        let result = part2("input-sample");
        assert_eq!(result, 40);
    }

    #[test]
    fn part2_test() {
        let result = part2("input");
        assert_eq!(result, 58097428661390);
    }
}
