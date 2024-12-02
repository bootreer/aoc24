use std::collections::HashMap;

pub fn solve() {
    let input = std::fs::read_to_string("inputs/day01.txt").unwrap();

    println!("===== DAY 01 =====");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let (mut v1, mut v2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let s = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (s[0], s[1])
        })
        .unzip();

    v1.sort();
    v2.sort();

    v1.iter().zip(v2).map(|(&l, r)| (l - r).abs()).sum()
}

fn part2(input: &str) -> i32 {
    let mut occurences = HashMap::new();

    let v: Vec<i32> = input
        .lines()
        .map(|line| {
            let s = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            *occurences.entry(s[1]).or_insert(0) += 1;
            s[0]
        })
        .collect();

    v.iter().map(|v| *occurences.get(v).unwrap_or(&0)).sum()
}
