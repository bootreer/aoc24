pub fn solve() {
    let input = std::fs::read_to_string("inputs/day02.txt").unwrap();

    println!("===== DAY 02 =====");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn to_iter(input: &str) -> impl Iterator<Item = Vec<i32>> + use<'_> {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    })
}

fn safe(level: &[i32]) -> bool {
    let a = level.is_sorted_by(|a, b| (1..=3).contains(&(a - b)));
    let b = level
        .iter()
        .rev()
        .is_sorted_by(|&a, &b| (1..=3).contains(&(a - b)));

    a || b
}

fn part1(input: &str) -> String {
    let res = to_iter(input).filter(|l| safe(l)).count();

    format!("{res}")
}

fn part2(input: &str) -> String {
    let res = to_iter(input)
        .filter(|l| {
            let mut b = false;
            for i in 0..l.len() {
                let mut l = l.clone();
                l.remove(i);
                b = b || safe(&l);
            }
            b
        })
        .count();
    format!("{res}")
}

#[allow(dead_code)]
mod test {

    static INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), String::from("2"));
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), String::from("2"));
    }
}
