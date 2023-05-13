use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};
// use adjacency_list::AdjacencyNodeList;
use color_core::RGBA8;

use graph_types::{
    errors::GraphError, placeholder::PlaceholderGraph, DictDefault, DictStorage, EdgeQuery, GraphEngine, GraphKind,
    ListStorage, NamedGraph, NodeQuery, NodesVisitor, Query,
};
use shape_core::Point;

pub struct Graphviz {
    graph: PlaceholderGraph,
    // not all node have a name, use sparse storage
    name: DictStorage<String>,
    // all node have a position, use dense storage
    position: ListStorage<Point<f32>>,
    /// not all node have a color, use sparse storage
    color: DictDefault<RGBA8>,
}

pub struct GraphvizConfig {
    pub is_script: bool,
}

impl<'i> NamedGraph<'i> for Graphviz {
    type NameRef = &'i str;
    type NameMut = &'i mut String;

    fn get_node_name(&'i self, node: NodeID) -> Option<Self::NameRef> {
        let name = self.name.get_data(node.into().into())?;
        Some(name.as_str())
    }

    fn mut_node_name(&'i mut self, node: NodeID) -> Option<Self::NameMut> {
        let name = self.name.mut_data(node.into().into())?;
        Some(name)
    }

    fn set_node_name(&'i mut self, node: NodeID, name: &str) {
        match self.mut_node_name(node) {
            Some(s) => *s = name.to_string(),
            None => {}
        }
    }

    fn get_edge_name<Q: Into<EdgeQuery>>(&'i self, edge: Q) -> Option<Self::NameRef> {
        let name = self.name.get_data(edge.into().into())?;
        Some(name.as_str())
    }

    fn mut_edge_name<Q: Into<EdgeQuery>>(&'i mut self, edge: Q) -> Option<Self::NameMut> {
        let name = self.name.mut_data(edge.into().into())?;
        Some(name)
    }

    fn set_edge_name<Q: Into<EdgeQuery>>(&'i mut self, edge: Q, name: &str) {
        match self.mut_edge_name(edge) {
            Some(s) => *s = name.to_string(),
            None => {}
        }
    }
}
