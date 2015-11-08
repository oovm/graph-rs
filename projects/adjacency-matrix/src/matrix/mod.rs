use std::borrow::Cow;
use std::collections::LinkedList;
use ndarray::Array2;
use graph_types::{Graph, NodeIndex, PureEdge, PureNode};

pub struct AdjacencyGraph<V> {
    edges: AdjacencyMatrix,
    /// metadata of vertexes
    vertexes: Vec<V>,
}

pub struct AdjacencyEdge {
    /// index of the source node
    source: usize,
    /// index of the target node
    target: usize,
}

enum AdjacencyMatrix {
    /// A matrix that is symmetric along the diagonal.
    Undirected {
        /// lower triangular matrix
        symmetric: LinkedList<usize>,
    },
    /// A matrix that is not symmetric along the diagonal.
    Directed {
        /// full matrix
        asymmetric: LinkedList<usize>,
    },
}

impl<V> AdjacencyGraph<V> {
    pub fn new() -> Self {
        Self {
            matrix: Array2::zeros((0, 0)),
            vertexes: Vec::new(),
        }
    }
}

impl<V> Graph for AdjacencyGraph<V> where V: Clone {
    type Node = PureNode<V>;
    type Edge = PureEdge<()>;

    fn count_nodes(&self) -> usize {
        self.matrix.shape()[0]
    }

    fn get_node(&self, index: usize) -> Option<Cow<Self::Node>> {
        let item = self.vertexes.get(index)?;
        let borrowed = PureNode::re_ref(PureNode::new(index, item));
        Some(Cow::Borrowed(borrowed))
    }

    fn mut_node(&mut self, index: usize) -> Option<&mut Self::Node> {
        let item = self.vertexes.get_mut(index)?;
        let borrowed = PureNode::re_mut(PureNode::new(index, item));
        Some(borrowed)
    }
    fn insert_node(&mut self, node: Self::Node) -> NodeIndex {
        let index = self.vertexes.len();
        self.vertexes.push(node.item.clone());
        self.matrix = self.matrix.insert_axis(0);
        self.matrix = self.matrix.insert_axis(1);
        self.matrix[[index, index]] = 0;
        index
    }
}