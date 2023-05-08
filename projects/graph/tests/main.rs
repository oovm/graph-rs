#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_serde() {
    let g = graph_theory::graph_engines::StarGraph::two_way(3);
    println!("{:?}", g);
    let s = serde_json::to_string(&g).unwrap();
    println!("{}", s);
}
