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

pub mod p1 {
    use std::collections::HashMap;

    use super::*;

    pub fn solve(input: &str) -> usize {
        let lines = input.lines().map(|l| Line::parse_borrowed(l).unwrap());
        /*
        for line in lines.clone() {
            println!("{:?}", line);
        }
        */
        let mut cwd = Vec::<&str>::new();
        let mut sizes = HashMap::<String, usize>::new();
        for l in lines {
            match l {
                Line::Command(c) => {
                    match c {
                        Command::Cd(dirname) => {
                            if dirname == "/" {
                                cwd.clear();
                            } else if dirname == ".." {
                                cwd.pop();
                            } else {
                                cwd.push(dirname);
                            }
                        }
                        Command::Ls => {
                            // ignore
                        }
                    }
                }
                Line::Output(o) => {
                    match o {
                        Output::DirStat(d) => {
                            // ignore
                        }
                        Output::FileStat(f) => {
                            for i in 0..=cwd.len() {
                                let path = cwd[0..cwd.len() - i].join("/");
                                *sizes.entry(path).or_default() += f.size;
                            }
                        }
                    }
                }
            }
        }
        sizes.values().copied().filter(|sz| *sz <= 100_000).sum()
    }

    #[test]
    fn test_solve() {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
        assert_eq!(solve(input), 95437);
    }
}
