use std::collections::BTreeMap;
// use adjacency_list::AdjacencyNodeList;
use color_core::{HSVColor, HSVA32, RGBA32, RGBA8};
use shape_core::Point;

pub struct Graphviz {
    /// each node contains a location, of course dense
    position: PositionProvider,
    /// size or weight of each node, of course dense
    // size: ListStorage<f32>,
    /// each node contains a color, sparse with a default value
    color: ColorProvider,
}

pub struct GraphvizConfig {
    pub is_script: bool,
}

pub struct PositionProvider {
    nodes: Vec<Point<f32>>,
    edges: Vec<Point<f32>>,
}

pub struct ColorProvider {
    node_default: RGBA8,
    edge_default: RGBA8,
    nodes: BTreeMap<usize, RGBA8>,
    edges: BTreeMap<usize, RGBA8>,
}
