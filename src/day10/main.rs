use anyhow::Result;
use std::collections::{HashSet, VecDeque};

use advent2022::read_input;

#[derive(Debug, Clone)]
enum Cmd {
    Noop,
    Add(i32),
}

impl Cmd {
    pub fn parse(input: &str) -> Result<Self> {
        Cmd::is_noop(input)
            .or_else(|| Cmd::is_add(input))
            .ok_or_else(|| anyhow::anyhow!("Invalid command: {}", input))
    }

    fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Add(_) => 2,
        }
    }

    fn is_noop(input: &str) -> Option<Self> {
        if input == "noop" {
            Some(Self::Noop)
        } else {
            None
        }
    }

    fn is_add(input: &str) -> Option<Self> {
        if &input[..4] == "addx" {
            let value = input[5..].parse::<i32>().ok()?;
            Some(Self::Add(value))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Cpu {
    cmds: VecDeque<Cmd>,
    state: CpuState,
    step: usize,
    reg_x: i32,
}

impl Cpu {
    pub fn parse(input: &str) -> Result<Self> {
        let cmds = input
            .lines()
            .map(Cmd::parse)
            .collect::<Result<VecDeque<_>>>()?;

        let state = CpuState::empty();
        let step = 0;
        let reg_x = 1;

        Ok(Self {
            cmds,
            state,
            step,
            reg_x,
        })
    }
}

#[derive(Debug, Clone)]
enum CpuState {
    Empty,
    Waiting { cmd: Cmd, cycles: usize },
}

impl CpuState {
    pub fn empty() -> Self {
        Self::Empty
    }

    pub fn cmd(input: Cmd) -> Self {
        let cycles = input.cycles();
        Self::Waiting { cmd: input, cycles }
    }

    // pub fn take(&mut self) -> Option<Cmd> {
    //     let mut current = Self::empty();
    //     mem::swap(self, &mut Self::empty());

    //     match current {
    //         Self::Empty => None,
    //         Self::Waiting { ref mut cycles, .. } if *cycles > 1 => {
    //             *cycles -= 1;
    //             None
    //         }
    //         Self::Waiting {
    //             cmd,
    //             ref mut cycles,
    //         } => {
    //             let cycles = *cycles - 1;

    //             if cycles == 0 {
    //                 *self = CpuState::empty();
    //                 Some(cmd)
    //             } else {
    //                 *self = CpuState::Waiting { cmd, cycles };
    //                 None
    //             }
    //         }
    //     }
    // }
}

#[derive(Debug, PartialEq)]
struct CpuInfo {
    step: usize,
    xreg: i32,
}

impl CpuInfo {
    fn new(step: usize, xreg: i32) -> Self {
        Self { step, xreg }
    }
}

impl Iterator for Cpu {
    type Item = CpuInfo;

    fn next(&mut self) -> Option<Self::Item> {
        self.step += 1;

        if let CpuState::Empty = self.state {
            self.state = self
                .cmds
                .pop_front()
                .map(CpuState::cmd)
                .unwrap_or_else(CpuState::empty);
        }

        match self.state {
            CpuState::Empty => None,
            CpuState::Waiting {
                ref mut cmd,
                ref mut cycles,
            } => {
                let info = CpuInfo::new(self.step, self.reg_x);

                *cycles -= 1;

                if *cycles == 0 {
                    // println!("OK process {:?}", &cmd);

                    match cmd {
                        Cmd::Noop => {}
                        Cmd::Add(value) => self.reg_x += *value,
                    }

                    self.state = CpuState::empty();
                }

                Some(info)
            }
        }
    }
}

fn main() -> Result<()> {
    let raw = read_input()?;
    let cpu = Cpu::parse(&raw)?;
    let cpu2 = cpu.clone();

    let key_points: HashSet<usize> = [20, 60, 100, 140, 180, 220].into();
    let list = cpu
        .filter(|i| key_points.contains(&i.step))
        .map(|i| i.step as i32 * i.xreg);

    let result_a: i32 = list.sum();
    println!("Result A: {}\nResult B:", result_a);

    for i in cpu2 {
        let xpos = (i.step as i32 - 1) % 40;
        let diff = (xpos - i.xreg).abs();

        if diff > 1 {
            print!(".");
        } else {
            print!("#");
        }

        if i.step % 40 == 0 {
            print!("\n")
        }
    }

    Ok(())
}
