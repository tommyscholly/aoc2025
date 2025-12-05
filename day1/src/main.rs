use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut at = 50;
    let mut times = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        let mut chars = line.chars();
        let right = chars.next() == Some('R');
        let num: i32 = chars.collect::<String>().parse().unwrap();
        // if num >= 100 {
        //     times += num / 100;
        // }
        // let num = num % 100;
        if right {
            for _ in 0..num {
                at += 1;

                if at > 99 {
                    at = 0;
                }

                if at == 0 {
                    times += 1;
                }
            }
        } else {
            for _ in 0..num {
                at -= 1;

                if at < 0 {
                    at = 99;
                }

                if at == 0 {
                    times += 1;
                }
            }
            // at -= num;
            // if at < 0 {
            //     at += 100
            // }
        }

        // println!("{at}");
        // if at == 0 {
        //     times += 1;
        // }
    }

    println!("{times}");
}
