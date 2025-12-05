use std::collections::HashSet;

pub fn part1(input: String) -> i64 {
    let id_ranges: Vec<&str> = input.split(",").collect();

    let mut invalid_ids = 0;
    for id_range in id_ranges {
        let split: Vec<&str> = id_range.split("-").collect();
        let start: i64 = split[0].trim().parse().unwrap();
        let end: i64 = split[1].trim().parse().unwrap();

        for id in start..=end {
            let mut id_str = id.to_string();
            if id_str.len() % 2 == 0 {
                let first_half = id_str.split_off(id_str.len() / 2);
                if id_str == first_half {
                    invalid_ids += id;
                }
            }
        }
    }

    invalid_ids
}

fn does_pattern_continue(pattern: &str, mut rest: String) -> bool {
    if !rest.len().is_multiple_of(pattern.len()) {
        return false;
    }

    while !rest.is_empty() {
        let pat: String = rest.chars().take(pattern.len()).collect();
        if pat != pattern {
            return false;
        }

        rest = rest.chars().skip(pattern.len()).collect();
    }

    true
}

pub fn part2(input: String) -> i64 {
    let id_ranges: Vec<&str> = input.split(",").collect();

    let mut invalid_ids = 0;
    for id_range in id_ranges {
        let split: Vec<&str> = id_range.split("-").collect();
        let start: i64 = split[0].trim().parse().unwrap();
        let end: i64 = split[1].trim().parse().unwrap();

        for id in start..=end {
            let id_str = id.to_string();

            for length in 1..=id_str.len() / 2 {
                let first_half = id_str.chars().take(length).collect::<String>();
                let second_half = id_str.chars().skip(length).take(length).collect::<String>();
                if first_half == second_half
                    && does_pattern_continue(&first_half, id_str.chars().skip(length * 2).collect())
                {
                    println!("invalid: {id}\n\t {first_half} = {second_half}");
                    invalid_ids += id;
                    break;
                }
            }
        }
    }

    invalid_ids
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn sample_part1() {
        let mut file = File::open("input-sample").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();

        assert_eq!(part1(input), 1227775554);
    }

    #[test]
    fn part1_test() {
        let mut file = File::open("input").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        let answer = part1(input);
        assert_eq!(answer, 23534117921);
    }

    #[test]
    fn sample_part2() {
        let mut file = File::open("input-sample").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();

        assert_eq!(part2(input), 4174379265);
    }

    #[test]
    fn part2_test() {
        let mut file = File::open("input").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        let answer = part2(input);
        assert_eq!(answer, 31755323497);
    }
}
