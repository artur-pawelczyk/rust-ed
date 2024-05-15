use std::fmt;

pub struct Buffer {
    pub contents: String,
    pub line: usize,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            contents: String::new(),
            line: 1
        }
    }
}

impl Buffer {
    pub fn with_contents(s: &str) -> Self {
        Self {
            contents: String::from(s),
            ..Default::default()
        }
    }

    pub fn current_line(&self) -> Line {
        self.line_at(self.line)
    }

    pub fn line_at(&self, line: usize) -> Line {
        if line > 1 {
            let mut lines = self.contents
                .char_indices()
                .filter(|(_, c)| *c == '\n')
                .map(|(n, _)| n)
                .skip(line - 2);

            Line {
                buf: &self.contents,
                start: lines.next().unwrap() + 1,
                end: lines.next().unwrap_or(self.contents.len())
            }
        } else {
            let end = self.contents.char_indices()
                .find(|(_, c)| *c == '\n')
                .map(|(n, _)| n)
                .unwrap_or(self.contents.len());

            Line {
                buf: &self.contents,
                start: 0,
                end
            }
        }
    }
}

#[derive(Debug)]
pub struct Line<'a> {
    buf: &'a str,
    start: usize,
    end: usize,
}

impl<'a> Line<'a> {
    pub fn pos(&self) -> usize {
        self.buf[0..self.start].lines().count() + 1
    }

    pub fn text(&self) -> &str {
        &self.buf[self.start..self.end]
    }

    pub fn set(&self, s: &str) {
    }
}

impl fmt::Display for Line<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}]{}",
               &self.buf[..self.start],
               &self.buf[self.start..self.end],
               &self.buf[self.end..]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_line() {
        let mut buf = Buffer::with_contents("first\nsecond\n");

        let l = buf.current_line();
        dbg!(&l);
        assert_eq!(l.pos(), 1);
        assert_eq!(l.text(), "first");

        buf.line = 2;
        let l = buf.current_line();
        assert_eq!(l.pos(), 2);
        assert_eq!(l.text(), "second");
    }

    #[test]
    fn test_line_at() {
        let buf = Buffer::with_contents("first\nsecond\nthird\n");

        assert_eq!(buf.line_at(3).pos(), 3);
        assert_eq!(buf.current_line().pos(), 1);
    }

    #[ignore]
    #[test]
    fn test_change_line() {
        let mut buf = Buffer::with_contents("first\nsecond\nthird\n");

        buf.current_line().set("changed");

        assert_eq!(buf.contents, "changed\nsecond\nthird\n");
    }
}
