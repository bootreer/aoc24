use std::collections::HashMap;

pub fn solve() {
    let input = std::fs::read_to_string("inputs/day11.txt").unwrap();

    println!("===== DAY 11 =====");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

// could be tail-recursive?
fn blink_single(stone: u64, n: u64, acc: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if let Some(&len) = memo.get(&(stone, n)) {
        return len;
    }

    if n == 0 {
        return acc;
    }

    let res = match stone {
        0 => blink_single(1, n - 1, acc, memo),
        i if i.ilog10() % 2 == 1 => {
            let t = 10u64.pow(i.ilog10() / 2 + 1);
            blink_single(i / t, n - 1, acc, memo) + blink_single(i % t, n - 1, acc, memo)
        }
        _ => blink_single(stone * 2024, n - 1, acc, memo),
    };

    memo.insert((stone, n), res);
    res
}

fn blink_mem(stones: Vec<u64>, n: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    stones
        .iter()
        .map(|&stone| blink_single(stone, n, 1, memo))
        .sum()
}

fn part1(input: &str) -> u64 {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();

    let mut memo = HashMap::new();
    blink_mem(stones, 25, &mut memo)
}

fn part2(input: &str) -> u64 {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();

    let mut memo = HashMap::new();
    blink_mem(stones, 75, &mut memo)
}

mod test {

    #[allow(unused)]
    static INPUT: &str = "125 17";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 55312);
    }
}
