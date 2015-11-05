use std::borrow::Cow;
use ndarray::Array2;
use graph_types::{Graph, NodeIndex, PureEdge};

pub struct AdjacencyMatrix {
    matrix: Array2<usize>,
}


impl Graph for AdjacencyMatrix {
    type Node = NodeIndex;
    type Edge = PureEdge<()>;

    fn count_nodes(&self) -> usize {
        todo!()
    }

    fn get_node(&self, index: usize) -> Option<Cow<'_, Self::Node>> {
        Some(Cow::Owned(index))
    }

    fn mut_node(&mut self, index: usize) -> Option<&mut Self::Node> {
        None
    }
}