#[derive(Copy, Clone, Debug)]
pub enum Entry {
    Node,
    Edge,
}

/// mark a entry weight
#[derive(Clone, Debug)]
pub struct EntryWeight<N> {
    weight: N,
}

#[derive(Clone, Debug)]
pub struct EntryName {
    name: String,
}

#[derive(Clone, Copy, Debug)]
pub struct Query {
    pub entry: Entry,
    pub index: usize,
}

impl Query {
    pub fn node(id: usize) -> Self {
        Self { entry: Entry::Node, index: id }
    }
    pub fn edge(id: usize) -> Self {
        Self { entry: Entry::Edge, index: id }
    }
}
