use anyhow::{anyhow, Result};

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn left(input: &str) -> Result<Self> {
        let left = input
            .chars()
            .nth(0)
            .ok_or_else(|| anyhow!("Missing left char"))?;

        match left {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            _ => Err(anyhow!("Invalid char: {}", left)),
        }
    }

    pub fn right(input: &str) -> Result<Self> {
        let right = input
            .chars()
            .nth(2)
            .ok_or_else(|| anyhow!("Missing right char"))?;

        match right {
            'X' => Ok(Move::Rock),
            'Y' => Ok(Move::Paper),
            'Z' => Ok(Move::Scissors),
            _ => Err(anyhow!("Invalid char: {}", right)),
        }
    }

    pub fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

enum Outcome {
    Lost,
    Draw,
    Won,
}

impl Outcome {
    pub fn score(&self) -> i32 {
        match self {
            Self::Lost => 0,
            Self::Draw => 3,
            Self::Won => 6,
        }
    }
}

#[derive(Debug)]
struct Round {
    left: Move,
    right: Move,
}

impl Round {
    pub fn parse(input: &str) -> Result<Self> {
        let left = Move::left(input)?;
        let right = Move::right(input)?;

        Ok(Self { left, right })
    }

    pub fn score(&self) -> i32 {
        self.right.score() + self.outcome().score()
    }

    pub fn outcome(&self) -> Outcome {
        match (&self.left, &self.right) {
            (Move::Rock, Move::Paper) => Outcome::Won,
            (Move::Rock, Move::Scissors) => Outcome::Lost,
            (Move::Paper, Move::Rock) => Outcome::Lost,
            (Move::Paper, Move::Scissors) => Outcome::Won,
            (Move::Scissors, Move::Rock) => Outcome::Won,
            (Move::Scissors, Move::Paper) => Outcome::Lost,
            _ => Outcome::Draw,
        }
    }
}

#[derive(Debug)]

struct Game {
    rounds: Vec<Round>,
}

impl Game {
    pub fn parse(input: &str) -> Result<Self> {
        let rounds = input
            .lines()
            .map(|line| Round::parse(line))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { rounds })
    }

    pub fn score(&self) -> i32 {
        self.rounds.iter().map(|round| round.score()).sum()
    }
}

fn main() -> Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("missing input filename"))?;

    let data = std::fs::read_to_string(&filename)?;
    let game = Game::parse(&data)?;
    println!("Score: {}", game.score());

    Ok(())
}
