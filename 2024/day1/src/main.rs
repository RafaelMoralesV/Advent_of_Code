use std::{collections::HashMap, fs};

use aoc::AoC;

struct Day1 {
    left: Vec<u64>,
    right: Vec<u64>,
}

impl AoC for Day1 {
    fn parse(input: String) -> Self {
        let mut left = Vec::new();
        let mut right = Vec::new();

        input.lines().for_each(|line| {
            let pair = line
                .split_whitespace()
                .map(|n| n.parse().expect("Input is clean"))
                .collect::<Vec<_>>();

            left.push(pair[0]);
            right.push(pair[1]);
        });

        left.sort_unstable();
        right.sort_unstable();

        Self { left, right }
    }

    fn puzzle_one(&self) -> u64 {
        self.left
            .iter()
            .zip(self.right.clone())
            .map(|(l, r)| l.abs_diff(r) as u64)
            .sum()
    }

    fn puzzle_two(&self) -> u64 {
        let mut freq: HashMap<_, u64> = HashMap::new();

        for num in self.right.iter() {
            *freq.entry(num).or_insert(0) += 1;
        }

        self.left
            .iter()
            .map(|num| num * (*freq.get(num).unwrap_or(&0)))
            .sum()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file is present and intact");
    let day1 = Day1::parse(input);

    print!("\n\tpuzzle one: > {}\n", day1.puzzle_one());

    print!("\n\tpuzzle two: > {}\n", day1.puzzle_two());
}
