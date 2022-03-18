use std::hash::Hasher;

#[derive(Clone, Eq)]
pub struct Position {
    pub column: usize,
    pub index:  usize,
    pub line:   usize,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.line, self.column)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::hash::Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl std::cmp::Ord for Position {
    fn cmp(&self, other: &Position) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl std::cmp::PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.index == other.index
    }
}
