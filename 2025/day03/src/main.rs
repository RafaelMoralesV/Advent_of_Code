use aoc::AoC;

#[derive(Clone, Debug)]
struct MainState {
    battery_arrays: Vec<Vec<u8>>,
}

impl AoC for MainState {
    fn parse(input: String) -> Self {
        Self {
            battery_arrays: input
                .lines()
                .map(|line| {
                    line.as_bytes()
                        .into_iter()
                        .map(|c| *c - ('0' as u8))
                        .collect()
                })
                .collect(),
        }
    }

    fn puzzle_one(&mut self) -> u64 {
        self.battery_arrays
            .iter()
            .map(|array| {
                let (index, largest) = &array[0..array.len() - 1]
                    .iter()
                    .enumerate()
                    .fold((0, 0), |acc, (i, v)| if acc.1 < *v { (i, *v) } else { acc });

                let next_largest = &array[*index + 1..array.len()].iter().max().unwrap();

                u64::from(largest * 10 + **next_largest)
            })
            .sum()
    }

    fn puzzle_two(&mut self) -> u64 {
        todo!()
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

        assert_eq!(357, state.puzzle_one());
    }

    #[test]
    fn row_check() {
        let mut state = MainState::parse(String::from("987654321111111"));

        assert_eq!(98, state.puzzle_one());
    }

    #[test]
    fn puzzle_two_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut state = MainState::parse(input);

        // assert_eq!(4174379265, state.puzzle_two());
    }
}
