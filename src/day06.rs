pub fn solve() {
    let input = std::fs::read_to_string("inputs/day06.txt").unwrap();

    println!("===== DAY 06 =====");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
enum Field {
    Obstruction,
    Guard(Direction),
    Empty,
    Visited,
}

#[derive(Clone)]
struct Guard {
    direction: Direction,
    pos: (i32, i32),
}

impl Guard {
    fn right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn parse_input(input: &str) -> (Vec<Vec<Field>>, Guard) {
    let mut guard = Guard {
        direction: Direction::Up,
        pos: (0, 0),
    };

    let map: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(i, s)| {
            s.chars()
                .enumerate()
                .map(|(j, c)| {
                    let pos = (i as i32, j as i32);
                    match c {
                        '.' => Field::Empty,
                        '#' => Field::Obstruction,
                        '^' => {
                            guard.pos = pos;
                            guard.direction = Direction::Up;
                            Field::Guard(Direction::Up)
                        }
                        '>' => {
                            guard.pos = pos;
                            guard.direction = Direction::Right;
                            Field::Guard(Direction::Right)
                        }
                        '<' => {
                            guard.pos = pos;
                            guard.direction = Direction::Left;
                            Field::Guard(Direction::Left)
                        }
                        'v' => {
                            guard.pos = pos;
                            guard.direction = Direction::Down;
                            Field::Guard(Direction::Down)
                        }
                        _ => unreachable!(),
                    }
                })
                .collect()
        })
        .collect();

    (map, guard)
}

fn part1(input: &str) -> i32 {
    let (mut map, mut guard) = parse_input(input);

    let mut acc = 1;
    loop {
        let pos = &guard.pos;
        let next = match &guard.direction {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Right => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0, pos.1 - 1),
        };

        // bruh
        if !((0..map.len() as i32).contains(&next.0) && (0..map[0].len() as i32).contains(&next.1))
        {
            break;
        }

        map[pos.0 as usize][pos.1 as usize] = Field::Visited;
        match map[next.0 as usize][next.1 as usize] {
            Field::Obstruction => guard.right(),
            Field::Empty => {
                map[next.0 as usize][next.1 as usize] = Field::Visited;
                acc += 1;
                guard.pos = next;
            }
            Field::Visited => {
                guard.pos = next;
            }
            _ => unreachable!(),
        }
    }

    acc
}

fn part2(input: &str) -> i32 {
    let (map, guard) = parse_input(input);

    let mut acc = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let mut map = map.clone();
            let mut guard = guard.clone();

            map[i][j] = match map[i][j] {
                Field::Empty => Field::Obstruction,
                _ => {
                    continue;
                }
            };

            // check loop
            loop {
                let pos = &guard.pos;
                let next = match &guard.direction {
                    Direction::Up => (pos.0 - 1, pos.1),
                    Direction::Down => (pos.0 + 1, pos.1),
                    Direction::Right => (pos.0, pos.1 + 1),
                    Direction::Left => (pos.0, pos.1 - 1),
                };

                if !((0..map.len() as i32).contains(&next.0)
                    && (0..map[0].len() as i32).contains(&next.1))
                {
                    break;
                }

                match map[next.0 as usize][next.1 as usize] {
                    Field::Obstruction => guard.right(),
                    Field::Guard(direction) if { direction == guard.direction } => {
                        acc += 1;
                        break;
                    }
                    Field::Empty | Field::Guard(_) => {
                        map[next.0 as usize][next.1 as usize] = Field::Guard(guard.direction);
                        guard.pos = next;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    acc
}

mod test {

    #[allow(unused)]
    static INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 41);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 6);
    }
}