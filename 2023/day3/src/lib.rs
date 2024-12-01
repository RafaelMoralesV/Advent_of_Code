use std::{collections::HashSet, fs};

#[derive(Debug)]
pub enum Symbol {
    Connection(char),
    Number(usize),
}

#[derive(Debug)]

pub struct Part {
    symbol: Symbol,
    neighbors: HashSet<(usize, usize)>,
}

pub fn problem(s: String) -> Vec<Part> {
    let mut parts = vec![];

    for (row, line) in s.lines().enumerate() {
        let mut current_number: Option<usize> = None;
        let mut currrent_neighbors: HashSet<(usize, usize)> = HashSet::new();

        for (col, c) in line.chars().enumerate() {
            if let Some(n) = c.to_digit(10) {
                current_number = Some(current_number.unwrap_or_default() * 10 + n as usize);
                currrent_neighbors.extend(get_neighbors(row, col));
            } else if c.ne(&'.') {
                parts.push(Part {
                    symbol: Symbol::Connection(c),
                    neighbors: get_neighbors(row, col),
                });
            }

            if !c.is_digit(10) {
                if let Some(num) = current_number {
                    parts.push(Part {
                        symbol: Symbol::Number(num),
                        neighbors: currrent_neighbors,
                    });
                }

                current_number = None;
                currrent_neighbors = HashSet::new();
            }
        }

        if let Some(num) = current_number {
            parts.push(Part {
                symbol: Symbol::Number(num),
                neighbors: currrent_neighbors,
            });
        }
    }

    parts
}

fn get_neighbors(row: usize, col: usize) -> HashSet<(usize, usize)> {
    HashSet::from([
        // Three numbers on row above
        (row.saturating_add_signed(-1), col.saturating_add_signed(-1)),
        (row.saturating_add_signed(-1), col),
        (row.saturating_add_signed(-1), col.saturating_add_signed(1)),
        // Three numbers on row (including itself)
        (row, col.saturating_add_signed(-1)),
        (row, col),
        (row, col.saturating_add_signed(1)),
        // Three numbers on row below
        (row.saturating_add_signed(1), col.saturating_add_signed(-1)),
        (row.saturating_add_signed(1), col),
        (row.saturating_add_signed(1), col.saturating_add_signed(1)),
    ])
}

pub fn problem_1() {
    let input = fs::read_to_string("input.txt").unwrap();

    let parsed = problem(input);

    println!("{parsed:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        problem(String::from(input));
    }

    #[test]
    fn first() {
        problem_1();
    }
}
