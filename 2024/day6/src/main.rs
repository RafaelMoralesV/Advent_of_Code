use std::fs;

use aoc::AoC;

#[derive(Debug)]
struct Day6 {
    board: Vec<Vec<Tile>>,
    current_place: (usize, usize),
    direction: Direction,
}

#[derive(Debug, PartialEq)]
enum Tile {
    Blank,
    Visited,
    Obstable,
}

#[derive(Debug)]
enum Direction {
    Up,
    Rigth,
    Down,
    Left,
}

impl Direction {
    fn rotate(current: &Self) -> Self {
        match current {
            Direction::Up => Direction::Rigth,
            Direction::Rigth => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Day6 {
    fn next_tile_position(&self) -> Option<(usize, usize)> {
        let (x, y) = self.current_place;
        match self.direction {
            Direction::Up => Some((x.checked_sub(1)?, y)),
            Direction::Rigth => Some((x, y + 1)),
            Direction::Down => Some((x + 1, y)),
            Direction::Left => Some((x, y.checked_sub(1)?)),
        }
    }

    fn next_tile(&self) -> Option<&Tile> {
        let (row, col) = self.next_tile_position()?;

        match self.direction {
            Direction::Up => self.board.get(row).and_then(|row| row.get(col)),
            Direction::Rigth => self.board.get(row).and_then(|row| row.get(col)),
            Direction::Down => self.board.get(row).and_then(|row| row.get(col)),
            Direction::Left => self.board.get(row).and_then(|row| row.get(col)),
        }
    }

    fn advance(&mut self) -> Option<()> {
        match self.next_tile()? {
            Tile::Blank => {
                self.current_place = self.next_tile_position()?;
                let tile = self
                    .board
                    .get_mut(self.current_place.0)?
                    .get_mut(self.current_place.1)?;

                *tile = Tile::Visited;
            }
            Tile::Visited => {
                self.current_place = self.next_tile_position()?;
            }
            Tile::Obstable => {
                self.direction = Direction::rotate(&self.direction);
            }
        }

        Some(())
    }
}

impl AoC for Day6 {
    fn parse(input: String) -> Self {
        let mut current_place = (0, 0);

        let board = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '#' => Tile::Obstable,
                        '^' => {
                            current_place = (i, j);
                            Tile::Visited
                        }
                        _ => Tile::Blank,
                    })
                    .collect()
            })
            .collect();

        Self {
            board,
            current_place,
            direction: Direction::Up,
        }
    }

    fn puzzle_one(&mut self) -> u64 {
        while self.advance().is_some() {}

        self.board
            .iter()
            .flatten()
            .filter(|tile| **tile == Tile::Visited)
            .count() as u64
    }

    fn puzzle_two(&mut self) -> u64 {
        todo!()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file is present and intact");
    let mut day6 = Day6::parse(input);

    print!("\n\tpuzzle one: > {}\n", day6.puzzle_one());

    print!("\n\tpuzzle two: > {}\n", day6.puzzle_two());
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> Day6 {
        let input =
            fs::read_to_string("example_one.txt").expect("Input file is present and intact");

        Day6::parse(input)
    }

    #[test]
    fn puzzle_one_example() {
        let mut day6 = setup();

        assert_eq!(41, day6.puzzle_one());
    }

    #[test]
    fn puzzle_two_example() {
        let mut day6 = setup();

        assert_eq!(123, day6.puzzle_two());
    }

    #[test]
    fn it_can_see_the_correct_tiles() {
        let binding = Day6::parse(String::from(".\n^"));
        let should_be_blank = binding.next_tile();

        let binding = Day6::parse(String::from("#\n^"));
        let should_be_obstacle = binding.next_tile();

        let binding = Day6::parse(String::from("^"));
        let should_be_border = binding.next_tile();

        assert_eq!(should_be_blank, Some(&Tile::Blank));
        assert_eq!(should_be_obstacle, Some(&Tile::Obstable));
        assert_eq!(should_be_border, None);
    }
}
