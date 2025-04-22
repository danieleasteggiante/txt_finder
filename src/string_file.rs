use std::fmt::Display;

pub struct StringFile {
    pub path: String,
    pub contents: Vec<LineNr>,
}

pub struct LineNr {
    pub line: String,
    pub number: usize,
}

impl Display for LineNr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.number, self.line)
    }
}

