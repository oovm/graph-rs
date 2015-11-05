#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeDirection {
    TwoWay,
    Forward,
    Reverse,
}

pub trait Edge {
    fn from(&self) -> NodeIndex;
    fn goto(&self) -> NodeIndex;
    fn direction(&self) -> EdgeDirection;
}


impl<M> Edge for PureEdge<M> {
    fn from(&self) -> NodeIndex {
        self.from
    }

    fn goto(&self) -> NodeIndex {
        self.goto
    }

    fn direction(&self) -> EdgeDirection {
        self.direction
    }
}
