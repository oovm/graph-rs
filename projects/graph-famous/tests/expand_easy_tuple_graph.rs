use graph_derive::Graph;

#[derive(Graph)]
pub struct EasyTupleGraph(#[easy_graph] i32);
