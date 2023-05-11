use color_core::{HSVColor, HSVA32, RGBA8};
use graph_theory::{
    entry_engines::{DictStorage, ListStorage},
    graph_engines::AdjacencyNodeList,
};
use shape_core::Point;

pub struct Graphviz {
    graph: AdjacencyNodeList,
    /// each node contains a location, of course dense
    position: ListStorage<Point<f32>>,
    /// each node contains a color, of course dense
    color: DictStorage<HSVA32>,
    color_of_default_node: HSVA32,
    color_of_default_edge: HSVA32,
}

pub struct GraphvizNode {
    pub position: Point<f32>,
    pub color: HSVA32,
}

impl Default for Graphviz {
    fn default() -> Self {
        Self {
            graph: AdjacencyNodeList::default(),
            position: ListStorage::default(),
            color: DictStorage::default(),
            color_of_default_node: HSVA32::from(RGBA8::new()),
            color_of_default_edge: HSVColor { h: (), s: (), v: (), a: () },
        }
    }
}
