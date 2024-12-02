pub trait AoC {
    fn parse() -> String;

    fn puzzle_one(s: String) -> u64;

    fn puzzle_two(s: String) -> u64;
}
