#[allow(dead_code)]
mod d1;

#[allow(dead_code)]
mod d2;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lines().map(Result::unwrap).collect();

    use d2::aoc::aoc;
    aoc(&lines);
}
