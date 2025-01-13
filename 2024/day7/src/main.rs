use aoc::AoC;
use std::collections::VecDeque;

#[derive(Debug)]
struct Entry {
    target: u64,
    data: VecDeque<u64>,
}

#[derive(Debug)]
struct Day7 {
    data: Vec<Entry>,
}

#[derive(Debug)]
enum Ops {
    Sum,
    Mul,
}

fn tree_two(target: u64, acum: u64, data: &mut VecDeque<u64>) -> Option<u64> {
    if acum == target {
        return Some(acum);
    }

    if let Some(value) = data.pop_front() {
        tree_two(target, acum + value, &mut data.clone())
            .or_else(|| tree_two(target, acum * value, &mut data.clone()))
    } else {
        None
    }
}

fn process_entry(entry: &mut Entry) -> Option<u64> {
    tree_two(
        entry.target,
        entry.data.pop_front().unwrap(),
        &mut entry.data,
    )
}

impl AoC for Day7 {
    fn parse(input: String) -> Self {
        let data = input
            .lines()
            .map(|line| {
                let (target, data) = line.split_once(":").unwrap();
                let target = target.parse().unwrap();

                let data = data
                    .split_whitespace()
                    .map(str::parse)
                    .filter_map(Result::ok)
                    .collect();

                Entry { target, data }
            })
            .collect();

        Self { data }
    }

    fn puzzle_one(&mut self) -> u64 {
        self.data
            .iter_mut()
            .map(process_entry)
            .filter_map(|val| val)
            .sum()
    }

    fn puzzle_two(&mut self) -> u64 {
        todo!()
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Input file is present and intact");
    let mut day7 = Day7::parse(input);

    print!("\n\tpuzzle one: > {}\n", day7.puzzle_one());

    print!("\n\tpuzzle two: > {}\n", day7.puzzle_two());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tree_success() {
        let mut entry = Entry {
            target: 190,
            data: VecDeque::from([10, 19]),
        };

        assert_eq!(Some(190), process_entry(&mut entry));
    }

    #[test]
    fn tree_fail_one() {
        let mut entry = Entry {
            target: 83,
            data: VecDeque::from([17, 5]),
        };

        assert_eq!(None, process_entry(&mut entry));
    }

    #[test]
    fn tree_fail_two() {
        let mut entry = Entry {
            target: 156,
            data: VecDeque::from([15, 6]),
        };

        assert_eq!(None, process_entry(&mut entry));
    }

    #[test]
    fn puzzle_one_example() {
        let input =
            std::fs::read_to_string("example_one.txt").expect("Input file is present and intact");

        let mut day7 = Day7::parse(input);

        assert_eq!(3749, day7.puzzle_one());
    }

    #[test]
    fn puzzle_two_example() {
        let input =
            std::fs::read_to_string("example_one.txt").expect("Input file is present and intact");

        let mut day7 = Day7::parse(input);

        assert_eq!(123, day7.puzzle_two());
    }
}
