use aoc::AoC;
use std::fs;

#[derive(Debug)]
struct Day5 {
    before_rule: Vec<Vec<u8>>,

    page_updates: Vec<Vec<usize>>,
}

enum RuleStatus {
    Valid,
    Invalid,
}

fn check_update(update: &Vec<usize>, rules: &Vec<Vec<u8>>) -> RuleStatus {
    for (i, page) in update.iter().enumerate() {
        for other in update.iter().skip(i + 1) {
            if rules
                .get(*other)
                .is_some_and(|rules| rules.contains(&(*page as u8)))
            {
                return RuleStatus::Invalid;
            }
        }
    }

    RuleStatus::Valid
}

impl AoC for Day5 {
    fn parse(input: String) -> Self {
        let (rules, updates) = input.split_once("\n\n").unwrap();
        let mut before_rule: Vec<Vec<u8>> = vec![Vec::new(); u8::max_value().into()];

        rules
            .lines()
            .map(|line| line.split_once("|").unwrap())
            .map(|(l, r)| (l.parse::<u8>().unwrap(), r.parse::<u8>().unwrap()))
            .for_each(|(l, r)| before_rule.get_mut::<usize>(l.into()).unwrap().push(r));

        let page_updates = updates
            .lines()
            .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
            .collect();

        Self {
            before_rule,
            page_updates,
        }
    }

    fn puzzle_one(&self) -> u64 {
        self.page_updates
            .iter()
            .filter(|&update| match check_update(update, &self.before_rule) {
                RuleStatus::Valid => true,
                RuleStatus::Invalid => false,
            })
            .map(|value| value[value.len() / 2])
            .sum::<usize>()
            .try_into()
            .unwrap()
    }

    fn puzzle_two(&self) -> u64 {
        todo!()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file is present and intact");
    let day5 = Day5::parse(input);

    print!("\n\tpuzzle one: > {}\n", day5.puzzle_one());

    print!("\n\tpuzzle two: > {}\n", day5.puzzle_two());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_one_example() {
        let input =
            fs::read_to_string("example_one.txt").expect("Input file is present and intact");

        let day5 = Day5::parse(input);

        assert_eq!(143, day5.puzzle_one());
    }
}
