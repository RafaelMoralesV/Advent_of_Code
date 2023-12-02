pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Set {
    pub red: Option<usize>,
    pub blue: Option<usize>,
    pub green: Option<usize>,
}

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    pub is_possible: bool,
    pub sets: Vec<Set>,
}

impl Set {
    pub fn is_possible(self: &Self, red: usize, blue: usize, green: usize) -> bool {
        if let Some(set_red) = self.red {
            if set_red > red {
                return false;
            }
        }

        if let Some(set_blue) = self.blue {
            if set_blue > blue {
                return false;
            }
        }

        if let Some(set_green) = self.green {
            if set_green > green {
                return false;
            }
        }

        true
    }

    pub fn parse_set(s: impl Into<String>) -> Self {
        let s: String = s.into();

        let mut red = None;
        let mut blue = None;
        let mut green = None;

        for cubes in s.trim().replace(";", "").split(",") {
            let (number, color) = cubes.trim().split_once(" ").unwrap();

            let number = String::from(number).parse().unwrap();

            match color {
                "red" => red = Some(number),
                "blue" => blue = Some(number),
                "green" => green = Some(number),
                _ => unreachable!(),
            }
        }

        Set { red, blue, green }
    }
}

impl Game {
    pub fn parse_game(s: impl Into<String>, limits: (usize, usize, usize)) -> Self {
        let line: String = s.into();

        let (id, sets) = line.split_once(":").unwrap();

        let (_, id) = id.trim().split_once(" ").unwrap();

        let mut game = Game {
            id: String::from(id).parse().unwrap(),
            is_possible: true,
            sets: Vec::new(),
        };

        for set in sets.split(";") {
            let set = Set::parse_set(set);

            if !set.is_possible(limits.0, limits.1, limits.2) {
                game.is_possible = false;
            }

            game.sets.push(set)
        }

        game
    }

    pub fn power(self: &Self) -> usize {
        let mut red = None;
        let mut green = None;
        let mut blue = None;

        for set in &self.sets {
            red = red.max(set.red);
            green = green.max(set.green);
            blue = blue.max(set.blue);
        }

        red.unwrap_or(1) * green.unwrap_or(1) * blue.unwrap_or(1)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn part1_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let mut games: Vec<Game> = Vec::new();

        for line in input.lines() {
            games.push(Game::parse_game(line, (12, 14, 13)));
        }

        let mut sum = 0;
        for game in games {
            if game.is_possible {
                sum += game.id;
            }
        }

        assert_eq!(sum, 8)
    }

    #[test]
    fn part1_real() {
        let mut sum = 0;
        for line in read_to_string("input.txt").unwrap().lines() {
            let game = Game::parse_game(line, (12, 14, 13));

            if game.is_possible {
                sum += game.id;
            }
        }

        println!(
            r#"
            La suma de juegos posibles es de {sum} !
            "#
        )
    }

    #[test]
    fn part2_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let mut powers: Vec<(Game, usize)> = Vec::new();
        for line in input.lines() {
            let game = Game::parse_game(line, (0, 0, 0));
            let power = game.power();

            powers.push((game, power))
        }

        assert_eq!(powers.pop().unwrap().1, 36);
        assert_eq!(powers.pop().unwrap().1, 630);
        assert_eq!(powers.pop().unwrap().1, 1560);
        assert_eq!(powers.pop().unwrap().1, 12);
        assert_eq!(powers.pop().unwrap().1, 48);
    }

    #[test]
    fn part2_real() {
        let mut sum = 0;
        for line in read_to_string("input.txt").unwrap().lines() {
            let game = Game::parse_game(line, (12, 14, 13));

            sum += game.power();
        }

        println!(
            r#"
            La suma de poderes es de {sum} !
            "#
        )
    }
}
