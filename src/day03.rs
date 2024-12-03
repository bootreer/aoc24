use regex::Regex;

pub fn solve() {
    let input = std::fs::read_to_string("inputs/day03.txt").unwrap();

    println!("===== DAY 03 =====");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    regex
        .captures_iter(input)
        .map(|cap| {
            (
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
            )
        })
        .map(|(l, r)| l * r)
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut split = input.split("don't()");
    let init = part1(split.next().unwrap_or_default());

    split
        .map(|s| s.split("do()").skip(1).map(part1).sum::<u32>())
        .sum::<u32>()
        + init
}

mod test {

    #[allow(unused)]
    static INPUT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 161);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 48);
    }
}
