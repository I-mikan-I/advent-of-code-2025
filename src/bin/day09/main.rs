use aoc_25::{Extract, read_input};
use regex::Regex;

fn part1() {
    let input = read_input("day09.txt");
    let r = Regex::new(r"(\d+),(\d+)").unwrap();
    let cords: Vec<_> = input
        .lines()
        .map(|l| <(usize, usize)>::extract(&r, l))
        .collect();

    // naive implementation
    let mut max = 0;
    for (i, c1) in cords.iter().enumerate() {
        for c2 in cords.iter().skip(i + 1) {
            let x = isize::abs(c1.0 as isize - c2.0 as isize);
            let y = isize::abs(c1.1 as isize - c2.1 as isize);
            let area = (x + 1) * (y + 1);
            if area > max {
                max = area;
            }
        }
    }
    println!("{}", max);
}
fn is_inside((x, y): (usize, usize), graph: &[(usize, usize)]) -> bool {
    let mut last = graph[0];
    let mut lines = vec![];
    for next @ (x2, y2) in graph.iter().skip(1).chain(graph.iter().take(1)) {
        let minx = last.0.min(*x2);
        let maxx = last.0.max(*x2);
        let miny = last.1.min(*y2);
        let maxy = last.1.max(*y2);
        if *x2 == x && miny <= y && maxy >= y || *y2 == y && minx <= x && maxx >= x {
            return true;
        } else if *y2 == last.1 &&
            // line left of x, y
             minx <= x && maxx > x && *y2 < y
        {
            lines.push(*next);
        }
        last = *next;
    }
    lines.len() % 2 == 1
}
fn intersects((from, to): ((usize, usize), (usize, usize)), graph: &[(usize, usize)]) -> bool {
    let mut last = graph[0];
    for &next @ (x2, y2) in graph.iter().skip(1).chain(graph.iter().take(1)) {
        if from.1 == to.1
            && x2 == last.0
            && from.0.min(to.0) < x2
            && from.0.max(to.0) > x2
            && y2.min(last.1) < from.1
            && y2.max(last.1) > from.1
            || from.0 == to.0
                && y2 == last.1
                && from.1.min(to.1) < y2
                && from.1.max(to.1) > y2
                && x2.min(last.0) < from.0
                && x2.max(last.0) > from.0
        {
            return true;
        }
        last = next;
    }
    false
}
fn part2() {
    let input = read_input("day09.txt");
    let r = Regex::new(r"(\d+),(\d+)").unwrap();
    let cords: Vec<_> = input
        .lines()
        .map(|l| <(usize, usize)>::extract(&r, l))
        .collect();
    let mut max = 0;
    for (i, c1) in cords.iter().enumerate() {
        let tentative: Vec<_> = cords
            .iter()
            .skip(i + 1)
            .cloned()
            .filter(|&(x, y)| {
                [(x, y), (c1.0, y), (x, c1.1)]
                    .iter()
                    .all(|&p| is_inside(p, &cords))
                    && [
                        ((x, y), (x, c1.1)),
                        ((x, c1.1), (c1.0, c1.1)),
                        ((c1.0, c1.1), (c1.0, y)),
                        ((c1.0, y), (x, y)),
                    ]
                    .iter()
                    .all(|&p| !intersects(p, &cords))
            })
            .collect();
        max = max.max(
            tentative
                .iter()
                .map(|&(x, y)| {
                    (isize::abs(x as isize - c1.0 as isize) + 1)
                        * (isize::abs(y as isize - c1.1 as isize) + 1)
                })
                .max()
                .unwrap_or(0),
        );
    }
    println!("{}", max);
}
fn main() {
    part1();
    part2();
}
