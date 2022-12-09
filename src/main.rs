#[allow(dead_code)]
mod d1;

#[allow(dead_code)]
mod d2;

#[allow(dead_code)]
mod d3;

mod d4;

fn main() {
    let stdin = std::io::stdin();
    let lines: Vec<_> = stdin.lines().map(Result::unwrap).collect();

    use d4::aoc::aoc;
    aoc(&lines);
}
