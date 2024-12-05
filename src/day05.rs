use crate::utils::parse_to_vec;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn solve() {
    let input = std::fs::read_to_string("inputs/day05.txt").unwrap();

    println!("===== DAY 05 =====");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn cmp(l: &u32, r: &u32, orderings: &HashMap<u32, HashSet<u32>>) -> Ordering {
    if orderings
        .get(l)
        .map(|post| post.get(r).is_some())
        .unwrap_or_default()
    {
        return Ordering::Less;
    }

    if orderings
        .get(r)
        .map(|post| post.get(l).is_some())
        .unwrap_or_default()
    {
        return Ordering::Greater;
    }

    Ordering::Equal
}

fn check_valid(update: &[u32], order: &HashMap<u32, HashSet<u32>>) -> bool {
    update.is_sorted_by(|l, r| cmp(l, r, order) != Ordering::Greater)
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let parts: Vec<_> = input.split("\n\n").collect();

    let order: HashMap<u32, HashSet<u32>> = parts[0]
        .lines()
        .map(|s| parse_to_vec::<u32>(s, "|").unwrap())
        .map(|v| (v[0], v[1]))
        .fold(HashMap::new(), |mut map, (l, r)| {
            map.entry(l).or_default().insert(r);
            map
        });

    let (valid, invalid): (Vec<_>, Vec<_>) = parts[1]
        .lines()
        .map(|s| parse_to_vec::<u32>(s, ",").unwrap())
        .partition(|u| check_valid(u, &order));

    (order, valid, invalid)
}

fn part1(input: &str) -> u32 {
    let (_, valid, _) = parse_input(input);
    valid.iter().map(|u| u[u.len() / 2]).sum()
}

fn part2(input: &str) -> u32 {
    let (map, _, mut invalid) = parse_input(input);

    #[allow(clippy::manual_inspect)]
    invalid
        .iter_mut()
        .map(|u| {
            u.sort_by(|l, r| cmp(l, r, &map));
            u
        })
        .map(|u| u[u.len() / 2])
        .sum()
}

mod test {

    #[allow(unused)]
    static INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 143);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 123);
    }
}
