use std::collections::{HashMap, HashSet};

use aoc::AoC;

#[derive(Clone, Debug)]
struct MainState {
    problems: Vec<Problem>,
}

#[derive(Clone, Debug)]
struct Problem {
    numbers: Vec<usize>,
    problem_type: ProblemType,
}

#[derive(Clone, Debug)]
enum ProblemType {
    Multiplication,
    Addition,
}

impl AoC for MainState {
    fn parse(input: String) -> Self {
        let mut problems = vec![];
        let mut numbers: HashMap<usize, Vec<usize>> = HashMap::new();

        input.lines().for_each(|line| {
            let mut curr_index = 0;
            line.split_whitespace().for_each(|n| {
                match n {
                    "*" => {
                        let problem_type = ProblemType::Multiplication;
                        let problem = Problem {
                            numbers: numbers.get(&curr_index).unwrap().clone(),
                            problem_type,
                        };
                        problems.push(problem)
                    }
                    "+" => {
                        let problem_type = ProblemType::Addition;
                        let problem = Problem {
                            numbers: numbers.get(&curr_index).unwrap().clone(),
                            problem_type,
                        };
                        problems.push(problem)
                    }
                    n => {
                        numbers
                            .entry(curr_index)
                            .or_insert(vec![])
                            .push(n.parse::<usize>().unwrap());
                    }
                }

                curr_index = curr_index + 1;
            });
        });

        Self { problems }
    }

    fn puzzle_one(&mut self) -> u64 {
        self.problems
            .iter()
            .map(|problem| {
                let num = match problem.problem_type {
                    ProblemType::Addition => problem
                        .numbers
                        .iter()
                        .fold(0u64, |acc, n| acc + (*n as u64)),
                    ProblemType::Multiplication => problem
                        .numbers
                        .iter()
                        .fold(1u64, |acc, n| acc * (*n as u64)),
                };

                num
            })
            .sum::<u64>()
    }

    fn puzzle_two(&mut self) -> u64 {
        todo!();
        0
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Input file is present and intact");
    let mut state = MainState::parse(input);

    print!("\n\tpuzzle one: > {}\n", state.clone().puzzle_one());

    print!("\n\tpuzzle two: > {}\n", state.puzzle_two());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_one_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut state = MainState::parse(input);

        assert_eq!(4277556, state.puzzle_one());
    }

    #[test]
    fn puzzle_two_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut state = MainState::parse(input);

        assert_eq!(14, state.puzzle_two());
    }
}
