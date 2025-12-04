use std::{env::args, fs, time::Instant};

const POW10: [u64; 10] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
];

#[inline(always)]
fn is_invalid_number(number: u64) -> bool {
    let digit_count = number.ilog10() + 1;
    if digit_count & 1 != 0 {
        return false;
    }

    let half_10_pow = POW10[(digit_count >> 1) as usize];
    let left_half = number / half_10_pow;
    let right_half = number & (half_10_pow - 1);

    left_half == right_half
}

pub fn main() {
    let input = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    let timer = Instant::now();

    let mut invalid_numbers_sum = 0;

    for number_range in input.split(',') {
        let (start, end) = number_range.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();

        for number in start..=end {
            if is_invalid_number(number) {
                invalid_numbers_sum += number;
            }
        }
    }

    let elapsed = timer.elapsed();
    println!("Answer: {}, Time taken: {:?}", invalid_numbers_sum, elapsed);
}
