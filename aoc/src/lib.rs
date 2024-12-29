pub trait AoC {
    fn parse(input: String) -> Self;

    fn puzzle_one(&mut self) -> u64;

    fn puzzle_two(&mut self) -> u64;
}
