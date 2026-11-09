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
            if line.trim().is_empty() {
                reading_available_ids = true;
            } else {
                if !reading_available_ids {
                    let (from, to) = line.split_once('-').unwrap();
                    let mut from = from.parse::<usize>().unwrap();
                    let mut to = to.parse::<usize>().unwrap();

                    fresh_ids.iter_mut().for_each(|range: &mut Ranges| {
                        // No coinciden
                        // Desde esta dentro del rango
                        // Hasta esta dentro del rango
                        // Ambos estan dentro del rango, mi rango desaparece
                        // El rango esta dentro de mi nuevo rango, el anterior desaparece
                        let from_is_inside = range.from <= from && range.to >= from;
                        let to_is_inside = range.from <= to && range.to >= to;

                        if from_is_inside && to_is_inside {
                            to = 0;
                            from = 0;
                        }

                        if from_is_inside {
                            from = range.to + 1;
                        }

                        if to_is_inside {
                            to = range.from - 1;
                        }

                        // El rango anterior es contenido por el nuevo. Lo quitamos.
                        if from <= range.from
                            && to >= range.from
                            && from <= range.to
                            && to >= range.to
                        {
                            range.to = 0;
                            range.from = 0;
                        }
                    });

                    if from <= to {
                        fresh_ids.push(Ranges { to, from });
                    }
                } else {
                    available_ids.push(line.parse::<usize>().unwrap());
                }
            }
        });

        let fresh_ids = fresh_ids
            .into_iter()
            .filter(|range| range.from != 0 && range.to != 0)
            .collect();

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
        self.fresh_ids
            .iter()
            .map(|range| range.to as u64 - range.from as u64 + 1u64)
            .sum::<u64>()
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

        dbg!(&state.fresh_ids);

        assert_eq!(14, state.puzzle_two());
    }
}
