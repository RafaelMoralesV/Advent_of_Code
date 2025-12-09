use aoc::AoC;

#[derive(Clone, Debug)]
struct MainState {
    floor: Vec<Vec<FloorTile>>,
}

#[derive(Clone, Debug)]
enum FloorTile {
    HasRoll,
    IsEmpty,
}

impl AoC for MainState {
    fn parse(input: String) -> Self {
        Self {
            floor: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '@' => FloorTile::HasRoll,
                            '.' => FloorTile::IsEmpty,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn puzzle_one(&mut self) -> u64 {
        let floor_ref = &self.floor;

        floor_ref
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter().enumerate().map(move |(y, tile)| {
                    if matches!(tile, FloorTile::HasRoll) {
                        get_surrounding_tiles(floor_ref, x, y)
                    } else {
                        4
                    }
                })
            })
            .filter(|count| *count < 4)
            .count() as u64
    }

    fn puzzle_two(&mut self) -> u64 {
        todo!()
    }
}

fn get_surrounding_tiles(floor: &Vec<Vec<FloorTile>>, x: usize, y: usize) -> u64 {
    let mut count = 0;

    // Convert current usize coordinates to signed isize (or i32) for calculation
    let sx = x as isize;
    let sy = y as isize;

    // Iterate over offsets
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue; // Skip the center tile
            }

            let nx = sx + dx;
            let ny = sy + dy;

            // Check if the resulting coordinates are negative (out of bounds on the left/top)
            if nx < 0 || ny < 0 {
                continue;
            }

            // Convert back to usize, which is safe since we checked for negative values
            let next_x = nx as usize;
            let next_y = ny as usize;

            // Use floor.get() to safely check bounds on the right/bottom
            if let Some(tile) = floor.get(next_x).and_then(|row| row.get(next_y)) {
                if matches!(tile, FloorTile::HasRoll) {
                    count += 1;
                }
            }
        }
    }
    count
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

        assert_eq!(13, state.puzzle_one());
    }

    #[test]
    fn puzzle_two_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut state = MainState::parse(input);

        // assert_eq!(3121910778619, state.puzzle_two());
    }
}
