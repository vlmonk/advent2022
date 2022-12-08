use anyhow::{anyhow, bail, Result};
use lazy_static::lazy_static;
use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    iter,
    rc::Rc,
};

use advent2022::read_input;

#[derive(Debug)]
enum Command {
    ChangeDir(String),
    ChangeDirRoot,
    CHangeDirUp,
    List,
    FileEntry(u64, String),
    DirEntry(String),
}

impl Command {
    pub fn parse(input: &str) -> Result<Self> {
        Self::try_fixed(input)
            .or_else(|| Self::try_dir_entry(input))
            .or_else(|| Self::try_file_entry(input))
            .or_else(|| Self::try_change_dir(input))
            .ok_or_else(|| anyhow::anyhow!("Invalid input: {}", input))
    }

    fn try_fixed(input: &str) -> Option<Self> {
        match input {
            "$ cd /" => Some(Self::ChangeDirRoot),
            "$ cd .." => Some(Self::CHangeDirUp),
            "$ ls" => Some(Self::List),
            _ => None,
        }
    }

    fn try_dir_entry(input: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(r"^dir (\w+)$").unwrap();
        }

        let matched = RE.captures(input)?;
        let capured = matched.get(1)?;
        Some(Self::DirEntry(capured.as_str().to_owned()))
    }

    fn try_file_entry(input: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(r"^(\d+) (\S+)$").unwrap();
        }

        let matched = RE.captures(input)?;
        let size = matched.get(1)?.as_str().parse::<u64>().ok()?;
        let name = matched.get(2)?.as_str().to_owned();
        Some(Self::FileEntry(size, name))
    }

    fn try_change_dir(input: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(r"^\$ cd (\w+)$").unwrap();
        }

        let matched = RE.captures(input)?;
        let name = matched.get(1)?.as_str().to_owned();
        Some(Self::ChangeDir(name))
    }
}

enum DirContent {
    Pending,
    Known {
        dirs: Vec<Rc<RefCell<Dir>>>,
        files: Vec<File>,
    },
}

struct File {
    name: String,
    size: u64,
}

impl File {
    pub fn new(name: String, size: u64) -> Self {
        Self { name, size }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "- {} (file, size={})", self.name, self.size)
    }
}

struct Dir {
    name: String,
    parent: Option<Rc<RefCell<Dir>>>,
    content: DirContent,
}

impl Dir {
    pub fn root() -> Self {
        Self {
            name: "/".into(),
            parent: None,
            content: DirContent::Pending,
        }
    }

    pub fn new(name: String, parent: Rc<RefCell<Dir>>) -> Self {
        Self {
            name,
            parent: Some(parent),
            content: DirContent::Pending,
        }
    }

    pub fn add_dir(&mut self, dir: Dir) {
        match self.content {
            DirContent::Known { ref mut dirs, .. } => dirs.push(Rc::new(RefCell::new(dir))),
            DirContent::Pending => {
                let dirs = vec![Rc::new(RefCell::new(dir))];
                let files = vec![];
                let content = DirContent::Known { dirs, files };
                self.content = content
            }
        }
    }

    pub fn add_file(&mut self, file: File) {
        match self.content {
            DirContent::Known { ref mut files, .. } => files.push(file),
            DirContent::Pending => {
                let dirs = vec![];
                let files = vec![file];
                let content = DirContent::Known { dirs, files };
                self.content = content
            }
        }
    }

    pub fn find_dir(&self, name: &str) -> Result<Rc<RefCell<Dir>>> {
        match self.content {
            DirContent::Pending => bail!("Try to find {} in pending dir", name),
            DirContent::Known { ref dirs, .. } => {
                let founed = dirs.iter().find(|item| item.borrow().name == name).cloned();
                founed.ok_or_else(|| anyhow!("Can't find dir {}", name))
            }
        }
    }

    pub fn print(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.content {
            DirContent::Known {
                ref dirs,
                ref files,
            } => {
                for _ in 0..indent {
                    write!(f, "  ")?;
                }

                write!(f, "{} (dir size={})", self.name, self.size())?;

                for dir in dirs {
                    write!(f, "\n")?;
                    dir.borrow().print(indent + 1, f)?;
                }

                for file in files {
                    write!(f, "\n")?;
                    for _ in 0..indent + 1 {
                        write!(f, "  ")?;
                    }

                    write!(f, "{}", file)?;
                }

                Ok(())
            }
            DirContent::Pending => {
                write!(f, "- {} (dir) (pending)", self.name)
            }
        }
    }

    pub fn size(&self) -> u64 {
        match self.content {
            DirContent::Known {
                ref dirs,
                ref files,
            } => {
                dirs.iter().map(|dir| dir.borrow().size()).sum::<u64>()
                    + files.iter().map(|file| file.size).sum::<u64>()
            }
            DirContent::Pending => 0,
        }
    }

    pub fn dirs(&self) -> impl Iterator<Item = (String, u64)> + '_ {
        let current = (self.name.clone(), self.size());
        let inner = match self.content {
            DirContent::Known { ref dirs, .. } => dirs,
            _ => todo!(),
        };
        let i = inner.iter().map(|item| item.borrow()).collect::<Vec<_>>();
        let j = i
            .into_iter()
            .map(|d| d.dirs().collect::<Vec<_>>())
            .flatten();

        iter::once(current).chain(j.into_iter())
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.print(0, f)
    }
}

struct Filesystem {
    root: Rc<RefCell<Dir>>,
    current: Rc<RefCell<Dir>>,
}

impl Filesystem {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(Dir::root()));

        Self {
            root: root.clone(),
            current: root,
        }
    }

    pub fn process(&mut self, cmd: Command) -> Result<()> {
        dbg!(&cmd);

        match cmd {
            Command::ChangeDirRoot => self.current = self.root.clone(),
            Command::List => {}
            Command::DirEntry(name) => {
                let dir = Dir::new(name, self.current.clone());
                self.current.borrow_mut().add_dir(dir);
            }

            Command::FileEntry(size, name) => {
                let file = File::new(name, size);
                self.current.borrow_mut().add_file(file);
            }
            Command::ChangeDir(name) => {
                let dir = self.current.borrow().find_dir(&name)?;
                self.current = dir;
            }
            Command::CHangeDirUp => {
                let dir = self
                    .current
                    .borrow()
                    .parent
                    .clone()
                    .ok_or_else(|| anyhow!("Parent dir not found!"))?;
                self.current = dir;
            }
        }

        Ok(())
    }

    pub fn dirs<'a>(&'a self) -> impl Iterator<Item = (String, u64)> {
        let x = self.root.clone();
        let y = x.borrow();
        let z = y.dirs().collect::<Vec<_>>();
        z.into_iter()
    }
}

impl Display for Filesystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root.borrow())
    }
}

fn main() -> Result<()> {
    let raw = read_input()?;
    let commands = raw
        .lines()
        .map(Command::parse)
        .collect::<Result<Vec<_>>>()?;

    let mut filesystem = Filesystem::new();

    commands
        .into_iter()
        .map(|cmd| {
            let result = filesystem.process(cmd);
            result
        })
        .collect::<Result<Vec<_>>>()?;

    println!("{}", filesystem);

    filesystem
        .dirs()
        .for_each(|(name, size)| println!("D: {} ({})", name, size));

    let result_a: u64 = filesystem
        .dirs()
        .filter_map(|(_, size)| if size <= 100000 { Some(size) } else { None })
        .sum();

    println!("Result A: {}", result_a);

    Ok(())
}
