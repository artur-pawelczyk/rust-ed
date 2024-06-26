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

    pub fn current_line(&self) -> Region {
        self.line_at(self.line)
    }

    pub fn line_at(&self, line: usize) -> Region {
        if line > 1 {
            let mut lines = self.contents
                .char_indices()
                .filter(|(_, c)| *c == '\n')
                .map(|(n, _)| n)
                .skip(line - 2);

            let start = lines.next().map(|n| n + 1);
            let end = lines.next().unwrap_or(self.contents.len());
            let start = start.or_else(|| self.contents[..end-1].rfind('\n')).unwrap_or(0);
            Region(start, end)
        } else {
            let end = self.contents.char_indices()
                .find(|(_, c)| *c == '\n')
                .map(|(n, _)| n)
                .unwrap_or(self.contents.len());

            Region(0, end)
        }
    }

    pub fn beginning_of_line(&self, n: usize) -> Point {
        if n > 1 {
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

    pub fn end_of_line(&self, n: usize) -> Point {
        self.contents
            .char_indices()
            .filter(|(_, c)| *c == '\n')
            .map(|(n, _)| n)
            .nth(n - 1)
            .map(|n| Point(n + 1))
            .unwrap_or(Point(0))
    }

    pub fn line_at_point(&self, p: &Point) -> usize {
        self.contents[..p.0].lines().count() + 1
    }

    pub fn region_line_number(&self, Region(start, _): &Region) -> usize {
        self.contents[..*start].lines().count() + 1
    }

    pub fn region_text(&self, r: &Region) -> &str {
        let Region(start, end) = r;
        &self.contents[*start..*end]
    }

    pub fn insert(&mut self, p: Point, s: &str) -> Point {
        self.contents.insert_str(p.0, s);
        Point(p.0 + s.len())
    }

    pub fn replace(&mut self, Region(start, end): Region, s: &str) -> Region {
        self.contents.replace_range(start..end, s);
        Region(start, s.len())
    }

    pub fn lines(&self) -> impl Iterator<Item = (usize, &str)> {
        self.contents.lines().enumerate().map(|(n, s)| (n+1, s))
    }

    pub fn lines_around(&self, l: usize, n: usize) -> impl Iterator<Item = (usize, &str)> {
        let (lines_before, lines_after) = split_integer(n);
        self.lines()
            .skip(if lines_before > l { 0 } else { l - lines_before - 1 })
            .take(lines_before + lines_after)
    }
}

fn split_integer(n: usize) -> (usize, usize) {
    if n % 2 == 0 {
        (n/2, n/2)
    } else {
        (n/2, n/2 + 1)
    }
}

#[derive(Debug)]
pub struct Region(usize, usize);

#[derive(Debug)]
pub struct Point(usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_line() {
        let mut buf = Buffer::with_contents("first\nsecond\n");

        let l = buf.current_line();
        assert_eq!(buf.region_line_number(&l), 1);
        assert_eq!(buf.region_text(&l), "first");

        buf.line = 2;
        let l = buf.current_line();
        assert_eq!(buf.region_line_number(&l), 2);
        assert_eq!(buf.region_text(&l), "second");
    }

    #[test]
    fn test_change_line() {
        let mut buf = Buffer::with_contents("first\nsecond\nthird\n");

        let r = buf.replace(buf.current_line(), "changed");

        assert_eq!(buf.contents, "changed\nsecond\nthird\n");
        assert_eq!(buf.region_text(&r), "changed");
    }

    #[test]
    fn test_insert_at_point() {
        let mut buf = Buffer::with_contents("first\nthird\n");

        let p = buf.beginning_of_line(2);
        let p = buf.insert(p, "second\n");
        assert_eq!(buf.contents, "first\nsecond\nthird\n");
        assert_eq!(p.0, 13);

        let p = buf.end_of_line(2);
        buf.insert(p, "more\n");
        assert_eq!(buf.contents, "first\nsecond\nmore\nthird\n");
    }

    #[test]
    fn test_line_at_point() {
        let buf = Buffer::with_contents("one\ntwo\nthree\nfour\n");

        let p = buf.beginning_of_line(2);
        assert_eq!(buf.line_at_point(&p), 2);
    }

    #[test]
    fn test_line_out_of_bounds() {
        let buf = Buffer::with_contents("one\ntwo\nthree\n");

        let p = buf.line_at(100);
        assert_eq!(buf.region_line_number(&p), 3);

        let p = buf.line_at(0);
        assert_eq!(buf.region_line_number(&p), 1);
    }

    #[test]
    fn test_line_iter() {
        let buf = Buffer::with_contents("one\ntwo\n");

        let v = buf.lines().collect::<Vec<_>>();
        assert_eq!(v[0], (1, "one"));
        assert_eq!(v[1], (2, "two"));
    }

    #[test]
    fn test_lines_around() {
        let text: String = (1..=20).map(|i| format!("{i}\n")).collect();
        let buf = Buffer::with_contents(&text);

        assert_eq!(buf.lines_around(5, 100).count(), 20);
        assert_eq!(collect_lines(buf.lines_around(5, 1)), vec!["5"]);
        assert_eq!(collect_lines(buf.lines_around(10, 4)), vec!["8", "9", "10", "11"]);
    }

    fn collect_lines<'a>(i: impl Iterator<Item = (usize, &'a str)>) -> Vec<&'a str> {
        i.map(|(_, s)| s).collect()
    }
}
