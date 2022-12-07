use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum Line<'a> {
    Command(Command<'a>),
    Output(Output<'a>),
}

impl<'a> Line<'a> {
    fn parse_borrowed(line: &'a str) -> Option<Self> {
        if let Some(c) = Command::parse_borrowed(line) {
            Some(Line::Command(c))
        } else if let Some(o) = Output::parse_borrowed(line) {
            Some(Line::Output(o))
        } else {
            None
        }
    }
}

#[test]
fn test_parse_line() {
    assert_eq!(
        Line::parse_borrowed("232179 vqqcvgts.vrc"),
        Some(Line::Output(Output::FileStat(FileStat {
            name: "vqqcvgts.vrc",
            size: 232179
        })))
    );
    assert_eq!(
        Line::parse_borrowed("dir bshmsns"),
        Some(Line::Output(Output::DirStat(DirStat { name: "bshmsns" })))
    );
    assert_eq!(
        Line::parse_borrowed("$ cd /"),
        Some(Line::Command(Command::Cd("/")))
    );
    assert_eq!(
        Line::parse_borrowed("$ ls"),
        Some(Line::Command(Command::Ls))
    );
}

#[derive(Debug, Eq, PartialEq)]
enum Command<'a> {
    Cd(&'a str),
    Ls,
}

impl<'a> Command<'a> {
    fn parse_borrowed(line: &'a str) -> Option<Self> {
        let mut parts = line.split(' ');
        let first = parts.next()?;
        if first != "$" {
            return None;
        }
        let second = parts.next()?;
        if second == "cd" {
            let dirname = parts.next()?;
            if parts.next().is_some() {
                return None;
            }
            return Some(Command::Cd(dirname));
        }
        if second == "ls" {
            if parts.next().is_some() {
                return None;
            }
            return Some(Command::Ls);
        }
        None
    }
}

#[test]
fn test_parse_command() {
    assert_eq!(Command::parse_borrowed("232179 vqqcvgts.vrc"), None);
    assert_eq!(Command::parse_borrowed("dir bshmsns"), None);
    assert_eq!(Command::parse_borrowed("$ cd /"), Some(Command::Cd("/")));
    assert_eq!(Command::parse_borrowed("$ ls"), Some(Command::Ls));
}

#[derive(Debug, Eq, PartialEq)]
enum Output<'a> {
    DirStat(DirStat<'a>),
    FileStat(FileStat<'a>),
}

impl<'a> Output<'a> {
    fn parse_borrowed(line: &'a str) -> Option<Self> {
        if let Some(d) = DirStat::parse_borrowed(line) {
            Some(Output::DirStat(d))
        } else if let Some(f) = FileStat::parse_borrowed(line) {
            Some(Output::FileStat(f))
        } else {
            None
        }
    }
}

#[test]
fn test_parse_output() {
    assert_eq!(
        Output::parse_borrowed("232179 vqqcvgts.vrc"),
        Some(Output::FileStat(FileStat {
            name: "vqqcvgts.vrc",
            size: 232179
        }))
    );
    assert_eq!(
        Output::parse_borrowed("dir bshmsns"),
        Some(Output::DirStat(DirStat { name: "bshmsns" }))
    );
}

#[derive(Debug, Eq, PartialEq)]
struct DirStat<'a> {
    name: &'a str,
}

impl<'a> DirStat<'a> {
    fn parse_borrowed(line: &'a str) -> Option<Self> {
        let mut parts = line.split(' ');
        let dir_token = parts.next()?;
        if dir_token != "dir" {
            return None;
        }
        let name = parts.next()?;
        if parts.next().is_some() {
            return None;
        }
        Some(DirStat { name })
    }
}

#[test]
fn test_parse_dirstat() {
    assert_eq!(DirStat::parse_borrowed("232179 vqqcvgts.vrc"), None);
    assert_eq!(
        DirStat::parse_borrowed("dir bshmsns"),
        Some(DirStat { name: "bshmsns" })
    );
}

#[derive(Debug, Eq, PartialEq)]
struct FileStat<'a> {
    name: &'a str,
    size: usize,
}

impl<'a> FileStat<'a> {
    fn parse_borrowed(line: &'a str) -> Option<Self> {
        let mut parts = line.split(' ');
        let size_str = parts.next()?;
        let name = parts.next()?;
        if parts.next().is_some() {
            return None;
        }
        let size: usize = size_str.parse().ok()?;
        Some(FileStat { name, size })
    }
}

#[test]
fn test_parse_filestat() {
    assert_eq!(
        FileStat::parse_borrowed("232179 vqqcvgts.vrc"),
        Some(FileStat {
            name: "vqqcvgts.vrc",
            size: 232179
        })
    );
    assert_eq!(FileStat::parse_borrowed("dir bshmsns"), None);
}

pub mod p1 {}
