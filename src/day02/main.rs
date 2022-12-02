use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn parse(input: char) -> Result<Self> {
        match input {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(anyhow!("Invalid char: {}", input)),
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

#[derive(Debug)]
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

    pub fn parse(input: char) -> Result<Self> {
        match input {
            'X' => Ok(Outcome::Lost),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Won),
            _ => Err(anyhow!("Invalid char for outcome: {}", input)),
        }
    }
}

#[derive(Debug)]
struct ScoreRound {
    left: Move,
    right: Move,
}

impl ScoreRound {
    pub fn parse(input: &str) -> Result<Self> {
        let left = nth_char(input, 0).and_then(Move::parse)?;
        let right = nth_char(input, 2).and_then(Move::parse)?;

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
struct GuessRound {
    left: Move,
    outcome: Outcome,
}

fn nth_char(input: &str, n: usize) -> Result<char> {
    input
        .chars()
        .nth(n)
        .ok_or_else(|| anyhow!("Char {} not found for input '{}'", n, input))
}

impl GuessRound {
    pub fn parse(input: &str) -> Result<Self> {
        let left = nth_char(input, 0).and_then(Move::parse)?;
        let outcome = nth_char(input, 2).and_then(Outcome::parse)?;

        Ok(Self { left, outcome })
    }

    pub fn score(&self) -> i32 {
        self.right().score() + self.outcome.score()
    }

    fn right(&self) -> Move {
        match (&self.left, &self.outcome) {
            (Move::Rock, Outcome::Lost) => Move::Scissors,
            (Move::Rock, Outcome::Won) => Move::Paper,
            (Move::Paper, Outcome::Lost) => Move::Rock,
            (Move::Paper, Outcome::Won) => Move::Scissors,
            (Move::Scissors, Outcome::Lost) => Move::Paper,
            (Move::Scissors, Outcome::Won) => Move::Rock,
            _ => self.left.clone(),
        }
    }
}

#[derive(Debug)]
struct ScoreGame {
    rounds: Vec<ScoreRound>,
}

impl ScoreGame {
    pub fn parse(input: &str) -> Result<Self> {
        let rounds = input
            .lines()
            .map(|line| ScoreRound::parse(line))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { rounds })
    }

    pub fn score(&self) -> i32 {
        self.rounds.iter().map(|round| round.score()).sum()
    }
}

#[derive(Debug)]
struct GuessGame {
    rounds: Vec<GuessRound>,
}

impl GuessGame {
    pub fn parse(input: &str) -> Result<Self> {
        let rounds = input
            .lines()
            .map(|line| GuessRound::parse(line))
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

    let game = ScoreGame::parse(&data)?;
    println!("Round A: {}", game.score());

    let game = GuessGame::parse(&data)?;
    println!("Round B: {}", game.score());

    Ok(())
}
