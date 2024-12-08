use std::collections::{HashMap, HashSet};

pub fn solve() {
    let input = std::fs::read_to_string("inputs/day08.txt").unwrap();

    println!("===== DAY 08 =====");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

type Map = (HashMap<char, Vec<(i32, i32)>>, i32, i32);

fn parse_input(input: &str) -> Map {
    let height = input.lines().count() as i32;
    let width = input.split_once('\n').unwrap().0.len() as i32;

    let all_antennas: HashMap<_, Vec<_>> =
        input
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (y, str)| {
                str.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .for_each(|(x, c)| {
                        acc.entry(c).or_default().push((x as i32, y as i32));
                    });
                acc
            });
    (all_antennas, height, width)
}

fn part1(input: &str) -> usize {
    let (all_antennas, height, width) = parse_input(input);

    let mut antinodes: HashSet<_> = HashSet::new();
    for antennas in all_antennas.values() {
        for &(x1, y1) in antennas {
            for &(x2, y2) in antennas {
                if x1 == x2 && y1 == y2 {
                    continue;
                }

                let x = x1 + (x1 - x2);
                let y = y1 + (y1 - y2);

                if (0..width).contains(&x) && (0..height).contains(&y) {
                    antinodes.insert((x, y));
                }
            }
        }
    }

    antinodes.len()
}

fn part2(input: &str) -> usize {
    let (all_antennas, height, width) = parse_input(input);

    let mut antinodes: HashSet<_> = HashSet::new();
    for antennas in all_antennas.values() {
        for &(x1, y1) in antennas {
            for &(x2, y2) in antennas {
                if x1 == x2 && y1 == y2 {
                    continue;
                }
                let x_diff = x1 - x2;
                let y_diff = y1 - y2;

                let mut x = x1;
                let mut y = y1;

                loop {
                    antinodes.insert((x, y));

                    x += x_diff;
                    y += y_diff;
                    if !(0..width).contains(&x) || !(0..height).contains(&y) {
                        break;
                    }
                }
            }
        }
    }

    antinodes.len()
}

mod test {

    #[allow(unused)]
    static INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 14);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 34);
    }
}
