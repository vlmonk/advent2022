use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use anyhow::Result;

#[derive(Debug)]
enum Step {
    Up(usize),
    Right(usize),
    Down(usize),
    Left(usize),
}

impl Step {
    pub fn parse(input: &str) -> Result<Self> {
        let value = input[2..].parse::<usize>()?;
        match &input[..1] {
            "U" => Ok(Self::Up(value)),
            "R" => Ok(Self::Right(value)),
            "D" => Ok(Self::Down(value)),
            "L" => Ok(Self::Left(value)),
            _ => anyhow::bail!("Invalid command: {}", &input[..1]),
        }
    }

    pub fn zero(&self) -> bool {
        use Step::*;

        match self {
            Up(0) | Right(0) | Down(0) | Left(0) => true,
            _ => false,
        }
    }

    pub fn dec(&mut self) {
        use Step::*;

        match self {
            Up(n) if *n > 0 => *self = Up(*n - 1),
            Right(n) if *n > 0 => *self = Right(*n - 1),
            Down(n) if *n > 0 => *self = Down(*n - 1),
            Left(n) if *n > 0 => *self = Left(*n - 1),
            _ => panic!("Invalid dec"),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn follow(&mut self, head: &Position) {
        let dx = head.x - self.x;
        let dy = head.y - self.y;

        match (dx, dy) {
            (-1..=1, -1..=1) => {}

            (dx, dy) => {
                self.x += dx.signum();
                self.y += dy.signum();
            }
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

struct Game {
    head: Position,
    tail: Position,

    current: Option<Step>,
    steps: VecDeque<Step>,
}

impl Game {
    pub fn new<T>(steps: T) -> Self
    where
        T: Into<VecDeque<Step>>,
    {
        let mut steps = steps.into();
        let head = Position::new(0, 0);
        let tail = Position::new(0, 0);
        let current = steps.pop_front();

        Self {
            steps,
            head,
            tail,
            current,
        }
    }
}

impl Iterator for Game {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref v) = self.current {
            if v.zero() {
                self.current = self.steps.pop_front();
            }
        }

        match self.current {
            Some(ref mut step) => {
                match step {
                    Step::Up(_) => self.head.y += 1,
                    Step::Right(_) => self.head.x += 1,
                    Step::Down(_) => self.head.y -= 1,
                    Step::Left(_) => self.head.x -= 1,
                }

                step.dec()
            }
            None => return None,
        }

        self.tail.follow(&self.head);

        Some(self.tail.clone())
    }
}

fn main() -> Result<()> {
    let raw = advent2022::read_input()?;
    let cmds = raw.lines().map(Step::parse).collect::<Result<Vec<_>>>()?;
    let game = Game::new(cmds);
    let mut points: HashSet<Position> = HashSet::new();

    for tail in game {
        points.insert(tail);
    }

    dbg!(points.len());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn follow_1() {
        let h = Position::new(0, 0);
        let mut t = Position::new(0, 0);

        t.follow(&h);
        assert_eq!(t, Position::new(0, 0));
    }

    #[test]
    fn follow_2() {
        let h = Position::new(0, 2);
        let mut t = Position::new(0, 0);

        t.follow(&h);
        assert_eq!(t, Position::new(0, 1));
    }

    #[test]
    fn follow_3() {
        let h = Position::new(10, 5);
        let mut t = Position::new(8, 5);

        t.follow(&h);
        assert_eq!(t, Position::new(9, 5));
    }

    #[test]
    fn follow_4() {
        let h = Position::new(2, 1);
        let mut t = Position::new(0, 0);

        t.follow(&h);
        assert_eq!(t, Position::new(1, 1));
    }
}
