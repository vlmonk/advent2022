use std::collections::HashMap;

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
    valves: usize,
}

impl Map {
    pub fn parse(input: &str) -> Result<Self> {
        let rooms = input.lines().map(Room::parse).collect::<Result<Vec<_>>>()?;
        let valves = rooms.iter().filter(|r| r.rate > 0).count();
        Ok(Self { rooms, valves })
    }

    pub fn find(&self, name: &str) -> &Room {
        let founed = self.rooms.iter().find(|el| el.name == name);
        founed.unwrap()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct State {
    current: String,
    open: Vec<String>,
    per_min: i32,
    total: i32,
}

impl State {
    fn new(current: &str) -> Self {
        let current = current.to_owned();
        let open = vec![];

        Self {
            current,
            open,
            per_min: 0,
            total: 0,
        }
    }

    fn process(&self, action: &Action, map: &Map) -> Self {
        let room = map.find(&self.current);
        let mut state = self.clone();
        state.total += state.per_min;

        match action {
            Action::Open => {
                state.per_min += room.rate;
                state.open.push(self.current.clone());
                state.open.sort();
            }
            Action::MoveTo(to) => state.current = to.clone(),
            Action::Sleep => {}
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
        // dbg!(&self.states);
        type Key = (String, Vec<String>);
        let mut states: HashMap<Key, State> = HashMap::new();

        for s in &self.states {
            for action in self.actions(s, &self.map) {
                let next = s.process(&action, &self.map);
                let total = next.total;
                let key: Key = (next.current.clone(), next.open.clone());
                let entry = states.entry(key).or_insert(next);
                if total > entry.total {
                    entry.total = total
                }
            }
        }

        // dbg!(&states);
        self.states = states.drain().map(|(_, v)| v).collect();
    }

    fn actions(&self, state: &State, map: &Map) -> Vec<Action> {
        let room = self.map.find(&state.current);
        let mut actions = vec![];

        if state.open.len() == map.valves {
            actions.push(Action::Sleep);
            return actions;
        }

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
    Sleep,
}

fn main() -> Result<()> {
    let input = advent2022::read_input()?;
    let map = Map::parse(&input)?;
    let mut game = Game::new(map);
    for n in 1..=30 {
        game.tick();
        println!("Tick {} - {}", n, game.states.len());
    }

    let a = game.states.iter().map(|s| s.total).max();
    dbg!(a);
    Ok(())
}
