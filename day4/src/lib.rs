use std::collections::HashSet;

type Coord = (usize, usize);

fn adjs(grid: &[Vec<char>], row: isize, col: isize) -> Vec<Coord> {
    let coords = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    let mut found_coords = Vec::new();
    for (row_inc, col_inc) in coords {
        let to_check_row = row + row_inc;
        let to_check_col = col + col_inc;

        if to_check_row >= 0
            && to_check_row < grid.len() as isize
            && to_check_col >= 0
            && to_check_col < grid[0].len() as isize
            && grid[to_check_row as usize][to_check_col as usize] == '@'
        {
            found_coords.push((to_check_row as usize, to_check_col as usize));
        }
    }

    found_coords
}

pub fn part1(grid: Vec<Vec<char>>) -> usize {
    let mut paper_rolls = HashSet::new();
    for row_idx in 0..grid.len() {
        for col_idx in 0..grid[0].len() {
            if grid[row_idx][col_idx] == '@' {
                let adj = adjs(&grid, row_idx as isize, col_idx as isize);
                if adj.len() < 4 {
                    paper_rolls.insert((row_idx, col_idx));
                }
            }
        }
    }

    paper_rolls.len()
}

pub fn part2_helper(grid: &mut [Vec<char>]) -> usize {
    let mut paper_rolls = HashSet::new();
    for row_idx in 0..grid.len() {
        for col_idx in 0..grid[0].len() {
            if grid[row_idx][col_idx] == '@' {
                let adj = adjs(grid, row_idx as isize, col_idx as isize);
                if adj.len() < 4 {
                    paper_rolls.insert((row_idx, col_idx));
                    grid[row_idx][col_idx] = '.';
                }
            }
        }
    }

    paper_rolls.len()
}

pub fn part2(mut grid: Vec<Vec<char>>) -> usize {
    let mut count = 0;
    loop {
        let change = part2_helper(&mut grid);
        if change == 0 {
            break;
        }
        count += change;
    }

    count
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::*;

    fn parse_grid_from_file(file_name: &str) -> Vec<Vec<char>> {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);
        let mut grid = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let chars: Vec<char> = line.chars().collect();
            grid.push(chars);
        }

        grid
    }

    #[test]
    fn sample_part1() {
        let grid = parse_grid_from_file("input-sample");

        assert_eq!(part1(grid), 13);
    }

    #[test]
    fn part1_test() {
        let grid = parse_grid_from_file("input");

        assert_eq!(part1(grid), 1543);
    }

    #[test]
    fn sample_part2() {
        let grid = parse_grid_from_file("input-sample");

        assert_eq!(part2(grid), 43);
    }

    #[test]
    fn part2_test() {
        let grid = parse_grid_from_file("input");

        assert_eq!(part2(grid), 9038);
    }
}
