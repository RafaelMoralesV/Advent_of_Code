use aoc::AoC;

struct Day12 {
    // ...
}

impl AoC for Day12 {
    fn parse(input: String) -> Self {
        todo!()
    }

    fn puzzle_one(&mut self) -> u64 {
        todo!()
    }

    fn puzzle_two(&mut self) -> u64 {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_one_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut day12 = Day12::parse(input);

        assert_eq!(55312, day12.puzzle_one());
    }

    #[test]
    fn puzzle_two_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut day12 = Day12::parse(input);

        assert_eq!(55312, day12.puzzle_two());
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Input file is present and intact");
    let mut day12 = Day12::parse(input.clone());

    print!("\n\tpuzzle one: > {}\n", day12.puzzle_one());

    let mut day12 = Day12::parse(input);
    print!("\n\tpuzzle two: > {}\n", day12.puzzle_two());
}
