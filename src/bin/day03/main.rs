use aoc_25::*;
use std::str::FromStr;

fn joltage_rec(iter: isize, bank: &[usize]) -> usize {
    if iter < 0 {
        return 0;
    }
    let (fstidx, val) = bank
        .iter()
        .enumerate()
        .take(bank.len() - iter as usize)
        .rev()
        .max_by_key(|(_, v)| **v)
        .unwrap();
    let rec_result = joltage_rec(iter - 1, &bank[fstidx + 1..bank.len()]);

    rec_result + val * 10usize.pow(iter as u32)
}

fn part1() {
    let input = read_input("day03.txt");
    let banks: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|i| usize::from_str(&i.to_string()).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let mut result = 0;
    for bank in banks {
        result += joltage_rec(1, &bank);
    }
    println!("{}", result);
}
fn part2() {
    let input = read_input("day03.txt");
    let banks: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|i| usize::from_str(&i.to_string()).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let mut result = 0;
    for bank in banks {
        result += joltage_rec(11, &bank);
    }
    println!("{}", result);
}

fn main() {
    part1();
    part2();
}
