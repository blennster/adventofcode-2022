use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("unknown move".to_string()),
        }
    }
}

impl Move {
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

impl From<Move> for Strategy {
    fn from(m: Move) -> Self {
        match m {
            // X
            Move::Rock => Strategy::Lose,
            // Y
            Move::Paper => Strategy::Draw,
            // Z
            Move::Scissors => Strategy::Win,
        }
    }
}

impl Strategy {
    fn get_move(&self, opponent: &Move) -> Move {
        match self {
            Strategy::Win => opponent.lose_to(),
            Strategy::Draw => *opponent,
            Strategy::Lose => opponent.win_to(),
        }
    }
}

enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl From<Strategy> for Outcome {
    fn from(strategy: Strategy) -> Self {
        match strategy {
            Strategy::Win => Outcome::Win,
            Strategy::Draw => Outcome::Draw,
            Strategy::Lose => Outcome::Lose,
        }
    }
}

pub fn aoc(lines: &[String]) {
    let rounds: Vec<Vec<Move>> = lines
        .iter()
        .map(|line| {
            line.split(' ')
                .map(|x| Move::from_str(x).unwrap())
                .collect()
        })
        .collect();

    p1(&rounds);
    p2(&rounds);
}

fn p1(rounds: &[Vec<Move>]) {
    let mut score = 0;
    for round in rounds {
        let [opponent, player] = [round[0], round[1]];
        let outcome = player.get_outcome(&opponent);
        score += outcome as i32 + player as i32;
    }

    println!("{}", score);
}

fn p2(rounds: &[Vec<Move>]) {
    let mut score = 0;
    for round in rounds {
        let opponent = round[0];
        let strategy: Strategy = round[1].into();
        let m = strategy.get_move(&opponent);
        let outcome: Outcome = strategy.into();

        score += outcome as i32 + m as i32;
    }

    println!("{}", score);
}
