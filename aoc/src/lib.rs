pub trait AoC {
    fn parse(input: String) -> Self;

    fn puzzle_one(&self) -> u64;

    fn puzzle_two(&self) -> u64;
}
