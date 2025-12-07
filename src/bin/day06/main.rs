use std::{
    ops::{Add, Mul},
    str::FromStr,
};

use aoc_25::read_input;

fn part1() {
    let input: Vec<_> = read_input("day06.txt")
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| String::from_str(s).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let operations = input
        .last()
        .unwrap()
        .iter()
        .map(|c| match c.as_str() {
            "*" => (1usize, <usize as Mul>::mul as fn(usize, usize) -> usize),
            "+" => (0usize, <usize as Add>::add as fn(usize, usize) -> usize),
            _ => panic!(),
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for i in 0..input[0].len() {
        let (mut result, op) = operations[i];
        for row in input.iter().take(input.len() - 1) {
            result = op(result, usize::from_str(&row[i]).unwrap());
        }
        sum += result;
    }
    println!("{}", sum);
}
fn part2() {
    let mut input: Vec<_> = read_input("day06.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();
    let ops = input.remove(input.len() - 1);

    let mut i = 0;
    let mut result = 0;
    while i < ops.len() {
        let op = ops[i];
        assert!(op == '+' || op == '*');
        let mut nums = vec![];
        while i < ops.len() && (i + 1 == ops.len() || ops[i + 1] == ' ') {
            nums.push(0);
            let tmp = nums.last_mut().unwrap();
            for row in input.iter() {
                match row[i] {
                    '1'..='9' => {
                        *tmp *= 10;
                        *tmp += row[i] as i32 - '0' as i32;
                    }
                    ' ' | '0' => {}
                    _ => panic!(),
                }
            }
            i += 1;
        }
        result += nums
            .iter()
            .fold(if op == '+' { 0usize } else { 1 }, |acc, &v| {
                if op == '+' {
                    acc + v as usize
                } else {
                    acc * v as usize
                }
            });
        i += 1;
    }
    println!("{}", result);
}
fn main() {
    part1();
    part2();
}
