use adjacency_matrix::{StaticDirected, StaticUndirected};
use graph_types::ToWolfram;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    println!("directed graph:");
    let mut graph = StaticUndirected::new(5);
    graph.connect((1, 3)).unwrap();
    graph.connect((1, 4)).unwrap();
    println!("{}", graph);
    println!("undirected graph:");
    let mut graph = StaticDirected::new(5);
    graph.connect((2, 3)).unwrap();
    graph.connect((2, 4)).unwrap();
    graph.connect((3, 1)).unwrap();
    graph.connect((4, 1)).unwrap();
    println!("{}", graph);
}
