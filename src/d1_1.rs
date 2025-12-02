use std::{env::args, fs, time::Instant};

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let timer = Instant::now();

    let mut cur_loc: i32 = 50;
    let mut password: i32 = 0;

    for line in input.lines() {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let steps = chars.as_str().parse::<i32>().unwrap();

        cur_loc += if dir == 'L' { -steps } else { steps };
        cur_loc = cur_loc.rem_euclid(100);
        password += if cur_loc == 0 { 1 } else { 0 };
    }

    let elapsed = timer.elapsed();
    println!("Answer: {}, Time taken: {:?}", password, elapsed);
}
