use std::borrow::Cow;
use ndarray::Array2;

mod edges;

pub type NodeIndex = usize;

pub trait Graph {
    type Vertex: Vertex;
    type Edge: Edge;
    fn vertexes<I>(&self) -> I where I: Iterator<Item=Cow<Self::Vertex>>;
    fn edges<I>(&self) -> I where I: Iterator<Item=Cow<Self::Edge>>;
}



pub struct PureEdge<M> {
    from: NodeIndex,
    goto: NodeIndex,
    direction: EdgeDirection,
    metadata: M,
}

pub trait Vertex {
    fn index(&self) -> NodeIndex;
}

pub struct PureVertex<M> {
    index: NodeIndex,
    metadata: M,
}

pub struct AdjacencyMatrix {
    matrix: Array2<usize>,
}


impl Graph for AdjacencyMatrix {
    type Vertex = ();
    type Edge = PureEdge;

    fn vertexes<I>(&self) -> I where I: Iterator<Item=Cow<Self::Vertex>> {
        todo!()
    }

    fn edges<I>(&self) -> I where I: Iterator<Item=Cow<Self::Edge>> {
        todo!()
    }
}