use super::*;
#[cfg(feature = "wolfram")]
use graph_types::{ToWolfram, WolframValue};

impl Display for DiGraphAM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.max_degree.to_string().len();
        for j in 0..self.rank {
            for i in 0..self.rank {
                let cell = self.matrix[j * self.rank + i];
                write!(f, "{:width$} ", cell.node_degree, width = width)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(feature = "wolfram")]
impl ToWolfram for AdjacencyCell {
    fn to_wolfram(&self) -> WolframValue {
        todo!()
    }
}
