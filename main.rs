use std::collections;

extern crate crates_index;
extern crate petgraph;

fn main() {
    let mut name_to_nodes = collections::HashMap::new();
    let mut graph = petgraph::Graph::<String, ()>::new();

    let index = crates_index::Index::new("_index".into());
    if !index.exists() {
        index.retrieve().expect("Could not fetch crates.io index");
    }
    for crate_ in index.crates() {
        let latest_version = crate_.latest_version();
        let node = graph.add_node(latest_version.name().to_owned());
        name_to_nodes.insert(latest_version.name().to_owned(), node);
    }
    for crate_ in index.crates() {
        let latest_version = crate_.latest_version();
        for dependency in latest_version.dependencies() {
            let from = name_to_nodes.get(latest_version.name()).expect("could not get latest version");
            let to = match name_to_nodes.get(dependency.name()) {
                Some(n) => n,
                None => return,
            };
            graph.add_edge(*from, *to, ());
        }
    }
    let config = [petgraph::dot::Config::EdgeNoLabel];
    let dot = petgraph::dot::Dot::with_config(&graph, &config);
    println!("{:?}", dot);
}
