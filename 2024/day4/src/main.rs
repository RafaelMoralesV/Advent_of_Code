use aoc::AoC;
use std::fs;

#[derive(Debug)]
struct Day4 {
    letters: Vec<Vec<char>>,
}

fn check_xmas(letters: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let m_possitions: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
    ];

    let a_possitions = m_possitions
        .iter()
        .map(|(x, y)| (x * 2, y * 2))
        .collect::<Vec<_>>();
    let s_possitions = m_possitions
        .iter()
        .map(|(x, y)| (x * 3, y * 3))
        .collect::<Vec<_>>();

    let coord_batch = m_possitions
        .iter()
        .zip(a_possitions.iter())
        .zip(s_possitions.iter())
        .map(|((x, y), z)| (x, y, z));

    let mut found = 0;

    for (m, a, s) in coord_batch {
        if check_char(&letters, row, col, m, 'M').is_some_and(|v| v)
            && check_char(&letters, row, col, a, 'A').is_some_and(|v| v)
            && check_char(&letters, row, col, s, 'S').is_some_and(|v| v)
        {
            found += 1;
        }
    }

    found
}

fn check_char(
    letters: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    coords: &(isize, isize),
    ch: char,
) -> Option<bool> {
    let row: isize = row.try_into().ok()?;
    let col: isize = col.try_into().ok()?;

    let x: usize = (row + coords.0).try_into().ok()?;
    let y: usize = (col + coords.1).try_into().ok()?;

    Some(
        letters
            .get(x)
            .and_then(|r| r.get(y))
            .is_some_and(|c| *c == ch),
    )
}

impl AoC for Day4 {
    fn parse(input: String) -> Self {
        let letters = input.lines().map(|line| line.chars().collect()).collect();

        Self { letters }
    }

    fn puzzle_one(&mut self) -> u64 {
        let mut count = 0;
        for (row, line) in self.letters.iter().enumerate() {
            for (col, letter) in line.iter().enumerate() {
                if *letter == 'X' {
                    count += check_xmas(&self.letters, row, col);
                }
            }
        }

        count as u64
    }

    fn puzzle_two(&mut self) -> u64 {
        let mut count = 0;

        let x_mases: Vec<Vec<Vec<char>>> = vec![
            vec![
                "M*S".chars().collect(),
                "*A*".chars().collect(),
                "M*S".chars().collect(),
            ],
            vec![
                "M*M".chars().collect(),
                "*A*".chars().collect(),
                "S*S".chars().collect(),
            ],
            vec![
                "S*S".chars().collect(),
                "*A*".chars().collect(),
                "M*M".chars().collect(),
            ],
            vec![
                "S*M".chars().collect(),
                "*A*".chars().collect(),
                "S*M".chars().collect(),
            ],
        ];

        for (row, line) in self.letters.iter().enumerate() {
            for (col, letter) in line.iter().enumerate() {
                for x_mas in x_mases.iter() {
                    if *letter == x_mas[0][0]
                        && self
                            .letters
                            .get(row + 2)
                            .and_then(|r| r.get(col))
                            .is_some_and(|c| *c == x_mas[2][0])
                        && self
                            .letters
                            .get(row + 1)
                            .and_then(|r| r.get(col + 1))
                            .is_some_and(|c| *c == x_mas[1][1])
                        && self
                            .letters
                            .get(row)
                            .and_then(|r| r.get(col + 2))
                            .is_some_and(|c| *c == x_mas[0][2])
                        && self
                            .letters
                            .get(row + 2)
                            .and_then(|r| r.get(col + 2))
                            .is_some_and(|c| *c == x_mas[2][2])
                    {
                        count += 1;
                    }
                }
            }
        }

        count as u64
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file is present and intact");
    let mut day3 = Day4::parse(input);

    print!("\n\tpuzzle one: > {}\n", day3.puzzle_one());

    print!("\n\tpuzzle two: > {}\n", day3.puzzle_two());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_one_example() {
        let input = String::from(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );

        let mut day4 = Day4::parse(input);

        assert_eq!(18, day4.puzzle_one());
    }

    #[test]
    fn puzzle_two_example() {
        let input = String::from(
            ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........",
        );

        let mut day4 = Day4::parse(input);

        assert_eq!(9, day4.puzzle_two());
    }
}
