#[derive(Copy, Clone, Debug)]
pub enum Entry {
    Node,
    Edge,
}
#[derive(Clone, Copy, Debug)]
pub struct Query {
    entry: Entry,
    index: usize,
}
