use wolfram_wxf::{ToWolfram, WolframValue};

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
