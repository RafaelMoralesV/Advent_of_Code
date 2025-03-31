use std::{collections::HashSet, fs};

use aoc::AoC;

#[derive(Debug, Clone)]
struct Day6 {
    board: Vec<Vec<Tile>>,
    current_place: (usize, usize),
    direction: Direction,
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Blank,
    Visited { how: HashSet<Direction> },
    Obstable,
}

enum RunEnded {
    GuardIsStuckInLoop,
    GuardWentOutOfBounds,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

    fn next_tile(&mut self) -> Option<&mut Tile> {
        let (row, col) = self.next_tile_position()?;

        match self.direction {
            Direction::Up => self.board.get_mut(row).and_then(|row| row.get_mut(col)),
            Direction::Rigth => self.board.get_mut(row).and_then(|row| row.get_mut(col)),
            Direction::Down => self.board.get_mut(row).and_then(|row| row.get_mut(col)),
            Direction::Left => self.board.get_mut(row).and_then(|row| row.get_mut(col)),
        }
    }

    fn advance(&mut self) -> Result<(), RunEnded> {
        match self
            .next_tile()
            .ok_or(RunEnded::GuardWentOutOfBounds)?
            .clone()
        {
            Tile::Blank => {
                self.current_place = self
                    .next_tile_position()
                    .ok_or(RunEnded::GuardWentOutOfBounds)?;
                let tile = self
                    .board
                    .get_mut(self.current_place.0)
                    .ok_or(RunEnded::GuardWentOutOfBounds)?
                    .get_mut(self.current_place.1)
                    .ok_or(RunEnded::GuardWentOutOfBounds)?;

                *tile = Tile::Visited {
                    how: HashSet::from([self.direction.clone()]),
                };
            }
            Tile::Visited { mut how } => {
                if how.contains(&self.direction) {
                    return Err(RunEnded::GuardIsStuckInLoop);
                } else {
                    how.insert(self.direction.clone());
                    self.current_place = self
                        .next_tile_position()
                        .ok_or(RunEnded::GuardWentOutOfBounds)?;
                }
            }
            Tile::Obstable => {
                self.direction = Direction::rotate(&self.direction);
            }
        }

        Ok(())
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
                            Tile::Visited {
                                how: HashSet::from([Direction::Up]),
                            }
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
        while self.advance().ok().is_some() {}

        self.board
            .iter()
            .flatten()
            .filter(|tile| match **tile {
                Tile::Visited { how: _ } => true,
                _ => false,
            })
            .count() as u64
    }

    fn puzzle_two(&mut self) -> u64 {
        let mut stuck_places = 0;
        let mut seen_tiles = HashSet::new();
        let original_state = self.clone();

        // Consigo un Hashset de todos los Tiles que he visitado.
        loop {
            seen_tiles.insert(self.current_place);

            match self.advance() {
                Ok(_) => continue,
                Err(_) => break,
            }
        }

        // Reemplazo cada Tile por un obstaculo, luego simulo el juego.
        seen_tiles.iter().for_each(|(x, y)| {
            let mut new_board = original_state.clone();

            // No es necesario verificar que no haya sido obstaculo
            // porque el jugador jamas puede pisar un obstaculo en
            // primer lugar; ademas, esta indexacion es segura porque
            // son solo coordenadas que sabemos que existen.
            new_board.board[*x][*y] = Tile::Obstable;

            loop {
                match new_board.advance() {
                    Ok(_) => continue,
                    Err(RunEnded::GuardWentOutOfBounds) => break,
                    Err(RunEnded::GuardIsStuckInLoop) => {
                        stuck_places += 1;
                        break;
                    }
                }
            }
        });

        stuck_places
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file is present and intact");
    let mut day6 = Day6::parse(input);
    print!("\n\tpuzzle one: > {}\n", day6.puzzle_one());

    let input = fs::read_to_string("input.txt").expect("Input file is present and intact");
    let mut day6 = Day6::parse(input);
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

        assert_eq!(6, day6.puzzle_two());
    }

    #[test]
    fn it_can_see_the_correct_tiles() {
        let mut binding = Day6::parse(String::from(".\n^"));
        let should_be_blank = binding.next_tile();

        let mut binding = Day6::parse(String::from("#\n^"));
        let should_be_obstacle = binding.next_tile();

        let mut binding = Day6::parse(String::from("^"));
        let should_be_border = binding.next_tile();

        assert_eq!(should_be_blank, Some(&mut Tile::Blank));
        assert_eq!(should_be_obstacle, Some(&mut Tile::Obstable));
        assert_eq!(should_be_border, None);
    }
}
