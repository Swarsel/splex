#![allow(dead_code)]
#![allow(unused_imports)]

mod construction;
mod graph;
mod grasp;
mod neighborhood;
mod parser;
mod solution;
mod symmat;
mod vnd;
mod gvns;

use crate::construction::{ConstructionHeuristic, Greedy};
use crate::grasp::GRASP;
use crate::neighborhood::movevertex::MoveVertex;
use crate::neighborhood::nflip::{self, NFlip};

use crate::neighborhood::oneflip::OneFlip;
use crate::neighborhood::stepfunction::StepFunction;
use crate::vnd::VND;

use std::fs;
use std::io::Write;

fn load_graph(id: usize) -> graph::Graph {
    let mut paths = if (49..=51).contains(&id) {
        fs::read_dir("../instances/inst_competition").unwrap()
    } else {
        fs::read_dir("../instances/test_instances").unwrap()
    };

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

    let total = std::time::Instant::now();

    for i in 49..=51 {
        println!("Graph: {}", i);

        let graph = load_graph(i);

        let vnd = VND::new(
            vec![
                    (Box::new(OneFlip), StepFunction::BestImprovement),
                    (Box::new(MoveVertex), StepFunction::FirstImprovement),
                    (Box::new(NFlip::<3>), StepFunction::FirstImprovement),
                ],
        ); 

        // let grasp = GRASP::new(Box::new(Greedy::new(0.7)), vnd);

        let solution = Greedy::new(0.7).construct(&graph, false);
        println!("Initial solution: {}", solution.cost);

        let start = std::time::Instant::now();
        let best = vnd.run(solution);
        // let best = grasp.run(&graph);
        let elapsed = start.elapsed();

        println!("Best solution: {}", best.cost);
        println!("Elapsed: {:?}", elapsed);


        writeln!(out, "Graph {}", i).unwrap();
        writeln!(out, "{:?}", best).unwrap();
    }


    let elapsed = total.elapsed();

    println!("Total elapsed: {:?}", elapsed);
    println!("NFlip improvements: {}", unsafe { nflip::NUM_IMPROVEMENTS });
    println!("NFlip iterations: {}", unsafe { nflip::I });
}
