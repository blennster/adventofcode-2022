#[allow(dead_code)]
mod d1;

#[allow(dead_code)]
mod d2;

#[allow(dead_code)]
mod d3;

#[allow(dead_code)]
mod d4;

mod d5;

fn main() {
    let stdin = std::io::stdin();
    let lines: Vec<_> = stdin.lines().map(Result::unwrap).collect();

    use d5::aoc::aoc;
    aoc(&lines);
}
