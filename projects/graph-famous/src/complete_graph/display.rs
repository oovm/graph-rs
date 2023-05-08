use super::*;

impl Debug for CompleteGraph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompleteGraph").field("nodes", &self.count_nodes()).field("edges", &self.count_edges()).finish()
    }
}

impl Display for CompleteGraph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CompleteGraph({})", self.mask)
    }
}
