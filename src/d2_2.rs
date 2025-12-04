use std::{env::args, fs, time::Instant};

const POW10: [u64; 20] = [
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
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
    10_000_000_000_000_000,
    100_000_000_000_000_000,
    1_000_000_000_000_000_000,
    10_000_000_000_000_000_000,
];

fn find_repeating_pattern(number: u64) -> Option<(u64, u32)> {
    let digit_count = number.ilog10() + 1;

    for pattern_len in 1..=(digit_count / 2) {
        if digit_count % pattern_len == 0 {
            let repeat_count = digit_count / pattern_len;

            let pattern = number / POW10[(digit_count - pattern_len) as usize];

            let multiplier = POW10[pattern_len as usize];
            let mut reconstructed = 0u64;
            for _ in 0..repeat_count {
                reconstructed = reconstructed * multiplier + pattern;
            }

            if reconstructed == number {
                return Some((pattern, repeat_count));
            }
        }
    }
    None
}

#[inline(always)]
fn is_invalid_number(number: u64) -> bool {
    find_repeating_pattern(number).is_some()
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
