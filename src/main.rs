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
#[allow(dead_code)]
mod d6;
#[allow(dead_code)]
mod d7;
#[allow(dead_code)]
mod d8;
#[allow(dead_code)]
mod d9;

// Skips needed since rustfmt does not support natural sort
#[rustfmt::skip]
mod d10;

fn main() {
    let stdin = std::io::stdin();
    let lines: Vec<_> = stdin.lines().map(Result::unwrap).collect();

    use d10::aoc::aoc;
    aoc(&lines);
}
