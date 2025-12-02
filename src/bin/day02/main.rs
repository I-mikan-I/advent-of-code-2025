use aoc_25::*;
use regex::Regex;

fn prev_palindrome(num: usize, divisor: usize) -> bool {
    if divisor < 2 {
        return false;
    }
    let digits: usize = num.ilog10() as usize + 1;
    let result = if digits.is_multiple_of(divisor) {
        let parts: Vec<_> = (1..=divisor)
            .map(|i| {
                num / (10usize.pow(digits as u32 - (digits as u32 / divisor as u32) * i as u32))
                    % 10usize.pow(digits as u32 / divisor as u32)
            })
            .collect();
        // all equal?
        parts.len() > 1 && parts.iter().take(parts.len() - 1).eq(parts.iter().skip(1))
    } else {
        false
    };
    result || prev_palindrome(num, divisor - 1)
}

fn count_palindromes(mut from: usize, to: usize, divisor: usize) -> usize {
    let mut invalid = 0;
    let from_beg = from;
    while from <= to {
        let mut digits: usize = from.ilog10() as usize + 1;
        if !digits.is_multiple_of(divisor) {
            digits += 1;
            from = 10usize.pow(digits as u32 - 1);
        } else {
            let top = from / (10usize.pow(digits as u32 - digits as u32 / divisor as u32));
            from = 0;
            for _ in 0..divisor {
                from *= 10usize.pow(digits as u32 / divisor as u32);
                from += top;
            }
            if from <= to && from >= from_beg && !prev_palindrome(from, divisor - 1) {
                invalid += from;
            }
            from += 10usize.pow(digits as u32 - digits as u32 / divisor as u32);
        }
    }
    invalid
}

fn part1() {
    let input = read_input("day02.txt");
    let r = Regex::new(r"(\d+)-(\d+)").unwrap();
    let full = input.lines().fold(String::new(), |s, n| s + n);
    let ranges: Vec<_> = full
        .split(',')
        .map(|s| <(usize, usize)>::extract(&r, s))
        .collect();

    let mut invalid = 0;

    for (from, to) in ranges {
        invalid += count_palindromes(from, to, 2);
    }
    println!("{}", invalid);
}

fn part2() {
    let input = read_input("day02.txt");
    let r = Regex::new(r"(\d+)-(\d+)").unwrap();
    let full = input.lines().fold(String::new(), |s, n| s + n);
    let ranges: Vec<_> = full
        .split(',')
        .map(|s| <(usize, usize)>::extract(&r, s))
        .collect();

    let mut invalid = 0;

    for (from, to) in ranges {
        for i in 2..100 {
            invalid += count_palindromes(from, to, i);
        }
    }
    println!("{}", invalid);
}

fn main() {
    part1();
    part2();
}
