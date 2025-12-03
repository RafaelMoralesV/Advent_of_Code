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
        self.ranges
            .clone()
            .into_iter()
            .flat_map(|range| range.from..=range.until)
            .filter(|n| check_invalid_expanded(*n))
            .sum()
    }
}

/// Solo necesito revisar que ambas mitades sean iguales.
fn check_invalid(number: u64) -> bool {
    let digits = number.ilog10() + 1;
    let cut = digits / 2;
    let upper = number / 10u64.pow(cut);
    let lower = number % 10u64.pow(cut);

    upper.eq(&lower)
}

/// Parto el numero en chunks de tamaños iguales, y comparo que esos chunks sean iguales.
fn check_invalid_expanded(number: u64) -> bool {
    let n_str = number.to_string();
    let digits = n_str.len();

    for i in 1..=(digits / 2) {
        if digits % i != 0 {
            continue;
        }

        let slice_iter = (0..digits).step_by(i).map(|start_idx| {
            let end_idx = start_idx + i;
            &n_str[start_idx..end_idx]
        });

        if all_equal(slice_iter) {
            return true;
        }
    }

    false
}

fn all_equal<T: Eq>(mut iter: impl Iterator<Item = T>) -> bool {
    let first = match iter.next() {
        None => return true,
        Some(val) => val,
    };

    iter.all(|item| item.eq(&first))
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

        assert_eq!(4174379265, state.puzzle_two());
    }

    #[test]
    fn known_invalid_ids() {
        assert!(check_invalid(11));
        assert!(check_invalid(22));
        assert!(check_invalid(99));
        assert!(check_invalid(1010));
        assert!(check_invalid(1188511885));
        assert!(check_invalid(222222));
        assert!(check_invalid(446446));
        assert!(check_invalid(38593859));
    }

    #[test]
    fn expanded_invalid_ids() {
        assert!(check_invalid_expanded(11));
        assert!(check_invalid_expanded(22));
        assert!(check_invalid_expanded(99));
        assert!(check_invalid_expanded(111));
        assert!(check_invalid_expanded(1010));
        assert!(check_invalid_expanded(1188511885));
        assert!(check_invalid_expanded(222222));
        assert!(check_invalid_expanded(446446));
        assert!(check_invalid_expanded(38593859));
        assert!(check_invalid_expanded(565656));
        assert!(check_invalid_expanded(824824824));
        assert!(check_invalid_expanded(2121212121));
    }

    #[test]
    fn known_valid_ids() {
        assert!(check_invalid(10) == false);
        assert!(check_invalid(21) == false);
        assert!(check_invalid(1009) == false);
        assert!(check_invalid(1188511884) == false);
        assert!(check_invalid(222221) == false);
        assert!(check_invalid(446445) == false);
        assert!(check_invalid(38593858) == false);
    }

    #[test]
    fn expanded_valid_ids() {
        assert!(check_invalid_expanded(10) == false);
        assert!(check_invalid_expanded(21) == false);
        assert!(check_invalid_expanded(1009) == false);
        assert!(check_invalid_expanded(1188511884) == false);
        assert!(check_invalid_expanded(222221) == false);
        assert!(check_invalid_expanded(446445) == false);
        assert!(check_invalid_expanded(38593858) == false);
    }
}
