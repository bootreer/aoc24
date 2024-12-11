use std::collections::VecDeque;

pub fn solve() {
    let input = std::fs::read_to_string("inputs/day09.txt").unwrap();

    println!("===== DAY 09 =====");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let mut next = 0;

    let mut free: VecDeque<u64> = VecDeque::new();

    let mut blocks: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .enumerate()
        .flat_map(|(i, len)| {
            let iter = next..next + len;
            next += len;
            if i % 2 == 0 {
                iter.map(|idx| (idx, i / 2)).collect()
            } else {
                iter.for_each(|i| free.push_back(i));
                vec![]
            }
        })
        .collect();

    for i in (0..blocks.len()).rev() {
        if blocks[i].0 < *free.front().unwrap() {
            break;
        }
        blocks[i].0 = free.pop_front().unwrap();
    }

    blocks.iter().map(|&(idx, id)| idx * id as u64).sum()
}

fn part2(input: &str) -> u64 {
    let mut idx = 0;
    let mut free_slots = vec![];

    let blocks: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .enumerate()
        .filter_map(|(i, len)| {
            idx += len;
            if i % 2 == 0 {
                Some((len, i as u64 / 2, idx - len))
            } else {
                free_slots.push((len, idx - len));
                None
            }
        })
        .collect();

    blocks
        .iter()
        .rev()
        .map(|&(len, id, mut idx)| {
            let free_idx = free_slots
                .iter()
                .position(|&(len_b, offset)| len_b >= len && offset < idx);

            if let Some(free_idx) = free_idx {
                idx = free_slots[free_idx].1;
                free_slots[free_idx].0 -= len;
                free_slots[free_idx].1 += len;
            }
            (idx..idx + len).map(|i| i * id).sum::<u64>()
        })
        .sum()
}

mod test {

    #[allow(unused)]
    static INPUT: &str = "2333133121414131402";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 1928);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 2858);
    }
}
