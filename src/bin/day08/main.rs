mod union_find;
use aoc_25::*;
use regex::Regex;
use union_find::*;

fn distance_euclid(
    (x1, y1, z1): (isize, isize, isize),
    (x2, y2, z2): (isize, isize, isize),
) -> f64 {
    // proj onto xy plane
    let x_dist = isize::abs(x1 - x2);
    let y_dist = isize::abs(y1 - y2);
    let z_dist = isize::abs(z1 - z2);
    let xy_dist = f64::sqrt((x_dist.pow(2) + y_dist.pow(2)) as f64);
    f64::sqrt(xy_dist.powf(2.0) + z_dist.pow(2) as f64)
}
type Coord = (isize, isize, isize);
fn parse_input() -> (Vec<Coord>, Vec<(Coord, Coord)>, DisjointSet<Coord>) {
    let input = read_input("day08.txt");
    let r = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
    let coords: Vec<_> = input
        .lines()
        .map(|l| <(isize, isize, isize)>::extract(&r, l))
        .collect();
    let mut pairs: Vec<_> = coords
        .iter()
        .cloned()
        .enumerate()
        .flat_map(|(i, c)| {
            coords
                .iter()
                .skip(i + 1)
                .cloned()
                .filter(move |c2| *c2 != c)
                .map(move |c2| (c, c2))
        })
        .collect();
    pairs.sort_by(|(l1, l2), (r1, r2)| {
        distance_euclid(*r1, *r2)
            .partial_cmp(&distance_euclid(*l1, *l2))
            .unwrap()
    });

    let mut ds = DisjointSet::new();
    coords.iter().for_each(|c| {
        ds.singleton(*c).unwrap();
    });
    (coords, pairs, ds)
}

fn part1() {
    let (coords, mut pairs, mut ds) = parse_input();
    for _ in 0..1000 {
        let (ref l, ref r) = pairs.pop().unwrap();
        ds.union(l, r);
    }
    // count
    let mut counts = Vec::new();
    for c in coords {
        let id = ds.find(&c).unwrap();
        counts.resize(usize::max(counts.len(), id + 1), 0);
        counts[id] += 1;
    }
    counts.sort_by_key(|e| std::cmp::Reverse(*e));
    let result: usize = counts.iter().take(3).product();

    println!("{:?}", result);
}
fn part2() {
    let (coords, mut pairs, mut ds) = parse_input();
    let fst = coords[0];
    let mut last = (fst, fst);
    while let Some(i) = ds.find(&fst)
        && ds.size(i).unwrap() < coords.len()
    {
        let (ref l, ref r) = pairs.pop().unwrap();
        ds.union(l, r);
        last = (*l, *r);
    }
    let result = last.0.0 * last.1.0;
    println!("{}", result);
}
fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use crate::union_find::DisjointSet;

    #[test]
    fn union_find_test() {
        let mut ds = DisjointSet::new();
        for i in 0..10 {
            ds.singleton(i);
        }
        for i in 1..5 {
            ds.union(&(i - 1), &i);
        }
        let id = ds.find(&0).unwrap();
        for i in 0..5 {
            assert!(ds.find(&i).unwrap() == id);
        }
    }
}
