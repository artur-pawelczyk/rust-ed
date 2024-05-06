use std::{collections::HashMap, fmt::{Debug, Formatter}};

use crate::{commands::noop, editor::{CommandContext, CommandError, Editor, EditorFn, LineOffset}};

pub struct CommandMap {
    map: HashMap<char, InnerCommand>,
    number: InnerCommand,
    noop: InnerCommand,
}

impl Default for CommandMap {
    fn default() -> Self {
        Self {
            map: Default::default(),
            number: Default::default(),
            noop: InnerCommand { f: Box::new(noop), name: Box::from("noop") },
        }
    }
}

struct InnerCommand {
    f: Box<dyn EditorFn>,
    name: Box<str>,
}

pub struct Command<'a> {
    f: &'a dyn EditorFn,
    name: &'a str,
    line: LineOffset,
}

impl Debug for Command<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Command {{ name: {} line: {:?} }}", self.name, self.line)
    }
}

impl Default for InnerCommand {
    fn default() -> Self {
        Self { f: Box::new(noop), name: Box::from("") }
    }
}

impl Command<'_> {
    pub fn run(&self, ed: &mut Editor) -> Result<(), CommandError> {
        let mut out = std::io::stdout();
        let mut ctx = CommandContext::with_output(&mut out).line_offset(&self.line);
        self.f.apply(ed, &mut ctx)
    }
}

impl CommandMap {
    pub fn lookup(&self, s: &str) -> Option<Command> {
        let s = s.trim();
        if s.is_empty() {
            let cmd = &self.noop;
            Some(Command { f: cmd.f.as_ref(), name: cmd.name.as_ref(), line: LineOffset::default() })
        } else if let Ok(line) = s.parse::<LineOffset>() {
            let cmd = &self.number;
            Some(Command { f: cmd.f.as_ref(), name: cmd.name.as_ref(), line })
        } else {
            let cmd = s.chars().next().and_then(|c| self.map.get(&c))?;
            Some(Command { f: cmd.f.as_ref(), name: cmd.name.as_ref(), line: LineOffset::default() })
        }
    }

    pub fn bind(&mut self, short: &str, name: &str, f: impl EditorFn + 'static) {
        if let Some(char) = short.chars().next() {
            self.map.insert(char, InnerCommand{ f: Box::new(f), name: Box::from(name) });
        }
    }

    pub fn bind_number(&mut self, name: &str, f: impl EditorFn + 'static) {
        self.number = InnerCommand { name: Box::from(name), f: Box::new(f) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_command() {
        let mut map = CommandMap::default();
        map.bind("a", "append", noop);
        let cmd = map.lookup("a").unwrap();
        assert_eq!(cmd.name, "append");
    }

    #[test]
    fn test_line_command() {
        let mut map = CommandMap::default();
        map.bind_number("goto-line", noop);

        let cmd = map.lookup("123").unwrap();
        assert_eq!(cmd.name, "goto-line");
        assert_eq!(cmd.line, LineOffset::Absolute(123));

        let cmd = map.lookup("+").unwrap();
        assert_eq!(cmd.name, "goto-line");
        assert_eq!(cmd.line, LineOffset::Relative(1));
    }

    #[test]
    fn test_noop_command() {
        let map = CommandMap::default();
        let cmd = map.lookup("").unwrap();
        assert_eq!(cmd.name, "noop");
    }
}

