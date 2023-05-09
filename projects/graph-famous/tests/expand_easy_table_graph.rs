use graph_derive::Graph;

#[derive(Graph)]
pub struct EasyTableGraph {
    #[easy_graph]
    mask: i64,
}
