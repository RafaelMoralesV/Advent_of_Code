use aoc::AoC;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Day8 {
    satelites: Satelites,
    map: Vec<Vec<char>>,
}

type Satelites = HashMap<char, Vec<(usize, usize)>>;
type Coords = (usize, usize);

fn get_projection(one: &Coords, two: &Coords) -> Option<Coords> {
    let x = (2 * one.0).checked_sub(two.0)?;
    let y = (2 * one.1).checked_sub(two.1)?;

    Some((x, y))
}

fn get_harmonic_projections(one: &Coords, two: &Coords, map: &[Vec<char>]) -> Vec<Coords> {
    let mut res = vec![*one, *two];
    let mut one = *one;
    let mut two = *two;

    loop {
        let next = get_projection(&one, &two);
        if let Some(next) = next.and_then(|next| {
            map.get(next.0)
                .and_then(|row| row.get(next.1))
                .map(|_| next)
        }) {
            res.push(next);
            two = one;
            one = next;
        } else {
            break;
        }
    }

    res
}

impl AoC for Day8 {
    fn parse(input: String) -> Self {
        let mut satelites: Satelites = HashMap::new();
        let mut map = vec![];

        input.lines().enumerate().for_each(|(i, line)| {
            map.push(line.chars().enumerate().fold(vec![], |mut acc, (j, c)| {
                if c.ne(&'.') {
                    satelites.entry(c).or_default().push((i, j));
                }

                acc.push(c);

                acc
            }));
        });

        Self { satelites, map }
    }

    fn puzzle_one(&mut self) -> u64 {
        self.satelites
            .values()
            // Se necesitan dos o mas antenas para crear antinodos.
            .filter(|val| val.len() > 1)
            .fold(vec![], |mut acc, positions| {
                acc.push(
                    positions
                        .iter()
                        .tuple_combinations::<(&Coords, &Coords)>()
                        // Genero un arreglo con ambas proyecciones.
                        .flat_map(|(a, b)| vec![get_projection(a, b), get_projection(b, a)])
                        // Filtro solo las proyecciones exitosas.
                        .flatten()
                        // Fitlro solo a lugares que existen en el mapa.
                        .flat_map(|(x, y)| {
                            self.map
                                .get(x)
                                .and_then(|row| row.get(y))
                                .map(|c| (c, x, y))
                        })
                        .collect::<Vec<_>>(),
                );

                acc
            })
            .iter()
            .flatten()
            .unique()
            .count() as u64
    }

    fn puzzle_two(&mut self) -> u64 {
        self.satelites
            .values()
            // Se necesitan dos o mas antenas para crear antinodos.
            .filter(|val| val.len() > 1)
            .fold(vec![], |mut acc, positions| {
                acc.push(
                    positions
                        .iter()
                        .tuple_combinations::<(&Coords, &Coords)>()
                        // Genero un arreglo con ambas proyecciones.
                        .flat_map(|(a, b)| {
                            vec![
                                get_harmonic_projections(a, b, &self.map),
                                get_harmonic_projections(b, a, &self.map),
                            ]
                        })
                        // Filtro solo las proyecciones exitosas.
                        .flatten()
                        // Fitlro solo a lugares que existen en el mapa.
                        .collect::<Vec<_>>(),
                );

                acc
            })
            .into_iter()
            .flatten()
            .unique()
            .count() as u64
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Input file is present and intact");
    let mut day8 = Day8::parse(input);

    print!("\n\tpuzzle one: > {}\n", day8.puzzle_one());

    let input = std::fs::read_to_string("input.txt").expect("Input file is present and intact");
    let mut day8 = Day8::parse(input);
    print!("\n\tpuzzle two: > {}\n", day8.puzzle_two());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_one_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut day8 = Day8::parse(input);

        assert_eq!(14, day8.puzzle_one());
    }

    #[test]
    fn puzzle_two_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut day8 = Day8::parse(input);

        assert_eq!(34, day8.puzzle_two());
    }
}
