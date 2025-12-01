use aoc_25::*;

fn part1() {
    let r = r"(.)(\d+)".try_into().unwrap();
    let m = read_input("day01.txt")
        .lines()
        .map(|l| <(char, i32)>::extract(&r, l))
        .scan(50, |state, (dir, val)| {
            match dir {
                'R' => {
                    *state += val;
                }
                'L' => {
                    *state -= val;
                }
                _ => panic!(),
            };
            *state = state.rem_euclid(100);
            Some(*state)
        })
        .collect::<Vec<_>>();
    let result = m.iter().filter(|&&v| v == 0).count();
    println!("{}", result);
}

fn part2() {
    let r = r"(.)(\d+)".try_into().unwrap();
    let m = read_input("day01.txt")
        .lines()
        .map(|l| <(char, i32)>::extract(&r, l))
        .scan(50, |state, (dir, val)| {
            let times;
            match dir {
                'R' => {
                    times = (*state + val) / 100;
                    *state += val;
                }
                'L' => {
                    times = (100 - *state + val) / 100 - (*state == 0) as i32;
                    *state -= val;
                }
                _ => panic!(),
            };
            *state = state.rem_euclid(100);
            Some((*state, times))
        })
        .collect::<Vec<_>>();
    let result: i32 = m.iter().map(|&v| v.1).sum();
    println!("{}", result);
}

fn main() {
    part1();
    part2();
}
