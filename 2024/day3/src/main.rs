use aoc::AoC;
use std::fs;

#[derive(Debug)]
struct Day3 {
    instructions: Vec<InstructionType>,
}

#[derive(Debug)]
enum InstructionType {
    Do,
    Dont,
    Mult(usize, usize),
}

impl AoC for Day3 {
    fn parse(mut input: String) -> Self {
        let mut input = input.as_mut_str();
        let mut instructions = Vec::new();

        while let Some(index) = input.find("mul(") {
            input = &mut input[(index + 4)..];

            let num = input
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();
            input = &mut input[num.len()..];

            let left = if !num.is_empty() && num.len() <= 3 {
                num.parse::<usize>().unwrap()
            } else {
                continue;
            };

            let comma = input.chars().next();
            input = &mut input[1..];

            if comma != Some(',') {
                continue;
            }

            let num = input
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();
            input = &mut input[num.len()..];

            let right = if !num.is_empty() && num.len() <= 3 {
                num.parse::<usize>().unwrap()
            } else {
                continue;
            };

            let right_paren = input.chars().next();
            input = &mut input[1..];

            if right_paren != Some(')') {
                continue;
            }

            instructions.push(InstructionType::Mult(left, right));
        }

        Self { instructions }
    }

    fn puzzle_one(&self) -> u64 {
        self.instructions
            .iter()
            .filter_map(|i| match i {
                InstructionType::Mult(l, r) => Some((l, r)),
                _ => None,
            })
            .map(|(left, right)| *left as u64 * *right as u64)
            .sum()
    }

    fn puzzle_two(&self) -> u64 {
        todo!()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file is present and intact");
    let day3 = Day3::parse(input);

    print!("\n\tpuzzle one: > {}\n", day3.puzzle_one());

    print!("\n\tpuzzle two: > {}\n", day3.puzzle_two());
}
