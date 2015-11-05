use ndarray::Array2;
use graph_types::{Graph, PureEdge, PureNode};

pub struct AdjacencyMatrix {
    matrix: Array2<usize>,
}


impl Graph for AdjacencyMatrix {
    type Node = PureNode<()>;
    type Edge = PureEdge<()>;

    fn get_nodes<I>(&self) -> I where I: Iterator<Item=Self::Node> {
        self.matrix
            .outer_iter()
            .enumerate()
            .map(|(index, _)| PureNode::new(index, ()) )
    }

    fn mut_nodes<I>(&mut self) -> I where I: Iterator<Item=Self::Node> {
        self.matrix
            .outer_iter_mut()
            .enumerate()
            .map(|(index, _)| PureNode::new(index, ()) )
    }
}