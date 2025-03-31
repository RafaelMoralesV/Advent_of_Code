use aoc::AoC;
use std::fs;

struct Day10 {}

impl AoC for Day10 {
    fn parse(_input: String) -> Self {
        todo!()
    }

    fn puzzle_one(&mut self) -> u64 {
        todo!()
    }

    fn puzzle_two(&mut self) -> u64 {
        todo!()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file is present and intact");
    let mut day10 = Day10::parse(input);

    print!("\n\tpuzzle one: > {}\n", day10.puzzle_one());

    print!("\n\tpuzzle two: > {}\n", day10.puzzle_two());
}
