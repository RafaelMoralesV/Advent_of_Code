use aoc::AoC;

#[derive(Clone, Debug)]
struct MainState {
    rotations: Vec<LockRotation>,
}

#[derive(Clone, Debug)]
enum LockRotation {
    Left { steps: u32 },
    Right { steps: u32 },
}

impl AoC for MainState {
    fn parse(input: String) -> Self {
        Self {
            rotations: input
                .lines()
                .map(|line| {
                    let (dir, line) = line.split_at(1);
                    match dir {
                        "L" => LockRotation::Left {
                            steps: line.parse().unwrap(),
                        },
                        "R" => LockRotation::Right {
                            steps: line.parse().unwrap(),
                        },
                        _ => unreachable!(),
                    }
                })
                .collect(),
        }
    }

    fn puzzle_one(&mut self) -> u64 {
        self.rotations
            .iter()
            .fold((50, 0u64), |acc, e| {
                let steps: i64 = match e {
                    LockRotation::Left { steps } => i64::from(*steps) * -1,
                    LockRotation::Right { steps } => i64::from(*steps),
                };

                let mut dial_position = acc.0 as i64 + i64::from(steps);

                while dial_position < 0 {
                    dial_position = 100 + dial_position;
                }

                if dial_position > 99 {
                    dial_position %= 100;
                }

                let zeroes_found = if dial_position == 0 { acc.1 + 1 } else { acc.1 };

                (dial_position, zeroes_found)
            })
            .1
    }

    fn puzzle_two(&mut self) -> u64 {
        self.rotations
            .iter()
            .fold((50, 0u64), |acc, e| {
                let steps: i64 = match e {
                    LockRotation::Left { steps } => i64::from(*steps) * -1,
                    LockRotation::Right { steps } => i64::from(*steps),
                };

                let wrapping_zeroes = steps / 100;
                let steps = steps % 100;

                let mut dial_position = acc.0 as i64 + i64::from(steps);
                let mut zeroes = acc.1 + wrapping_zeroes.abs() as u64;

                if dial_position == 0 {
                    zeroes += 1;
                }

                if dial_position < 0 && acc.0 == 0 {
                    zeroes += 1;
                    dial_position = 100 + dial_position;
                }

                if dial_position > 99 {
                    zeroes += (dial_position / 100) as u64;
                    dial_position %= 100;
                };

                (dial_position, zeroes)
            })
            .1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_one_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut state = MainState::parse(input);

        assert_eq!(3, state.puzzle_one());
    }

    #[test]
    fn puzzle_two_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut state = MainState::parse(input);

        assert_eq!(6, state.puzzle_two());
    }

    #[test]
    fn multiple_rotations() {
        let input = "R1000";
        let mut state = MainState::parse(input.into());

        assert_eq!(10, state.puzzle_two());
    }

    #[test]
    fn tricky_rotation() {
        let input = "L350";
        let mut state = MainState::parse(input.into());

        assert_eq!(4, state.puzzle_two());
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Input file is present and intact");
    let mut state = MainState::parse(input);

    print!("\n\tpuzzle one: > {}\n", state.clone().puzzle_one());

    print!("\n\tpuzzle two: > {}\n", state.puzzle_two());
}
