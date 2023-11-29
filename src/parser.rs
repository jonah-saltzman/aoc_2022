#[derive(Debug)]
pub enum CdTarget {
    Root,
    Parent,
    Named(String),
}

#[derive(Debug)]
pub enum Command {
    Ls,
    Cd(CdTarget),
}

#[derive(Debug)]
pub struct LsFile {
    pub size: usize,
}

#[derive(Debug)]
pub struct LsFolder {
    pub name: String,
}

#[derive(Debug)]
pub enum LsLine {
    File(LsFile),
    Folder(LsFolder),
}

#[derive(Debug)]
pub struct LsOutput(Vec<LsLine>);

impl IntoIterator for LsOutput {
    type Item = LsLine;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug)]
pub enum Group {
    Input(Command),
    Output(LsOutput),
}

#[derive(Debug, Default)]
enum ParserState {
    #[default]
    Input,
    Output(LsOutput),
}

pub struct Parser {
    state: Option<ParserState>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            state: Some(ParserState::Input),
        }
    }

    pub fn line(&mut self, line: &str) -> Vec<Group> {
        let tokens = line.split(' ');
        match self.state.take().unwrap() {
            ParserState::Input => {
                let command = Parser::parse_command(tokens.skip(1));
                self.state = match command {
                    Command::Ls => Some(ParserState::Output(LsOutput(vec![]))),
                    Command::Cd(_) => Some(ParserState::Input),
                };
                vec![Group::Input(command)]
            }
            ParserState::Output(output) => match (output, line.starts_with('$')) {
                (lines, true) => {
                    let command = Parser::parse_command(tokens.skip(1));
                    self.state = match command {
                        Command::Ls => Some(ParserState::Output(LsOutput(vec![]))),
                        Command::Cd(_) => Some(ParserState::Input),
                    };
                    let command = Group::Input(command);
                    let ls_output = Group::Output(lines);
                    vec![ls_output, command]
                }
                (mut lines, false) => {
                    let new_ls_line = Parser::parse_lsline(tokens);
                    lines.0.push(new_ls_line);
                    self.state = Some(ParserState::Output(lines));
                    vec![]
                }
            },
        }
    }

    pub fn end(&mut self) -> Option<Group> {
        match self.state.take().unwrap() {
            ParserState::Input => None,
            ParserState::Output(lines) => Some(Group::Output(lines)),
        }
    }

    fn parse_command<'a>(mut tokens: impl Iterator<Item = &'a str>) -> Command {
        match tokens.next().unwrap() {
            "ls" => Command::Ls,
            "cd" => {
                let target = tokens.next().unwrap();
                let target: CdTarget = match target {
                    "/" => CdTarget::Root,
                    ".." => CdTarget::Parent,
                    dir => CdTarget::Named(dir.to_owned()),
                };
                Command::Cd(target)
            }
            _ => unreachable!(),
        }
    }

    fn parse_lsline<'a>(mut tokens: impl Iterator<Item = &'a str>) -> LsLine {
        match tokens.next().unwrap() {
            "dir" => {
                let dir_name = tokens.next().unwrap();
                LsLine::Folder(LsFolder {
                    name: dir_name.to_owned(),
                })
            }
            bytes => {
                let size: usize = bytes.parse().unwrap();
                LsLine::File(LsFile { size })
            }
        }
    }
}
