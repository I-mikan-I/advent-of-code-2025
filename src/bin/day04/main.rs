use aoc_25::read_input;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Type {
    Paper,
    Empty,
}

fn part1() {
    let input = read_input("day04.txt");
    let map: Vec<_> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Type::Empty,
                    '@' => Type::Paper,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut neighbors: Vec<Vec<u32>> = vec![vec![0; map[0].len()]; map.len()];

    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == Type::Paper {
                for i in -1..=1 {
                    for j in -1..=1 {
                        let r_ = r as i32 - i;
                        let c_ = c as i32 - j;
                        if r_ >= 0
                            && r_ < map.len() as i32
                            && c_ >= 0
                            && c_ < map[r_ as usize].len() as i32
                        {
                            neighbors[r_ as usize][c_ as usize] += 1;
                        }
                    }
                }
                neighbors[r][c] -= 1;
            }
        }
    }
    let count = map
        .iter()
        .flatten()
        .zip(neighbors.iter().flatten())
        .filter(|(t, _)| **t == Type::Paper)
        .filter(|(_, c)| **c < 4)
        .count();

    println!("{}", count);
}

fn remove_paper(map: &mut [Vec<Type>]) -> usize {
    let mut neighbors: Vec<Vec<u32>> = vec![vec![0; map[0].len()]; map.len()];

    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == Type::Paper {
                for i in -1..=1 {
                    for j in -1..=1 {
                        let r_ = r as i32 - i;
                        let c_ = c as i32 - j;
                        if r_ >= 0
                            && r_ < map.len() as i32
                            && c_ >= 0
                            && c_ < map[r_ as usize].len() as i32
                        {
                            neighbors[r_ as usize][c_ as usize] += 1;
                        }
                    }
                }
                neighbors[r][c] -= 1;
            }
        }
    }
    let mut removed = 0;
    for r in 0..neighbors.len() {
        for c in 0..neighbors[r].len() {
            if map[r][c] == Type::Paper && neighbors[r][c] < 4 {
                map[r][c] = Type::Empty;
                removed += 1;
            }
        }
    }
    removed
}

fn part2() {
    let input = read_input("day04.txt");
    let mut map: Vec<_> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Type::Empty,
                    '@' => Type::Paper,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut removed = remove_paper(&mut map);
    while let i = remove_paper(&mut map)
        && i > 0
    {
        removed += i;
    }
    println!("{}", removed);
}

fn main() {
    part1();
    part2();
}
