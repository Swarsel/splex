mod solution;
mod graph;
mod parser;
mod construction;
mod symmat;

use construction::ConstructionHeuristic;

use std::fs;

use crate::graph::Graph;

fn load_graph(id: usize) -> graph::Graph {
    let mut paths = fs::read_dir("../instances/test_instances").unwrap();

    let fp = paths.find(|path| {
        let path = path.as_ref().unwrap().path();
        let path = path.to_str().unwrap();

        path.contains(format!("heur{:03}_", id).as_str())
    });

    // load file 
    let fp = fp.unwrap().unwrap().path();
    let fp = fp.to_str().unwrap();

    let content = fs::read_to_string(fp).unwrap();

    parser::parse(&content).unwrap()
}


fn main() {
    for i in 1..=10 {
        let graph = load_graph(i);


        let solution = construction::Greedy::new(0.4).construct(&graph);

        println!("Graph: {}", i);
        println!("{:?}", solution);
        dbg!(Graph::get_connection_components(&graph.initial).len());
    }
}
