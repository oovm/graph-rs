use super::*;
#[cfg(feature = "wolfram")]
use graph_types::{ToWolfram, WolframValue};

impl Display for DiGraphAM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let size = self.degree.end.to_string().len();
        // for row in self.matrix.rows() {
        //     for j in row.iter() {
        //         write!(f, "{:width$} ", j, width = size)?;
        //     }
        //     writeln!(f)?;
        // }
        todo!();
        Ok(())
    }
}

#[cfg(feature = "wolfram")]
impl ToWolfram for AdjacencyCell {
    fn to_wolfram(&self) -> WolframValue {
        todo!()
    }
}
