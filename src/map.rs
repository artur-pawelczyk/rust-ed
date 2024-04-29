use std::collections::HashMap;

use crate::editor::Command;

#[derive(Default)]
pub struct CommandMap {
    map: HashMap<char, Command>,
}

impl CommandMap {
    pub fn lookup(&self, s: &str) -> Option<Command> {
        if s.trim().is_empty() {
            Some(Command::Noop)
        } else if let Ok(n) = s.parse::<usize>() {
            Some(Command::Line(n))
        } else {
            s.chars().next().and_then(|c| self.map.get(&c)).map(|c| *c)
        }
    }

    pub fn bind(&mut self, s: &str, cmd: Command) {
        if let Some(char) = s.chars().next() {
            self.map.insert(char, cmd);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_command() {
        let mut map = CommandMap::default();
        map.bind("a", Command::Noop);
        assert!(map.lookup("a").is_some());
    }

    #[test]
    fn test_line_command() {
        let map = CommandMap::default();
        let cmd = map.lookup("123").unwrap();
        assert_eq!(cmd, Command::Line(123));
    }

    #[test]
    fn test_noop_command() {
        let map = CommandMap::default();
        let cmd = map.lookup("").unwrap();
        assert_eq!(cmd, Command::Noop);
    }
}
