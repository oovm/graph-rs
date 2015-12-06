
use super::*;


impl Display for UndirectedEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ↔ {}", self.from, self.goto)
    }
}

impl Display for DirectedEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} → {}", self.from, self.goto)
    }
}