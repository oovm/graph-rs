pub struct AdjacencyMatrix {
    matrix: Array2<usize>,
}


impl Graph for AdjacencyMatrix {
    type Node = ();
    type Edge = PureEdge;

    fn nodes<I>(&self) -> I where I: Iterator<Item=Cow<Self::Node>> {
        todo!()
    }

    fn edges<I>(&self) -> I where I: Iterator<Item=Cow<Self::Edge>> {
        todo!()
    }
}