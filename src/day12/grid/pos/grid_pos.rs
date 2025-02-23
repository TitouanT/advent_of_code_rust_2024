use super::Pos;
#[derive(Debug, Clone, Copy)]
pub struct GridPos {
    pub line: usize,
    pub column: usize,
}

impl GridPos {
    pub fn from(p: Pos) -> Self {
        let p = p + p.heading.corner();
        Self {
            line: p.line,
            column: p.column,
        }
    }
}
