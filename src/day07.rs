pub fn solve() {
    let input = std::fs::read_to_string("inputs/day07.txt").unwrap();

    println!("===== DAY 07 =====");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn check(res: i64, vals: &[i64]) -> bool {
    if res < 0 {
        return false;
    }
    match *vals {
        [] => res == 0,
        [..] => {
            let (last, rest) = vals.split_last().unwrap();
            (res % last == 0 && check(res / last, rest)) || check(res - last, rest)
        }
    }
}

fn check_concat(res: i64, vals: &[i64]) -> bool {
    if res < 0 {
        return false;
    }
    match *vals {
        [] => res == 0,
        [..] => {
            let (last, rest) = vals.split_last().unwrap();
            let t = 10i64.pow(last.ilog10() + 1);

            let div = res % last == 0 && check_concat(res / last, rest);
            let concat = (*last == res % t) && check_concat(res / t, rest);

            div || concat || check_concat(res - last, rest)
        }
    }
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> impl Iterator<Item = (i64, Vec<i64>)> + '_ {
    input.lines().map(|l| {
        let l: Vec<_> = l.split(":").collect();
        (
            l[0].parse::<i64>().unwrap(),
            l[1].split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>(),
        )
    })
}

fn part1(input: &str) -> i64 {
    parse_input(input)
        .filter(|(res, vals)| check(*res, vals))
        .map(|v| v.0)
        .sum()
}

fn part2(input: &str) -> i64 {
    parse_input(input)
        .filter(|(res, vals)| check_concat(*res, vals))
        .map(|v| v.0)
        .sum()
}

mod test {

    #[allow(unused)]
    static INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 3749);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 11387);
    }
}
