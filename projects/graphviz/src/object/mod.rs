use color_core::HSVA32;
use graph_theory::{
    entry_engines::{DictStorage, ListStorage},
    graph_engines::AdjacencyNodeList,
};
use shape_core::Point;

pub struct Graphviz {
    graph: AdjacencyNodeList,
    // each node contains a location, of course dense
    position: ListStorage<Point<f32>>,
    ///
    color: DictStorage<HSVA32>,
}

impl Default for Graphviz {
    fn default() -> Self {
        Self { graph: AdjacencyNodeList::default(), position: ListStorage::default(), color: DictStorage::default() }
    }
}
