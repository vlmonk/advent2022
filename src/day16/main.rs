use std::collections::HashSet;

use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Room {
    name: String,
    rate: i32,
    dst: Vec<String>,
}

impl Room {
    pub fn parse(input: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
                    .unwrap();
        }

        let err = || anyhow::anyhow!("Can't parse input: {}", input);

        let caps = RE.captures(input).ok_or_else(err)?;
        let name = caps.get(1).map(|v| v.as_str().to_owned()).ok_or_else(err)?;
        let rate = caps
            .get(2)
            .and_then(|v| v.as_str().parse::<i32>().ok())
            .ok_or_else(err)?;

        let dst = caps
            .get(3)
            .map(|v| {
                v.as_str()
                    .split(", ")
                    .map(|chunk| chunk.to_owned())
                    .collect::<Vec<_>>()
            })
            .ok_or_else(err)?;

        Ok(Self { name, rate, dst })
    }
}

#[derive(Debug)]
struct Map {
    rooms: Vec<Room>,
}

impl Map {
    pub fn parse(input: &str) -> Result<Self> {
        let rooms = input.lines().map(Room::parse).collect::<Result<Vec<_>>>()?;
        Ok(Self { rooms })
    }

    pub fn find(&self, name: &str) -> &Room {
        let founed = self.rooms.iter().find(|el| el.name == name);
        founed.unwrap()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct State {
    current: String,
    open: HashSet<String>,
}

impl State {
    fn new(current: &str) -> Self {
        let current = current.to_owned();
        let open = HashSet::new();

        Self { current, open }
    }

    fn process(&self, action: &Action) -> Self {
        let mut state = self.clone();
        match action {
            Action::Open => {
                state.open.insert(self.current.clone());
            }
            Action::MoveTo(to) => state.current = to.clone(),
        };

        state
    }
}

struct Game {
    map: Map,
    states: Vec<State>,
}

impl Game {
    fn new(map: Map) -> Self {
        let states = vec![State::new("AA")];
        Self { map, states }
    }

    fn tick(&mut self) {
        dbg!(&self.states);
        let mut states = vec![];

        for s in &self.states {
            for action in self.actions(s) {
                let next = s.process(&action);
                let founded = states.iter().find(|current| **current == next);
                if founded.is_none() {
                    states.push(s.process(&action));
                }
            }
        }

        dbg!(&states);
        self.states = states
    }

    fn actions(&self, state: &State) -> Vec<Action> {
        let room = self.map.find(&state.current);
        let mut actions = vec![];

        for dst in &room.dst {
            actions.push(Action::MoveTo(dst.clone()))
        }

        if room.rate > 0 && !state.open.contains(&state.current) {
            actions.push(Action::Open);
        }

        actions
    }
}

enum Action {
    Open,
    MoveTo(String),
}

fn main() -> Result<()> {
    let input = advent2022::read_input()?;
    let map = Map::parse(&input)?;
    let mut game = Game::new(map);
    game.tick();
    game.tick();
    game.tick();
    Ok(())
}
