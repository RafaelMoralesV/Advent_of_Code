use std::collections::HashMap;

use aoc::AoC;
use either::*;

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
        type Cache = HashMap<u64, u64>;
        let mut cache = Cache::new();

        for rock in self.rocks.iter() {
            cache.entry(*rock).insert_entry(1);
        }

        for _ in 0..75 {
            let mut new_cache = Cache::new();

            for (rock, count) in cache.into_iter() {
                if rock == 0 {
                    new_cache
                        .entry(1)
                        .and_modify(|v| *v += count)
                        .or_insert(count);

                    continue;
                }

                let rock_digits = rock.ilog10() + 1;
                if rock_digits % 2 == 0 {
                    // Izquierda
                    new_cache
                        .entry(rock / 10u64.pow(rock_digits / 2))
                        .and_modify(|v| *v += count)
                        .or_insert(count);

                    // Derecha
                    new_cache
                        .entry(rock % 10u64.pow(rock_digits / 2))
                        .and_modify(|v| *v += count)
                        .or_insert(count);

                    continue;
                }

                new_cache
                    .entry(rock * 2024)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
            }

            cache = new_cache;
        }

        cache.values().sum::<u64>()
    }
}

impl Day11 {
    fn blink(&self) -> Vec<u64> {
        self.rocks
            .iter()
            .flat_map(|rock| {
                // Si es cero, se convierte en uno
                if *rock == 0 {
                    return Either::<[u64; 1], [u64; 2]>::Left([1]).into_iter();
                }

                // Si tiene digitos pares, se parten en dos y se convierte cada mitad
                // en dos rocas distintas
                let rock_digits = rock.ilog10() + 1;
                if rock_digits % 2 == 0 {
                    return Either::<[u64; 1], [u64; 2]>::Right([
                        rock / 10u64.pow(rock_digits / 2),
                        rock % 10u64.pow(rock_digits / 2),
                    ])
                    .into_iter();
                }

                // En cualquier otro caso, la roca se multiplica por 2024
                Either::<[u64; 1], [u64; 2]>::Left([rock * 2024]).into_iter()
            })
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

        assert_eq!(55312, day11.puzzle_one());
    }

    #[test]
    fn puzzle_two_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut day11 = Day11::parse(input);

        assert_eq!(55312, day11.puzzle_two());
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Input file is present and intact");
    let mut day11 = Day11::parse(input.clone());

    print!("\n\tpuzzle one: > {}\n", day11.puzzle_one());

    let mut day11 = Day11::parse(input);
    print!("\n\tpuzzle two: > {}\n", day11.puzzle_two());
}
