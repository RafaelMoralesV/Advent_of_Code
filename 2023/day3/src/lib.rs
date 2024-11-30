#[derive(Debug)]
pub enum Symbol {
    Nothing,
    Connection(char),
    Number(usize),
}

#[derive(Debug)]
pub struct Point {
    pub row: usize,
    pub col: usize,
    pub s: Symbol,
}

pub fn problem(s: String) {
    let _points = s
        .lines()
        .into_iter()
        .enumerate()
        .map(|(row, line)| {
            line.char_indices()
                .map(move |ci| Point {
                    row,
                    col: ci.0,
                    s: match ci.1 {
                        '.' => Symbol::Nothing,
                        token if token.is_numeric() => Symbol::Number(
                            token.to_digit(10).expect("Character is a number") as usize,
                        ),
                        _ => Symbol::Connection(ci.1),
                    },
                })
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<_>>();

    println!("{_points:?}");
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
}
