use aoc::AoC;

struct Day11 {
    rocks: Vec<u64>,
}

impl AoC for Day11 {
    fn parse(input: String) -> Self {
        Day11 {
            rocks: input
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }

    fn puzzle_one(&mut self) -> u64 {
        for _ in 0..25 {
            self.rocks = self.blink();
        }

        self.rocks.len() as u64
    }

    fn puzzle_two(&mut self) -> u64 {
        for _ in 0..75 {
            self.rocks = self.blink();
        }

        self.rocks.len() as u64
    }
}

impl Day11 {
    fn blink(&self) -> Vec<u64> {
        self.rocks
            .iter()
            .map(|rock| {
                if *rock == 0 {
                    return vec![1];
                }

                let rock_str = rock.to_string();
                if rock_str.len() % 2 == 0 {
                    let (one, two) = rock_str.split_at(rock_str.len() / 2);
                    return vec![one.parse().unwrap(), two.parse().unwrap()];
                }

                vec![rock * 2024]
            })
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_one_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut day11 = Day11::parse(input);

        assert_eq!(37, day11.puzzle_one());
    }

    #[test]
    fn puzzle_two_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut day11 = Day11::parse(input);

        assert_eq!(2858, day11.puzzle_two());
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Input file is present and intact");
    let mut day11 = Day11::parse(input);

    print!("\n\tpuzzle one: > {}\n", day11.puzzle_one());

    print!("\n\tpuzzle two: > {}\n", day11.puzzle_two());
}
