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

fn get_numbers(input: &mut String) -> Option<usize> {
    let prefix = input
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();

    if !prefix.is_empty() && prefix.len() <= 3 {
        let num = prefix.parse().ok();
        *input = input.strip_prefix(&prefix).unwrap().to_string();

        num
    } else {
        None
    }
}

fn get_mul(input: &mut String) -> Option<InstructionType> {
    let left = get_numbers(input)?;
    *input = input.strip_prefix(",")?.to_string();
    let right = get_numbers(input)?;
    *input = input.strip_prefix(")")?.to_string();

    Some(InstructionType::Mult(left, right))
}

impl AoC for Day3 {
    fn parse(mut input: String) -> Self {
        let mut instructions = Vec::new();

        while !input.is_empty() {
            if let Some(next) = input.strip_prefix("do()") {
                input = next.to_string();
                instructions.push(InstructionType::Do);
                continue;
            }

            if let Some(next) = input.strip_prefix("don't()") {
                input = next.to_string();
                instructions.push(InstructionType::Dont);
                continue;
            }

            if let Some(next) = input.strip_prefix("mul(") {
                input = next.to_string();

                if let Some(mul) = get_mul(&mut input) {
                    instructions.push(mul);
                }

                continue;
            }

            input.remove(0);
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
        let mut doing_work = true;
        let mut total = 0;
        for instruction in self.instructions.iter() {
            match instruction {
                InstructionType::Do => doing_work = true,
                InstructionType::Dont => doing_work = false,
                InstructionType::Mult(l, r) => {
                    if doing_work {
                        total += (l * r) as u64;
                    }
                }
            }
        }

        total
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file is present and intact");
    let day3 = Day3::parse(input);

    print!("\n\tpuzzle one: > {}\n", day3.puzzle_one());

    print!("\n\tpuzzle two: > {}\n", day3.puzzle_two());
}
