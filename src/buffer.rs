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

    pub fn current_line(&mut self) -> Line {
        self.line_at(self.line)
    }

    pub fn line_at(&mut self, line: usize) -> Line {
        if line > 1 {
            let mut lines = self.contents
                .char_indices()
                .filter(|(_, c)| *c == '\n')
                .map(|(n, _)| n)
                .skip(line - 2);

            Line {
                start: lines.next().unwrap() + 1,
                end: lines.next().unwrap_or(self.contents.len()),
                buf: &mut self.contents,
            }
        } else {
            let end = self.contents.char_indices()
                .find(|(_, c)| *c == '\n')
                .map(|(n, _)| n)
                .unwrap_or(self.contents.len());

            Line {
                buf: &mut self.contents,
                start: 0,
                end
            }
        }
    }

    pub fn beginning_of_line(&self, n: usize) -> Point {
        if n > 1 {
            dbg!(self.contents.char_indices().filter(|(_, c)| *c == '\n').collect::<Vec<_>>());
            self.contents
                .char_indices()
                .filter(|(_, c)| *c == '\n')
                .map(|(n, _)| n)
                .nth(n - 2)
                .map(|n| Point(n + 1))
                .unwrap_or(Point(0))
        } else {
            Point(0)
        }
    }

    pub fn insert(&mut self, p: &Point, s: &str) {
        self.contents.insert_str(p.0, s);
    }
}

#[derive(Debug)]
pub struct Line<'a> {
    buf: &'a mut String,
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

    pub fn set(&mut self, s: &str) {
        self.buf.replace_range(self.start..self.end, s);
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

#[derive(Debug)]
struct Point(usize);

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
        let mut buf = Buffer::with_contents("first\nsecond\nthird\n");

        assert_eq!(buf.line_at(3).pos(), 3);
        assert_eq!(buf.current_line().pos(), 1);
    }

    #[test]
    fn test_change_line() {
        let mut buf = Buffer::with_contents("first\nsecond\nthird\n");

        buf.current_line().set("changed");

        assert_eq!(buf.contents, "changed\nsecond\nthird\n");
        assert_eq!(buf.current_line().text(), "changed");
    }

    #[test]
    fn test_insert_at_point() {
        let mut buf = Buffer::with_contents("first\nthird\n");

        let p = buf.beginning_of_line(2);
        dbg!(&p);
        buf.insert(&p, "second\n");

        assert_eq!(buf.contents, "first\nsecond\nthird\n");
    }
}
