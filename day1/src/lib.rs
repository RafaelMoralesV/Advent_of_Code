pub fn number_parser(s: impl Into<String>) -> u32 {
    let s: String = s.into();

    let nums = s
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();

    *nums.first().unwrap_or(&0) * 10 + *nums.last().unwrap_or(&0)
}

pub fn new_number_parser(s: impl Into<String>) -> usize {
    let s: String = s.into();

    let spelled_letters = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut first = (0, s.len() + 1);
    let mut last = (0, 0);
    for (i, digit) in spelled_letters.iter().enumerate() {
        if let Some(index) = s.find(digit) {
            if index < first.1 {
                first = (i, index);
            }
        }

        if let Some(index) = s.rfind(digit) {
            if index >= last.1 {
                last = (i, index);
            }
        }
    }

    for (i, c) in s.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            if i < first.1 {
                first = (digit as usize, i);
            }

            if i >= last.1 {
                last = (digit as usize, i);
            }
        }
    }

    first.0 * 10 + last.0
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn part1_examples() {
        let examples = vec![
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
            ("aksjndfaksjd", 0),
        ];

        let mut sum = 0;
        for example in examples {
            let num = number_parser(example.0);
            sum += num;
            assert_eq!(num, example.1);
        }

        assert_eq!(sum, 142);
    }

    #[test]
    fn part1_real() {
        let mut sum = 0;
        for line in read_to_string("input.txt").unwrap().lines() {
            sum += number_parser(line);
        }

        println!(
            r#"
                 The answer is {sum}!
            "#
        );
    }

    #[test]
    fn part2_examples() {
        let examples = vec![
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("aksjndfaksjd", 0),
            ("7pqrstsixteen", 76),
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
            ("aksjndfaksjd", 0),
            ("eightmqhp79two7eight", 88),
            ("7smbmnrhpk", 77),
        ];

        let mut sum = 0;
        for example in examples {
            let num = new_number_parser(example.0);
            sum += num;
            assert_eq!(
                num, example.1,
                "Parseo encontrado: {} - Esperado: {} \t {}",
                num, example.1, example.0
            );
        }

        assert_eq!(sum, 281 + 142 + 88 + 77);
    }

    #[test]
    fn part2_real() {
        let mut sum = 0;
        for line in read_to_string("input.txt").unwrap().lines() {
            sum += new_number_parser(line);
        }

        println!(
            r#"
                 The answer is {sum}!
            "#
        );
    }
}
