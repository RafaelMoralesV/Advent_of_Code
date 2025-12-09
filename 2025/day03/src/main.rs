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
                .map(|line| line.as_bytes().iter().map(|c| *c - b'0').collect())
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
        self.battery_arrays
            .iter()
            .map(|arr| joltage_from_twelve(arr))
            .sum()
    }
}

fn joltage_from_twelve(array: &[u8]) -> u64 {
    const EXPECTED_LENGTH: usize = 12;
    let mut missing_length = EXPECTED_LENGTH;

    let mut prev_index = 0;
    let mut total: u64 = 0;
    while missing_length > 0 {
        let allowed_length = array.len() - missing_length;

        let (index, max) = array[prev_index..=allowed_length]
            .iter()
            .enumerate()
            .fold((0, 0), |acc, (i, v)| if acc.1 < *v { (i, *v) } else { acc });

        prev_index += index + 1;
        total = total * 10 + u64::from(max);

        missing_length -= 1;
    }

    total
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
        assert_eq!(987654321111, state.puzzle_two());
    }

    #[test]
    fn puzzle_two_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut state = MainState::parse(input);

        assert_eq!(3121910778619, state.puzzle_two());
    }
}
