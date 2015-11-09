use crate::NodeIndex;

mod simple;

pub trait Edge {
    fn from(&self) -> NodeIndex;
    fn goto(&self) -> NodeIndex;
    fn direction(&self) -> EdgeDirection;
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeDirection {
    TwoWay,
    Forward,
    Reverse,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UndirectedEdge {
    pub from: usize,
    pub goto: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectedEdge {
    pub from: usize,
    pub goto: usize,
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PureEdge<M> {
    from: NodeIndex,
    goto: NodeIndex,
    direction: EdgeDirection,
    metadata: M,
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
