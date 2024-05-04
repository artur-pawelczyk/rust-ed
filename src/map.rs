use std::collections::HashMap;

use crate::{commands::noop, editor::{CommandContext, CommandError, Editor, EditorFn}};

pub struct CommandMap {
    map: HashMap<char, Command>,
    number: Command,
    noop: Command,
}

impl Default for CommandMap {
    fn default() -> Self {
        Self {
            map: Default::default(),
            number: Default::default(),
            noop: Command { f: Box::new(noop), name: Box::from("noop") },
        }
    }
}

pub struct Command {
    f: Box<dyn EditorFn>,
    name: Box<str>,
}

impl Default for Command {
    fn default() -> Self {
        Self { f: Box::new(noop), name: Box::from("") }
    }
}

impl Command {
    pub fn run(&self, ed: &mut Editor) -> Result<(), CommandError> {
        let mut out = std::io::stdout();
        self.f.apply(ed, &mut CommandContext::with_output(&mut out))
    }
}

impl CommandMap {
    pub fn lookup(&self, s: &str) -> Option<&Command> {
        if s.trim().is_empty() {
            Some(&self.noop)
        } else if let Ok(_) = s.parse::<usize>() {
            Some(&self.number)
        } else {
            s.chars().next().and_then(|c| self.map.get(&c))
        }
    }

    pub fn bind(&mut self, short: &str, name: &str, f: impl EditorFn + 'static) {
        if let Some(char) = short.chars().next() {
            self.map.insert(char, Command{ f: Box::new(f), name: Box::from(name) });
        }
    }

    pub fn bind_number(&mut self, name: &str, f: impl EditorFn + 'static) {
        self.number = Command { name: Box::from(name), f: Box::new(f) };
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
        assert_eq!(cmd.name.as_ref(), "append");
    }

    #[test]
    fn test_line_command() {
        let mut map = CommandMap::default();
        map.bind_number("goto-line", noop);
        let cmd = map.lookup("123").unwrap();
        assert_eq!(cmd.name.as_ref(), "goto-line");
    }

    #[test]
    fn test_noop_command() {
        let map = CommandMap::default();
        let cmd = map.lookup("").unwrap();
        assert_eq!(cmd.name.as_ref(), "noop");
    }
}

