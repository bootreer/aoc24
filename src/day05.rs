use crate::utils::parse_to_vec;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn solve() {
    let input = std::fs::read_to_string("inputs/day05.txt").unwrap();

    println!("===== DAY 01 =====");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn check_valid(update: &[u32], order: &HashMap<u32, HashSet<u32>>) -> bool {
    let update_idx: HashMap<u32, Vec<usize>> =
        update
            .iter()
            .copied()
            .enumerate()
            .fold(HashMap::new(), |mut map, (i, v)| {
                map.entry(v).or_default().push(i);
                map
            });

    // bruteforcey
    for (i, v) in update.iter().enumerate() {
        let afters = order.get(v);
        let valid = afters
            .map(|set| {
                set.iter()
                    .map(|a| update_idx.get(a))
                    .map(|v| v.map(|v| v.iter().all(|&idx| i < idx)).unwrap_or(true))
                    .all(|b| b)
            })
            .unwrap_or(true);

        if !valid {
            return false;
        }
    }

    true
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
            u.sort_by(|l, r| {
                if map
                    .get(l)
                    .map(|post| post.get(r).is_some())
                    .unwrap_or_default()
                {
                    return Ordering::Less;
                }

                if map
                    .get(r)
                    .map(|post| post.get(l).is_some())
                    .unwrap_or_default()
                {
                    return Ordering::Greater;
                }

                Ordering::Equal
            });
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
