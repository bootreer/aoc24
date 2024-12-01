pub fn solve() {
    let input = std::fs::read_to_string("inputs/day01.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let (mut v1, mut v2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let s = line.split_whitespace().collect::<Vec<&str>>();
            (s[0].parse::<i32>().unwrap(), s[1].parse::<i32>().unwrap())
        })
        .unzip();

    v1.sort();
    v2.sort();

    let res = v1
        .iter()
        .zip(v2)
        .fold(0, |acc, (&l, r)| acc + (l - r).abs());

    println!("Part01: {res}");
}

fn part2(input: &str) {
    use std::collections::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::new();

    let v: Vec<i32> = input
        .lines()
        .map(|line| {
            let s = line.split_whitespace().collect::<Vec<&str>>();
            let occur = map.entry(s[1].parse::<i32>().unwrap()).or_insert(0);
            *occur += 1;
            s[0].parse::<i32>().unwrap()
        })
        .collect();

    let res = v
        .iter()
        .fold(0, |acc, &v| acc + v * (*map.get(&v).unwrap_or(&0)));

    println!("Part02: {res}");
}
