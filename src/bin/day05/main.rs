use std::str::FromStr;

use aoc_25::*;
use regex::Regex;

fn part1() {
    let input = read_input("day05.txt");
    let [l, r]: [&str; 2] = input.split("\n\n").collect::<Vec<_>>().try_into().unwrap();
    let r1 = Regex::new(r"(\d+)-(\d+)").unwrap();
    let ranges: Vec<_> = l
        .lines()
        .map(|l| <(usize, usize)>::extract(&r1, l))
        .collect();
    let test = |id| ranges.iter().any(|(l, h)| *l <= id && *h >= id);

    let mut fresh = 0;
    for id in r.lines().map(|l| usize::from_str(l).unwrap()) {
        if test(id) {
            fresh += 1;
        }
    }
    println!("{}", fresh);
}
fn part2() {
    let input = read_input("day05.txt");
    let [l, _]: [&str; 2] = input.split("\n\n").collect::<Vec<_>>().try_into().unwrap();
    let r1 = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut ranges: Vec<_> = l
        .lines()
        .map(|l| <(usize, usize)>::extract(&r1, l))
        .collect();

    ranges.sort_by_key(|(l, _)| *l);
    let mut i = 0;
    while i < ranges.len() - 1 {
        let (l1, r1) = ranges[i];
        let (l2, r2) = ranges[i + 1];
        if r1 >= l2 {
            ranges[i] = (usize::min(l1, l2), usize::max(r1, r2));
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }
    let result = ranges.into_iter().map(|(l, r)| r - l + 1).sum::<usize>();
    println!("{}", result);
}
fn main() {
    part1();
    part2();
}
