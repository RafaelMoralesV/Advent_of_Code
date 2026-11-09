use aoc::AoC;

#[derive(Clone, Debug)]
struct MainState {
    fresh_ids: Vec<Ranges>,
    available_ids: Vec<usize>,
}

#[derive(Clone, Debug)]
struct Ranges {
    from: usize,
    to: usize,
}

impl AoC for MainState {
    fn parse(input: String) -> Self {
        let mut reading_available_ids = false;
        let mut fresh_ids = vec![];
        let mut available_ids = vec![];

        input.lines().for_each(|line| {
            println!("{line}");
            if line.trim().is_empty() {
                reading_available_ids = true;
            } else {
                if !reading_available_ids {
                    let (from, to) = line.split_once('-').unwrap();
                    let from = from.parse::<usize>().unwrap();
                    let to = to.parse::<usize>().unwrap();

                    fresh_ids.push(Ranges { to, from });
                } else {
                    available_ids.push(line.parse::<usize>().unwrap());
                }
            }
        });

        Self {
            fresh_ids,
            available_ids,
        }
    }

    fn puzzle_one(&mut self) -> u64 {
        let mut intersection: u64 = 0;

        self.available_ids.iter().for_each(|id| {
            let is_fresh = self
                .fresh_ids
                .iter()
                .any(|range| range.to >= *id && range.from <= *id);

            if is_fresh {
                intersection += 1
            }
        });

        intersection
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

        assert_eq!(3, state.puzzle_one());
    }

    #[test]
    fn puzzle_two_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut state = MainState::parse(input);

        // assert_eq!(3121910778619, state.puzzle_two());
    }
}
