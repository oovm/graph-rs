use super::*;
#[cfg(feature = "wolfram")]
use graph_types::{ToWolfram, WolframValue};

impl Display for AdjacencyMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.degree.end.to_string().len();
        for row in self.matrix.rows() {
            for j in row.iter() {
                write!(f, "{:width$} ", j, width = size)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(feature = "wolfram")]
impl ToWolfram for AdjacencyMatrix {
    fn to_wolfram(&self) -> WolframValue {
        let rows = self
            .matrix
            .chunks_exact(self.nodes())
            .map(|row| WolframValue::list(row.iter().map(|edge| WolframValue::Integer64(*edge as i64)).collect()))
            .collect();
        WolframValue::function("AdjacencyGraph", vec![WolframValue::list(rows)])
    }
}
