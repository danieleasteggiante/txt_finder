use std::fmt::Display;
#[derive(Debug)]
pub struct StringFile {
    pub path: String,
    pub contents: Vec<LineNr>,
}
#[derive(Debug)]
pub struct LineNr {
    pub line: String,
    pub number: usize,
}

impl Display for LineNr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} -> {}", self.number, self.line)
    }
}


