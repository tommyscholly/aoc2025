use std::{
    collections::VecDeque,
    fs::File,
    io::Read,
    str::{Chars, FromStr},
};

fn parse_input1<'a>(
    file: &str,
    file_contents: &'a mut String,
) -> (Vec<Vec<&'a str>>, Vec<&'a str>) {
    let mut file = File::open(file).unwrap();
    file.read_to_string(file_contents).unwrap();

    let mut lines: Vec<&str> = file_contents.lines().collect();
    let operations = lines.pop().unwrap();
    let ops: Vec<&str> = operations.split_whitespace().collect();
    let nums: Vec<Vec<&str>> = lines
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .collect();

    (nums, ops)
}

#[derive(Debug)]
pub enum Op {
    Add,
    Mul,
}

impl Op {
    pub fn from_char(c: char) -> Self {
        match c {
            '+' => Self::Add,
            '*' => Self::Mul,
            _ => unimplemented!(),
        }
    }

    pub fn init(&self) -> i64 {
        match self {
            Op::Add => 0,
            Op::Mul => 1,
        }
    }
}

#[derive(Debug)]
pub struct Problem {
   pub  nums: Vec<i64>,
    pub op: Op,
}

impl Problem {
    pub fn execute(&self) -> i64 {
        self.nums
            .iter()
            .fold(self.op.init(), |acc, elem| match self.op {
                Op::Mul => acc * elem,
                Op::Add => acc + elem,
            })
    }
}

pub fn part2(file: &str) -> i64 {
    let mut file_contents = String::new();
    let mut file = File::open(file).unwrap();
    file.read_to_string(&mut file_contents).unwrap();

    let mut lines: Vec<&str> = file_contents.lines().collect();
    let operations = lines.pop().unwrap();
    let mut lines_chars: Vec<Chars<'_>> = lines.into_iter().map(|line| line.chars()).collect();
    let mut op_char_vec: VecDeque<char> = operations.chars().collect();

    let mut probs: Vec<Problem> = Vec::new();
    while !op_char_vec.is_empty() {
        let op = op_char_vec.pop_front().unwrap();
        let op = Op::from_char(op);
        let mut to_grab = 1;
        while let Some(' ') = op_char_vec.front() {
            to_grab += 1;
            op_char_vec.pop_front();
        }

        let mut num_chars: Vec<Vec<char>> = Vec::new();
        for line_nums in &mut lines_chars {
            let mut chars: Vec<char> = line_nums.take(to_grab).collect();
            if !op_char_vec.is_empty() {
                chars.pop();
            };
            num_chars.push(chars);
        }

        // convert to the actual numbers now
        let mut nums = Vec::new();
        for idx in (0..num_chars[0].len()).rev() {
            let mut num = 0;
            for num_char in &num_chars {
                let c = num_char[idx];
                if c == ' ' {
                    continue;
                } else {
                    let n = c as i64 - '0' as i64;
                    num *= 10;
                    num += n;
                }
            }
            nums.push(num);
        }

        probs.push(Problem { nums, op });
    }

    probs.into_iter().map(|p| p.execute()).sum()
}

pub fn part1(file: &str) -> i64 {
    let mut file_contents = String::new();
    let (nums, ops) = parse_input1(file, &mut file_contents);

    let mut tot = 0;
    for (i, op) in ops.into_iter().enumerate() {
        print!("{op}: ");
        let mut amt = 0;
        for row in &nums {
            let num: i64 = row[i].parse().unwrap();
            print!("{num} ");
            if amt == 0 {
                amt = num;
            } else {
                match op {
                    "+" => amt += num,
                    "*" => amt *= num,
                    _ => unreachable!(),
                }
            }
        }

        tot += amt;
    }

    tot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        assert_eq!(part1("input-sample"), 4277556);
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1("input"), 5346286649122);
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2("input-sample"), 3263827);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("input"), 10389131401929);
    }
}
