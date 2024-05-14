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
}
