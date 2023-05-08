use super::*;


#[cfg(feature = "wolfram_wxf")]
use wolfram_wxf::{ToWolfram, WolframValue};

#[cfg(feature = "wolfram_wxf")]
impl ToWolfram for CompleteGraph {
    fn to_wolfram(&self) -> WolframValue {
        let n = WolframValue::Integer64(self.rank as i64);

        match self.directed {
            true => {
                let arg1 = WolframValue::pair("DirectedEdges", true, false);
                WolframValue::function("CompleteGraph", vec![n, arg1])
            }
            false => WolframValue::function("CompleteGraph", vec![n]),
        }
    }
}

impl Debug for CompleteGraph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompleteGraph")
            .field("directed", &self.directed)
            .field("nodes", &self.count_nodes())
            .field("edges", &self.count_edges())
            .finish()
    }
}

impl Display for CompleteGraph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.directed {
            true => write!(f, "CompleteGraph({}, directed)", self.rank),
            false => write!(f, "CompleteGraph({})", self.rank),
        }
    }
}