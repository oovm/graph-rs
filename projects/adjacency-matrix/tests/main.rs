use graph_types::UndirectedEdge;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let mut graph = StaticUndirected::<(), ()>::new(5);
    graph.connect(UndirectedEdge { from: 4, goto: 4 }).unwrap();
    println!("{}", graph.edges.len());
    println!("{}", graph);
}
