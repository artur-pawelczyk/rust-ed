use std::collections::HashMap;

use crate::editor::CommandEnum;

#[derive(Default)]
pub struct CommandMap {
    map: HashMap<char, CommandEnum>,
}

impl CommandMap {
    pub fn lookup(&self, s: &str) -> Option<CommandEnum> {
        if s.trim().is_empty() {
            Some(CommandEnum::Noop)
        } else if let Ok(n) = s.parse::<usize>() {
            Some(CommandEnum::Line(n))
        } else {
            s.chars().next().and_then(|c| self.map.get(&c)).map(|c| *c)
        }
    }

    pub fn bind(&mut self, s: &str, cmd: CommandEnum) {
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
        map.bind("a", CommandEnum::Noop);
        assert!(map.lookup("a").is_some());
    }

    #[test]
    fn test_line_command() {
        let map = CommandMap::default();
        let cmd = map.lookup("123").unwrap();
        assert_eq!(cmd, CommandEnum::Line(123));
    }

    #[test]
    fn test_noop_command() {
        let map = CommandMap::default();
        let cmd = map.lookup("").unwrap();
        assert_eq!(cmd, CommandEnum::Noop);
    }
}
