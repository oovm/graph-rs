use std::cmp::{max, min};
use std::collections::LinkedList;
use std::fmt::{Debug, Display};
use std::ops::Range;
use graph_types::{EdgeDirection, GraphResult, PureEdge};

#[derive(Clone, Debug)]
pub struct StaticUndirected<V = (), E = ()> {
    /// metadata of vertexes
    vertexes: Vec<V>,
    /// edges, lower triangular matrix
    edges: Vec<AdjacencyEdge<E>>,
}

impl<V, E> Display for StaticUndirected<V, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.min_degree().to_string().len();
        let max = self.vertexes.len();
        for i in 0..max {
            for j in 0..max {
                if j > i {
                    write!(f, "{:width$} ", ".", width = size)?;
                } else {
                    let index = i * (i + 1) / 2 + j;
                    let edge = self.edges.get(index).unwrap();
                    write!(f, "{:width$} ", edge.degree, width = size)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct AdjacencyEdge<M> {
    degree: usize,
    metadata: M,
}

impl<V, E> StaticUndirected<V, E> {
    pub fn max_degree(&self) -> usize {
        self.edges.iter().map(|edge| edge.degree).max().unwrap_or(0)
    }
    pub fn min_degree(&self) -> usize {
        0
    }
}

impl<V, E> StaticUndirected<V, E> where V: Default, E: Default {
    pub fn new(size: usize) -> Self {
        let mut vertexes = Vec::with_capacity(size);
        vertexes.resize_with(size, V::default);
        let mut edges = Vec::with_capacity((size + 1) * size / 2);
        edges.resize_with((size + 1) * size / 2, AdjacencyEdge::default);
        Self {
            vertexes,
            edges,
        }
    }
    pub fn get_connection(&self, edge: FastEdge) -> GraphResult<&E> {
        let min_index = edge.min_index();
        let max_index = edge.max_index();
        let index = max_index * (max_index + 1) / 2 + min_index;
        match edge.two_way {
            true => {
                match self.edges.get(index) {
                    Some(s) => {
                        Ok(&s.metadata)
                    }
                    None => { unreachable!("{}", index) }
                }
            }
            false => { unreachable!() }
        }
    }


    pub fn set_connection<F>(&mut self, edge: FastEdge, edit: F) -> GraphResult<usize>
        where F: Fn(&mut AdjacencyEdge<E>)
    {
        let min_index = edge.min_index();
        let max_index = edge.max_index();
        let index = max_index * (max_index + 1) / 2 + min_index;
        match edge.two_way {
            true => {
                match self.edges.get_mut(index) {
                    Some(s) => {
                        edit(s);
                        Ok(s.degree)
                    }
                    None => { unreachable!("{}", index) }
                }
            }
            false => { unreachable!() }
        }
    }
    pub fn connect(&mut self, edge: FastEdge) -> GraphResult<usize> {
        self.set_connection(edge, |e| e.degree = e.degree.saturating_add(1))
    }
    pub fn disconnect(&mut self, edge: FastEdge) -> GraphResult<usize> {
        self.set_connection(edge, |e| e.degree = e.degree.saturating_sub(1))
    }
}

pub struct FastEdge {
    pub two_way: bool,
    pub from: usize,
    pub goto: usize,
}

impl FastEdge {
    pub fn max_index(&self) -> usize {
        max(self.from, self.goto)
    }
    pub fn min_index(&self) -> usize {
        min(self.from, self.goto)
    }
    pub fn as_range(&self) -> Range<usize> {
        self.min_index()..self.max_index()
    }
}

#[test]
fn test() {
    let mut graph = StaticUndirected::<(), ()>::new(5);
    graph.connect(FastEdge {
        two_way: true,
        from: 4,
        goto: 4,
    }).unwrap();
    println!("{}", graph.edges.len());
    println!("{}", graph);
}