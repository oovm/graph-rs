use crate::CompleteGraph;
use graph_types::{GraphEngine, GraphKind};

#[cfg(feature = "wolfram")]
impl graph_types::ToWolfram for CompleteGraph {
    /// Convert rust [CompleteGraph] to wolfram [CompleteGraph](https://reference.wolfram.com/language/ref/CompleteGraph.html)
    fn to_wolfram(&self) -> graph_types::WolframValue {
        let n = graph_types::WolframValue::Integer64(self.rank() as i64);
        let args = match self.graph_kind() {
            GraphKind::Directed => {
                let arg1 = graph_types::WolframValue::pair("DirectedEdges", true, false);
                vec![n, arg1]
            }
            GraphKind::Undirected => vec![n],
        };
        graph_types::WolframValue::function("CompleteGraph", args)
    }
}
