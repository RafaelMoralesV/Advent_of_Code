use aoc::AoC;

#[derive(Clone, Debug)]
struct MainState {
    ranges: Vec<Range>,
}

#[derive(Clone, Debug)]
struct Range {
    from: u64,
    until: u64,
}

impl AoC for MainState {
    fn parse(input: String) -> Self {
        Self {
            ranges: input
                .split(',')
                .into_iter()
                .map(|ranges| {
                    let (from, until) = ranges.trim().split_once('-').unwrap();
                    let from = from.parse().unwrap();
                    let until = until.parse().unwrap();

                    Range { from, until }
                })
                .collect(),
        }
    }

    fn puzzle_one(&mut self) -> u64 {
        self.ranges
            .clone()
            .into_iter()
            .flat_map(|range| range.from..=range.until)
            .filter(|n| check_invalid(*n))
            .sum()
    }

    fn puzzle_two(&mut self) -> u64 {
        todo!()
    }
}

fn check_invalid(number: u64) -> bool {
    let digits = number.ilog10() + 1;
    let cut = digits / 2;
    let upper = number / 10u64.pow(cut);
    let lower = number % 10u64.pow(cut);

    upper.eq(&lower)
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

        assert_eq!(1227775554, state.puzzle_one());
    }

    #[test]
    fn puzzle_two_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut state = MainState::parse(input);

        assert_eq!(1227775554, state.puzzle_two());
    }

    #[test]
    fn known_invalid_ids() {
        assert!(check_invalid(11));
        assert!(check_invalid(22));
        assert!(check_invalid(1010));
        assert!(check_invalid(1188511885));
        assert!(check_invalid(222222));
        assert!(check_invalid(446446));
        assert!(check_invalid(38593859));
    }

    #[test]
    fn known_valid_ids() {
        assert!(check_invalid(10).eq(&false));
        assert!(check_invalid(21).eq(&false));
        assert!(check_invalid(1009).eq(&false));
        assert!(check_invalid(1188511884).eq(&false));
        assert!(check_invalid(222221).eq(&false));
        assert!(check_invalid(446445).eq(&false));
        assert!(check_invalid(38593858).eq(&false));
    }
}
