use aoc::AoC;
use itertools::Itertools;

#[derive(Debug)]
struct Day9 {
    filesystem: Vec<FSBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FSBlock {
    File { id: u64 },
    Blank,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FSChunk {
    block_type: FSBlock,
    size: usize,
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
        while self.filesystem.last() == Some(&FSBlock::Blank) {
            self.filesystem.pop();
        }

        // Modifico el vector hacia una representacion 'de mas alto nivel' en forma de chunks.
        let mut fs: Vec<_> = self
            .filesystem
            .clone()
            .into_iter()
            .chunk_by(|item| item.clone())
            .into_iter()
            .map(|(variant, group)| FSChunk {
                block_type: variant,
                size: group.count(),
            })
            .collect();

        // A partir de esto tengo que iterar y mutar el vector...
        let file_sizes = fs
            .iter()
            .filter_map(|f| match f.block_type {
                FSBlock::Blank => None,
                FSBlock::File { id } => Some((id, f.size)),
            })
            .rev()
            .collect::<Vec<_>>();

        for (file_id, size) in file_sizes {
            if let Some((blank_index, _)) = fs
                .iter()
                .find_position(|f| matches!(f.block_type, FSBlock::Blank) && f.size >= size)
            {
                let (file_index, _) = fs
                    .iter()
                    .find_position(|i| match i.block_type {
                        FSBlock::File { id } => file_id == id,
                        _ => false,
                    })
                    .unwrap();

                if blank_index > file_index {
                    continue;
                }

                fs[blank_index].size -= size;
                let file = fs.remove(file_index);

                fs.splice(blank_index..blank_index, vec![file]);
                fs.splice(
                    file_index + 1..file_index + 1,
                    vec![FSChunk {
                        block_type: FSBlock::Blank,
                        size,
                    }],
                );
            }
        }

        self.filesystem = fs
            .iter()
            .map(|i| vec![i.block_type.clone(); i.size])
            .flatten()
            .collect::<Vec<_>>();

        self.filesystem
            .iter()
            .map(|fsb| match fsb {
                FSBlock::File { id } => id,
                FSBlock::Blank => &0,
            })
            .enumerate()
            .fold(0, |acc, (i, id)| acc + i as u64 * id)
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

        assert_eq!(2858, day9.puzzle_two());
    }
}
