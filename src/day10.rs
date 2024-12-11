use std::collections::HashSet;

pub fn solve() {
    let input = std::fs::read_to_string("inputs/day10.txt").unwrap();

    println!("===== DAY 10 =====");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn compute_stats((row, col): (i32, i32), map: &[Vec<u32>]) -> (usize, usize) {
    let rows = map.len();
    let cols = map[0].len();

    let mut stack = vec![(row, col)];
    let mut peaks = HashSet::new();
    let mut rating = 0;

    while let Some((row, col)) = stack.pop() {
        let height = map[row as usize][col as usize];
        let target = height + 1;

        if height == 9 {
            peaks.insert((row, col));
            rating += 1;
            continue;
        }

        for (o_r, o_c) in DIRECTIONS {
            let (n_r, n_c) = (o_r + row, o_c + col);

            if !(0..rows as i32).contains(&n_r) || !(0..cols as i32).contains(&n_c) {
                continue;
            }
            if map[n_r as usize][n_c as usize] == target {
                stack.push((n_r, n_c));
            }
        }
    }

    (peaks.len(), rating)
}

fn parse_input(input: &str) -> (Vec<Vec<u32>>, Vec<(i32, i32)>) {
    let mut trailheads: Vec<(i32, i32)> = vec![];

    let map: Vec<Vec<u32>> = input
        .lines()
        .enumerate()
        .map(|(row, s)| {
            s.chars()
                .enumerate()
                .map(|(col, c)| {
                    let h = c.to_digit(10).unwrap();
                    if h == 0 {
                        trailheads.push((row as i32, col as i32));
                    }
                    h
                })
                .collect()
        })
        .collect();

    (map, trailheads)
}

fn part1(input: &str) -> usize {
    let (map, trailheads) = parse_input(input);
    trailheads.iter().map(|&head| compute_stats(head, &map).0).sum()
}

fn part2(input: &str) -> usize {
    let (map, trailheads) = parse_input(input);
    trailheads.iter().map(|&head| compute_stats(head, &map).1).sum()
}

mod test {

    #[allow(unused)]
    static INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 36);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 81);
    }
}
