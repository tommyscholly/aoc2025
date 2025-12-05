use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_ranges(reader: &mut BufReader<File>) -> Vec<(i64, i64)> {
    let mut range_set = Vec::new();

    loop {
        let mut id_range = String::new();
        reader.read_line(&mut id_range).unwrap();
        let id_range = id_range.trim();

        if id_range.is_empty() {
            break;
        }

        let split: Vec<&str> = id_range.split("-").collect();
        let start: i64 = split[0].trim().parse().unwrap();
        let end: i64 = split[1].trim().parse().unwrap();

        range_set.push((start, end));
    }

    range_set.sort_unstable();
    let mut idx = 0;
    while idx < range_set.len() - 1 {
        let current = range_set[idx];
        let next = range_set[idx + 1];
        if current.1 >= next.0 {
            if current.1 < next.1 {
                range_set[idx] = (current.0, next.1);
            }
            // we remove regardless, either current subsumes next, or we merge the two
            range_set.remove(idx + 1);
        } else {
            idx += 1;
        }
    }

    range_set
}

pub fn part1(mut reader: BufReader<File>) -> usize {
    let range_set = get_ranges(&mut reader);

    let mut count = 0;
    loop {
        let mut id = String::new();
        let Ok(_) = reader.read_line(&mut id) else {
            break;
        };
        let id = id.trim();
        if id.is_empty() {
            break;
        }

        let id: i64 = id.parse().unwrap();
        for (start, stop) in &range_set {
            if id >= *start && id <= *stop {
                count += 1;
                break;
            }
        }
    }

    count
}

pub fn part2(mut reader: BufReader<File>) -> i64 {
    let range_set = get_ranges(&mut reader);

    let mut count = 0;
    for range in range_set {
        count += range.1 - range.0 + 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let file = File::open("input-sample").unwrap();
        let res = part1(BufReader::new(file));
        assert_eq!(res, 3);
    }

    #[test]
    fn part1_test() {
        let file = File::open("input").unwrap();
        let res = part1(BufReader::new(file));
        assert_eq!(res, 617);
    }

    #[test]
    fn sample_part2() {
        let file = File::open("input-sample").unwrap();
        let res = part2(BufReader::new(file));
        assert_eq!(res, 14);
    }

    #[test]
    fn part2_test() {
        let file = File::open("input").unwrap();
        let res = part2(BufReader::new(file));
        assert_eq!(res, 338258295736104);
    }
}
