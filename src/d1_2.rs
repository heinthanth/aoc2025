use std::{env::args, fs, time::Instant};

fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let timer = Instant::now();

    let mut cur_loc = 50;
    let mut password = 0;

    for line in input.lines() {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let steps: i32 = chars.as_str().parse().unwrap();
        let is_left = dir == 'L';

        let first_zero = if cur_loc == 0 {
            100
        } else if is_left {
            cur_loc
        } else {
            100 - cur_loc
        };
        if steps >= first_zero {
            password += 1 + (steps - first_zero) / 100;
        }
        cur_loc = if is_left {
            (cur_loc - steps).rem_euclid(100)
        } else {
            (cur_loc + steps) % 100
        };
    }

    let elapsed = timer.elapsed();
    println!("Answer: {}, Time taken: {:?}", password, elapsed);
}
