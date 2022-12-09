#[derive(Clone, Copy, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn from_str(c: &str) -> Self {
        match c {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Invalid move"),
        }
    }

    fn lose_to(&self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn win_to(&self) -> Self {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn get_outcome(&self, opponent: &Move) -> Outcome {
        if self == opponent {
            return Outcome::Draw;
        }

        if self.win_to() == *opponent {
            return Outcome::Win;
        }

        Outcome::Lose
    }
}

enum Strategy {
    Win,
    Draw,
    Lose,
}

impl Strategy {
    fn from_move(m: &Move) -> Self {
        match m {
            // X
            Move::Rock => Strategy::Lose,
            // Y
            Move::Paper => Strategy::Draw,
            // Z
            Move::Scissors => Strategy::Win,
        }
    }

    fn get_move(&self, opponent: &Move) -> Move {
        match self {
            Strategy::Win => opponent.lose_to(),
            Strategy::Draw => *opponent,
            Strategy::Lose => opponent.win_to(),
        }
    }

    fn to_outcome(&self) -> Outcome {
        match self {
            Strategy::Win => Outcome::Win,
            Strategy::Draw => Outcome::Draw,
            Strategy::Lose => Outcome::Lose,
        }
    }
}

enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

pub fn aoc(lines: &[String]) {
    let moves: Vec<Vec<Move>> = lines
        .iter()
        .map(|line| line.split(' ').map(Move::from_str).collect())
        .collect();

    p1(&moves);
    p2(&moves);
}

fn p1(moves: &[Vec<Move>]) {
    let mut score = 0;
    for round in moves {
        let [opponent, player] = [round[0], round[1]];
        let outcome = player.get_outcome(&opponent);
        score += outcome as i32 + player as i32;
    }

    println!("{}", score);
}

fn p2(moves: &[Vec<Move>]) {
    let mut score = 0;
    for round in moves {
        let [opponent, field2] = [round[0], round[1]];
        let strategy = Strategy::from_move(&field2);
        let outcome = strategy.to_outcome();
        let m = strategy.get_move(&opponent);

        score += outcome as i32 + m as i32;
    }

    println!("{}", score);
}
