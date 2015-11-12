use crate::{DirectedEdge, UndirectedEdge};
use wolfram_wxf::{ToWolfram, WolframValue};

impl ToWolfram for UndirectedEdge {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::function(
            "UndirectedEdge",
            vec![WolframValue::Integer64(self.from as i64 + 1), WolframValue::Integer64(self.goto as i64 + 1)],
        )
    }
}

impl ToWolfram for DirectedEdge {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::function(
            "DirectedEdge",
            vec![WolframValue::Integer64(self.from as i64 + 1), WolframValue::Integer64(self.goto as i64 + 1)],
        )
    }
}
