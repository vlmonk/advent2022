use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Blueprint {
    id: u32,
    ore_robot_ore: u32,
    clay_robot_ore: u32,
    obsidian_robor_ore: u32,
    obsidian_robot_clay: u32,
    geode_robot_ore: u32,
    geode_robot_obsidian: u32,
}

impl Blueprint {
    pub fn parse(input: &str) -> Self {
        lazy_static! {
           static ref RE: Regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
        }

        let caps = RE.captures(input).unwrap();
        let id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let ore_robot_ore = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let clay_robot_ore = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let obsidian_robor_ore = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();
        let obsidian_robot_clay = caps.get(5).unwrap().as_str().parse::<u32>().unwrap();
        let geode_robot_ore = caps.get(6).unwrap().as_str().parse::<u32>().unwrap();
        let geode_robot_obsidian = caps.get(7).unwrap().as_str().parse::<u32>().unwrap();

        Self {
            id,
            ore_robot_ore,
            clay_robot_ore,
            obsidian_robor_ore,
            obsidian_robot_clay,
            geode_robot_ore,
            geode_robot_obsidian,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    ore_count: u32,
    clay_count: u32,
    obsidian_count: u32,
    geode_count: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

#[derive(Clone, Copy)]
enum Action {
    Wait,
    BuildOreRobot,
    BuildClayRobot,
    BuildObsidianRobot,
    BuildGeodeRobot,
}

impl Action {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Wait,
            Self::BuildOreRobot,
            Self::BuildClayRobot,
            // Self::BuildObsidianRobot,
        ]
        .into_iter()
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            ore_count: 0,
            clay_count: 0,
            obsidian_count: 0,
            geode_count: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }

    pub fn proces(&self, action: Action, blueprint: &Blueprint) -> Option<Self> {
        let mut state = self.clone();

        let mut add_ore_robots = 0;
        let mut add_clay_robots = 0;
        let mut add_obsidian_robots = 0;
        let mut add_geode_robots = 0;

        match action {
            Action::BuildOreRobot => {
                if state.ore_count >= blueprint.ore_robot_ore {
                    state.ore_count -= blueprint.ore_robot_ore;
                    add_ore_robots += 1;
                } else {
                    return None;
                }
            }
            Action::BuildClayRobot => {
                if state.ore_count >= blueprint.clay_robot_ore {
                    state.ore_count -= blueprint.clay_robot_ore;
                    add_clay_robots += 1;
                }
            }
            Action::BuildObsidianRobot => {
                if state.ore_count >= blueprint.obsidian_robor_ore
                    && state.clay_count >= blueprint.obsidian_robot_clay
                {
                    state.ore_count -= blueprint.obsidian_robor_ore;
                    state.clay_count -= blueprint.obsidian_robot_clay;
                    add_obsidian_robots += 1;
                } else {
                    return None;
                }
            }
            Action::BuildGeodeRobot => {
                if state.ore_count >= blueprint.geode_robot_ore
                    && state.obsidian_count >= blueprint.geode_robot_obsidian
                {
                    state.ore_count -= blueprint.geode_robot_ore;
                    state.obsidian_count -= blueprint.geode_robot_obsidian;
                    add_geode_robots += 1;
                } else {
                    return None;
                }
            }
            Action::Wait => {}
        }

        state.ore_count += state.ore_robots;
        state.clay_count += state.clay_robots;
        state.obsidian_count += state.obsidian_robots;
        state.geode_count += state.geode_robots;

        state.ore_robots += add_ore_robots;
        state.clay_robots += add_clay_robots;
        state.obsidian_robots += add_obsidian_robots;
        state.geode_robots += add_geode_robots;

        Some(state)
    }

    pub fn worse_than(&self, other: &State) -> bool {
        if self == other {
            return false;
        }

        if self.same_robots(other) && self.less_minerals(other) {
            return true;
        }

        if self.same_minerals(other) && self.less_robots(other) {
            return true;
        }

        false
    }

    fn same_robots(&self, other: &State) -> bool {
        self.ore_robots == other.ore_robots
            && self.clay_robots == other.clay_robots
            && self.obsidian_robots == other.obsidian_robots
            && self.geode_robots == other.geode_robots
    }

    fn same_minerals(&self, other: &State) -> bool {
        self.ore_count == other.ore_count
            && self.clay_count == other.clay_count
            && self.obsidian_count == other.obsidian_count
            && self.geode_count == other.geode_count
    }

    fn less_minerals(&self, other: &State) -> bool {
        self.ore_count <= other.ore_count
            && self.clay_count <= other.clay_count
            && self.obsidian_count <= other.obsidian_count
            && self.geode_count <= other.geode_count
    }

    fn less_robots(&self, other: &State) -> bool {
        self.ore_robots <= other.ore_robots
            && self.clay_robots <= other.clay_robots
            && self.obsidian_robots <= other.obsidian_robots
            && self.geode_robots <= other.geode_robots
    }
}

#[derive(Debug)]
struct Game {
    step: usize,
    blueprint: Blueprint,
    states: HashSet<State>,
}

impl Game {
    fn new(blueprint: Blueprint) -> Self {
        let step = 0;
        let state = State::new();

        Self {
            step,
            blueprint,
            states: [state].into(),
        }
    }

