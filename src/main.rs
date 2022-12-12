#[allow(dead_code)]
mod d1;
#[allow(dead_code)]
mod d2;
#[allow(dead_code)]
mod d3;
#[allow(dead_code)]
mod d4;
#[allow(dead_code)]
mod d5;

mod d6;

fn main() {
    let stdin = std::io::stdin();
    let lines: Vec<_> = stdin.lines().map(Result::unwrap).collect();

    use d6::aoc::aoc;
    aoc(&lines);
}
