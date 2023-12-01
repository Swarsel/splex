mod construction;
mod graph;
mod grasp;
mod neighborhood;
mod parser;
mod solution;
mod symmat;
mod vnd;

use crate::construction::ConstructionHeuristic;
use crate::grasp::GRASP;
use crate::neighborhood::nflip::NFlip;

use crate::neighborhood::stepfunction::StepFunction;
use crate::vnd::VND;

use std::fs;
use std::io::Write;

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
    let mut out = fs::File::create("out.txt").unwrap();

    for i in 9..=10 {
        println!("Graph: {}", i);

        let graph = load_graph(i);

        let vnd = VND::new(
            Box::new(construction::Greedy::new(0.7)),
            vec![(Box::new(NFlip::new(2)), StepFunction::FirstImprovement)],
        );

        let grasp = GRASP::new(vnd);

        let start = std::time::Instant::now();
        // let best = vnd.run(&graph);
        let best = grasp.run(&graph, true);
        let elapsed = start.elapsed();

        println!("Best solution: {}", best.cost);
        println!("Elapsed: {:?}", elapsed);

        writeln!(out, "Graph {}", i).unwrap();
        writeln!(out, "{:?}", best).unwrap();
    }
}
