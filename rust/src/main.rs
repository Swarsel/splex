mod construction;
mod graph;
mod neighborhood;
mod parser;
mod solution;
mod symmat;

use crate::construction::ConstructionHeuristic;
use crate::neighborhood::oneflip::OneFlip;

use std::fs;

use crate::graph::Graph;
use crate::neighborhood::neighborhood::Neighborhood;

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
    for i in 1..=1 {
        let graph = load_graph(i);

        let solution = construction::Greedy::new(0.7).construct(&graph);

        println!("Initial solution: {}", solution.cost);

        let neighborhood = OneFlip;

        let mut best = solution.clone();

        for neighbor in neighborhood.iter_neighbors(solution.clone()) {
            if neighbor.is_valid() {
                println!("Neighbor: {}", neighbor.cost);
                if neighbor.cost < best.cost {
                    best = neighbor;
                }
            }
        }

        println!("Graph: {}", i);
        println!("{:?}", best);
        dbg!(Graph::get_connection_components(&graph.initial).len());
    }
}
