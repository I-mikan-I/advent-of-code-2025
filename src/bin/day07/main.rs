use std::collections::{HashMap, HashSet};

use aoc_25::read_input;

fn splitter_rec(
    (r, c): (usize, usize),
    splitters: &mut HashSet<(usize, usize)>,
    lookup_table: &[Vec<(usize, usize)>],
) {
    let next @ (r_, c_) = lookup_table[r][c];
    if next == (0, 0) {
        return;
    }
    if splitters.remove(&next) {
        if c_ > 0 {
            splitter_rec((r_, c_ - 1), splitters, lookup_table);
        }
        if c_ < lookup_table[r_].len() - 1 {
            splitter_rec((r_, c_ + 1), splitters, lookup_table);
        }
    }
}

fn count_rec(
    (r, c): (usize, usize),
    splitters: &HashSet<(usize, usize)>,
    lookup_table: &[Vec<(usize, usize)>],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let next @ (r_, c_) = lookup_table[r][c];
    if let Some(&v) = cache.get(&next) {
        v
    } else {
        let result = if splitters.contains(&next) {
            let mut result = 0;
            if c_ > 0 {
                result += count_rec((r_, c_ - 1), splitters, lookup_table, cache);
            }
            if c_ < lookup_table[r_].len() - 1 {
                result += count_rec((r_, c_ + 1), splitters, lookup_table, cache);
            }
            result
        } else {
            1
        };
        cache.insert(next, result);
        result
    }
}

#[allow(clippy::type_complexity)]
fn parse_input() -> (
    (usize, usize),
    HashSet<(usize, usize)>,
    Vec<Vec<(usize, usize)>>,
) {
    let input: Vec<Vec<_>> = read_input("day07.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut lookup_table = vec![vec![(0, 0); input[0].len()]; input.len()];
    let mut splitters = HashSet::new();
    let mut start_id = (0, 0);
    for r in 0..input.len() {
        for (c, &sym) in input[r].iter().enumerate() {
            let splitter_id = (r, c);
            if sym == 'S' {
                start_id = splitter_id;
            }
            if sym == '^' {
                let mut t = r as isize - 1;
                while t >= 0
                    && let '.' | 'S' = input[t as usize][c]
                {
                    lookup_table[t as usize][c] = splitter_id;
                    t -= 1;
                }
                splitters.insert(splitter_id);
            }
        }
    }
    (start_id, splitters, lookup_table)
}

fn part1() {
    let (start_id, mut splitters, lookup_table) = parse_input();
    let begin = splitters.len();
    splitter_rec(start_id, &mut splitters, &lookup_table);
    let end = splitters.len();
    println!("{}", begin - end);
}
fn part2() {
    let (start_id, splitters, lookup_table) = parse_input();
    let result = count_rec(start_id, &splitters, &lookup_table, &mut HashMap::new());
    println!("{}", result);
}
fn main() {
    part1();
    part2();
}
