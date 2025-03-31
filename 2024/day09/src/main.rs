use aoc::AoC;

#[derive(Debug)]
struct Day9 {
    filesystem: Vec<FSBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FSBlock {
    File { id: u64 },
    Blank,
}

impl AoC for Day9 {
    fn parse(input: String) -> Self {
        let mut ind = false;
        let mut id = 0;
        let filesystem = input
            .chars()
            .filter_map(|c| c.to_digit(10))
            .flat_map(|n| {
                ind = !ind;
                if ind {
                    // File
                    id += 1;

                    vec![FSBlock::File { id: id - 1 }; n as usize]
                } else {
                    // Blank
                    vec![FSBlock::Blank; n as usize]
                }
            })
            .collect();

        Self { filesystem }
    }

    fn puzzle_one(&mut self) -> u64 {
        while self.filesystem.last() == Some(&FSBlock::Blank) {
            self.filesystem.pop();
        }

        while self.filesystem.contains(&FSBlock::Blank) {
            // Intercambio el siguiente espacio en blanco con el ultimo valor del vector;
            let i = self
                .filesystem
                .iter()
                .position(|fsb| *fsb == FSBlock::Blank)
                .unwrap();

            let j = self.filesystem.len() - 1;

            self.filesystem.swap(i, j);

            self.filesystem.pop();
        }

        self.filesystem
            .iter()
            .filter_map(|fsb| match fsb {
                FSBlock::File { id } => Some(id),
                FSBlock::Blank => None,
            })
            .enumerate()
            .fold(0, |acc, (i, id)| acc + i as u64 * id)
    }

    fn puzzle_two(&mut self) -> u64 {
        todo!()
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Input file is present and intact");
    let mut day9 = Day9::parse(input);

    print!("\n\tpuzzle one: > {}\n", day9.puzzle_one());

    let input = std::fs::read_to_string("input.txt").expect("Input file is present and intact");
    let mut day9 = Day9::parse(input);
    print!("\n\tpuzzle two: > {}\n", day9.puzzle_two());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_one_example() {
        let input = String::from("2333133121414131402");

        let mut day9 = Day9::parse(input);

        assert_eq!(1928, day9.puzzle_one());
    }

    #[test]
    fn puzzle_two_example() {
        let input = String::from("2333133121414131402");

        let mut day9 = Day9::parse(input);

        assert_eq!(34, day9.puzzle_two());
    }
}
