use day3::*;

fn main() {
    use std::{fs::File, io::Read};

    let mut file = File::open("input").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let answer = part2(input);
    println!("{answer}");
}
