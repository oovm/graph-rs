use super::*;

#[derive(Clone, Debug)]
pub struct BridgeRange<'i, G>
where
    G: GraphEngine<'i>,
{
    graph: &'i G,
    indexer: Range<usize>,
}

impl<'i, G> Iterator for BridgeRange<'i, G>
where
    G: GraphEngine<'i>,
{
    type Item = IndeterminateEdge;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.indexer.next()?;
        match self.graph.get_bridge(index) {
            Ok(o) => Some(o),
            Err(_) => self.next(),
        }
    }
}

impl<'i, G> DoubleEndedIterator for BridgeRange<'i, G>
where
    G: GraphEngine<'i>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.indexer.next_back()?;
        match self.graph.get_bridge(index) {
            Ok(o) => Some(o),
            Err(_) => self.next_back(),
        }
    }
}
