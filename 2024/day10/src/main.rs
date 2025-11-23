use aoc::AoC;
use std::fs;

struct Day10 {
    map: Vec<Vec<u8>>,
}

impl AoC for Day10 {
    fn parse(input: String) -> Self {
        Day10 {
            map: input
                .lines()
                .map(|line| {
                    line.chars()
                        .flat_map(|ch| ch.to_string().parse().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        }
    }

    fn puzzle_one(&mut self) -> u64 {
        let mut acc: u64 = 0;
        for (x, row) in self.map.clone().into_iter().enumerate() {
            for (y, number) in row.into_iter().enumerate() {
                if number == 0 {
                    let mut seen_trailheads = vec![];
                    let paths = self.viable_paths(x, y, &mut seen_trailheads) as u64;
                    //println!("Encontre {paths} caminos en [{x}][{y}]");
                    acc += paths;
                }
            }
        }

        acc
    }

    fn puzzle_two(&mut self) -> u64 {
        let mut acc: u64 = 0;
        for (x, row) in self.map.clone().into_iter().enumerate() {
            for (y, number) in row.into_iter().enumerate() {
                if number == 0 {
                    let paths = self.path_ratings(x, y) as u64;
                    //println!("Encontre {paths} caminos en [{x}][{y}]");
                    acc += paths;
                }
            }
        }

        acc
    }
}

impl Day10 {
    fn path_ratings(&self, x: usize, y: usize) -> usize {
        if self.map[x][y] == 9 {
            return 1;
        }

        let mut paths = 0;

        if self.can_go_up(x, y) {
            //println!(" >[{x}][{y}] UP ");
            paths += self.path_ratings(x - 1, y);
        }

        if self.can_go_down(x, y) {
            //println!(" >[{x}][{y}] DOWN ");
            paths += self.path_ratings(x + 1, y);
        }

        if self.can_go_left(x, y) {
            //println!(" >[{x}][{y}] LEFT ");
            paths += self.path_ratings(x, y - 1);
        }

        if self.can_go_right(x, y) {
            //println!(" >[{x}][{y}] RIGHT ");
            paths += self.path_ratings(x, y + 1);
        }

        paths
    }
    fn viable_paths(&self, x: usize, y: usize, seen_trailheads: &mut Vec<(usize, usize)>) -> usize {
        if self.map[x][y] == 9 {
            if seen_trailheads.contains(&(x, y)) {
                return 0;
            }

            seen_trailheads.push((x, y));
            return 1;
        }

        let mut paths = 0;

        if self.can_go_up(x, y) {
            //println!(" >[{x}][{y}] UP ");
            paths += self.viable_paths(x - 1, y, seen_trailheads);
        }

        if self.can_go_down(x, y) {
            //println!(" >[{x}][{y}] DOWN ");
            paths += self.viable_paths(x + 1, y, seen_trailheads);
        }

        if self.can_go_left(x, y) {
            //println!(" >[{x}][{y}] LEFT ");
            paths += self.viable_paths(x, y - 1, seen_trailheads);
        }

        if self.can_go_right(x, y) {
            //println!(" >[{x}][{y}] RIGHT ");
            paths += self.viable_paths(x, y + 1, seen_trailheads);
        }

        paths
    }

    fn exists_up(&self, x: usize, y: usize) -> bool {
        if x == 0 {
            return false;
        }

        self.map.get(x - 1).is_some_and(|row| row.get(y).is_some())
    }

    fn exists_down(&self, x: usize, y: usize) -> bool {
        self.map.get(x + 1).is_some_and(|row| row.get(y).is_some())
    }

    fn exists_left(&self, x: usize, y: usize) -> bool {
        if y == 0 {
            return false;
        }

        self.map.get(x).is_some_and(|row| row.get(y - 1).is_some())
    }

    fn exists_right(&self, x: usize, y: usize) -> bool {
        self.map.get(x).is_some_and(|row| row.get(y + 1).is_some())
    }

    fn can_go_up(&self, x: usize, y: usize) -> bool {
        self.exists_up(x, y) && self.map[x][y] + 1 == self.map[x - 1][y]
    }

    fn can_go_down(&self, x: usize, y: usize) -> bool {
        self.exists_down(x, y) && self.map[x][y] + 1 == self.map[x + 1][y]
    }
    fn can_go_left(&self, x: usize, y: usize) -> bool {
        self.exists_left(x, y) && self.map[x][y] + 1 == self.map[x][y - 1]
    }
    fn can_go_right(&self, x: usize, y: usize) -> bool {
        self.exists_right(x, y) && self.map[x][y] + 1 == self.map[x][y + 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_one_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut day10 = Day10::parse(input);

        assert_eq!(36, day10.puzzle_one());
    }

    #[test]
    fn puzzle_two_example() {
        let input =
            std::fs::read_to_string("example_input.txt").expect("Input file is present and intact");

        let mut day10 = Day10::parse(input);

        assert_eq!(2858, day10.puzzle_two());
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file is present and intact");
    let mut day10 = Day10::parse(input);

    print!("\n\tpuzzle one: > {}\n", day10.puzzle_one());

    print!("\n\tpuzzle two: > {}\n", day10.puzzle_two());
}
