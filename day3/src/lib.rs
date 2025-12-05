use std::{
    collections::{HashMap, VecDeque},
    hash::{DefaultHasher, Hash, Hasher},
};

pub fn part1(input: String) -> i64 {
    let lines = input.lines();

    let mut sum: i64 = 0;
    for line in lines {
        let mut nums = Vec::new();
        for (idx, digit) in line.chars().enumerate() {
            let digit = digit as i64 - '0' as i64;
            nums.push((idx, digit));
        }

        nums.sort_unstable_by(|(_, a_digit), (_, b_digit)| b_digit.cmp(a_digit));

        let first = nums[0];
        let second = nums[1];
        if first.0 < second.0 {
            let bat = first.1 * 10 + second.1;
            sum += bat;
        } else {
            let mut bat = second.1 * 10 + first.1;
            for (idx, digit) in nums {
                if idx > first.0 {
                    let other_bat = first.1 * 10 + digit;
                    if other_bat > bat {
                        bat = other_bat;
                    }
                }
            }
            sum += bat;
        }
    }

    sum
}

fn get_highest_sequence(digits: &str, k: usize) -> i64 {
    let n = digits.len();
    let digits: Vec<i64> = digits.chars().map(|c| c as i64 - '0' as i64).collect();

    let mut stack = Vec::new();

    // want to remove the lowest digits at the front, as they have the biggest impact on the sum of
    // the 12 digit sequence
    for (i, digit) in digits.into_iter().enumerate() {
        while let Some(&last) = stack.last()
            && last < digit
            && (stack.len() - 1 + (n - i)) >= k
        {
            stack.pop();
        }

        if stack.len() < k {
            stack.push(digit);
        }
    }

    let mut max = 0;
    for digit in stack {
        max *= 10;
        max += digit;
    }

    return max;
}

pub fn part2(input: String) -> i64 {
    let lines = input.lines();

    let mut sum = 0;
    for line in lines {
        let max = get_highest_sequence(line, 12);
        sum += max;
    }

    sum
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

        assert_eq!(part1(input), 357);
    }

    #[test]
    fn part1_test() {
        let mut file = File::open("input").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        let answer = part1(input);

        assert_eq!(answer, 17229);
    }

    #[test]
    fn sample_part2() {
        let mut file = File::open("input-sample").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();

        assert_eq!(part2(input), 3121910778619);
    }

    #[test]
    fn part2_test() {
        let mut file = File::open("input").unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        let answer = part2(input);

        assert_eq!(answer, 170520923035051);
    }
}
