use aoc::AoC;
use std::fs;

#[derive(Debug)]
struct Day2 {
    reports: Vec<Vec<u64>>,
}

#[derive(Debug, PartialEq, Eq)]
enum ReportStatus {
    Safe,
    Unsafe,
}

impl AoC for Day2 {
    fn parse(input: String) -> Self {
        Self {
            reports: input
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| num.parse().expect("Input is unaltered"))
                        .collect()
                })
                .collect(),
        }
    }

    fn puzzle_one(&self) -> u64 {
        self.reports
            .iter()
            .map(check_safety)
            .filter(|status| *status == ReportStatus::Safe)
            .count() as u64
    }

    fn puzzle_two(&self) -> u64 {
        self.reports
            .iter()
            .map(laxer_safety)
            .filter(|status| *status == ReportStatus::Safe)
            .count() as u64
    }
}

fn check_safety(report: &Vec<u64>) -> ReportStatus {
    let mut report_pair = report.iter().peekable();

    let mut grow = 0;

    while let Some(&item) = report_pair.next() {
        if let Some(&next) = report_pair.peek() {
            let diff: i64 = item as i64 - *next as i64;

            if diff.abs() > 3 || diff.abs() < 1 {
                return ReportStatus::Unsafe;
            }

            if grow == 0 {
                grow += diff;
            } else {
                match grow {
                    n if n > 0 && diff < 0 => return ReportStatus::Unsafe,
                    n if n < 0 && diff > 0 => return ReportStatus::Unsafe,
                    _ => continue,
                }
            }
        }
    }

    ReportStatus::Safe
}

fn laxer_safety(report: &Vec<u64>) -> ReportStatus {
    todo!()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file is present and intact");
    let day2 = Day2::parse(input);

    print!("\n\tpuzzle one: > {}\n", day2.puzzle_one());

    print!("\n\tpuzzle two: > {}\n", day2.puzzle_two());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = String::from(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );

        let day2 = Day2::parse(input);

        assert_eq!(2, day2.puzzle_one());
    }

    #[test]
    fn part_two_example() {
        let input = String::from(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );

        let day2 = Day2::parse(input);

        assert_eq!(4, day2.puzzle_two());
    }
}