    pub fn parse(input: &str) -> Self {
        let blueprint = Blueprint::parse(input);
        Self::new(blueprint)
    }

    pub fn tick(&mut self) {
        let mut next = HashSet::new();

        for state in &self.states {
            if let Some(state) = state.proces(Action::BuildGeodeRobot, &self.blueprint) {
                next.insert(state);
            } else if let Some(state) = state.proces(Action::BuildObsidianRobot, &self.blueprint) {
                next.insert(state);
            } else {
                for action in Action::all() {
                    if let Some(state) = state.proces(action, &self.blueprint) {
                        next.insert(state);
                    }
                }
            }
        }

        self.step += 1;
        self.states = next;

        self.cleanup();
    }

    pub fn cleanup(&mut self) {
        let max_geode_robots = self
            .states
            .iter()
            .map(|state| state.geode_robots)
            .max()
            .unwrap_or(0);

        let mut next = HashSet::new();

        for a in self.states.iter() {
            if a.geode_robots < max_geode_robots {
                continue;
            }

            let better = self
                .states
                .iter()
                .find_map(|b| if a.worse_than(b) { Some(b) } else { None });

            if better.is_none() {
                next.insert(a.clone());
            }
        }

        self.states = next;
    }

    pub fn score_a(&self) -> u32 {
        self.max_geode() * self.blueprint.id
    }

    pub fn score_b(&self) -> u32 {
        self.max_geode()
    }

    fn max_geode(&self) -> u32 {
        self.states
            .iter()
            .map(|state| state.geode_count)
            .max()
            .unwrap_or(0)
    }
}

const GAME_A_COUNT: usize = 24;
const GAME_B_COUNT: usize = 32;

fn main() {
    let raw = advent2022::read_input().unwrap();
    let mut games: Vec<_> = raw.lines().map(Game::parse).collect();

    for _ in 0..GAME_A_COUNT {
        games.iter_mut().for_each(|game| game.tick());
    }

    let result_a: u32 = games.iter().map(|game| game.score_a()).sum();
    println!("Task A: {result_a}");

    let mut games: Vec<_> = games.into_iter().take(3).collect();

    for _ in GAME_A_COUNT..GAME_B_COUNT {
        games.iter_mut().for_each(|game| game.tick());
    }

    let result_b: u32 = games
        .iter()
        .map(|game| game.score_b())
        .reduce(|a, b| a * b)
        .unwrap();

    println!("Task B: {result_b}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blueprint_parse() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        let expected = Blueprint {
            id: 1,
            ore_robot_ore: 4,
            clay_robot_ore: 2,
            obsidian_robor_ore: 3,
            obsidian_robot_clay: 14,
            geode_robot_ore: 2,
            geode_robot_obsidian: 7,
        };

        assert_eq!(Blueprint::parse(&input), expected);
    }
}
