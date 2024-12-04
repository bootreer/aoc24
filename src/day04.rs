use regex::Regex;

pub fn solve() {
    let input = std::fs::read_to_string("inputs/day04.txt").unwrap();

    println!("===== DAY 04 =====");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn find_xmas(lines: &[String]) -> usize {
    let xmas = Regex::new("XMAS").unwrap();
    let samx = Regex::new("SAMX").unwrap();

    lines
        .iter()
        .map(|l| xmas.find_iter(l).count() + samx.find_iter(l).count())
        .sum()
}

fn part1(input: &str) -> usize {
    let chars: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let horizontal = input.lines().map(String::from).collect::<Vec<_>>();
    let mut transposed = vec![String::new(); horizontal[0].len()];
    input.lines().for_each(|l| {
        l.chars()
            .enumerate()
            .for_each(|(i, ch)| transposed[i].push(ch))
    });

    let hor = find_xmas(&horizontal);
    let vert = find_xmas(&transposed);

    let mut diag = 0;
    for i in 0..=chars.len() - 4 {
        for j in 0..=chars[i].len() - 4 {
            let top = [
                chars[i][j],
                chars[i + 1][j + 1],
                chars[i + 2][j + 2],
                chars[i + 3][j + 3],
            ];

            let bot = [
                chars[i + 3][j],
                chars[i + 2][j + 1],
                chars[i + 1][j + 2],
                chars[i][j + 3],
            ];

            if matches!(top, ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X']) {
                diag += 1;
            }

            if matches!(bot, ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X']) {
                diag += 1;
            }
        }
    }

    diag + vert + hor
}

fn part2(input: &str) -> u32 {
    let chars: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut acc = 0;
    for i in 0..=chars.len() - 3 {
        for j in 0..=chars[i].len() - 3 {
            let top = [chars[i][j], chars[i + 1][j + 1], chars[i + 2][j + 2]];
            let bot = [chars[i + 2][j], chars[i + 1][j + 1], chars[i][j + 2]];

            if matches!(top, ['M', 'A', 'S'] | ['S', 'A', 'M'])
                && matches!(bot, ['M', 'A', 'S'] | ['S', 'A', 'M'])
            {
                acc += 1;
            }
        }
    }
    acc
}

mod test {

    #[allow(unused)]
    static INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 18);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 9);
    }
}
