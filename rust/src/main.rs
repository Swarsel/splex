mod solution;
mod graph;
mod parser;
mod construction;
mod symmat;

use construction::ConstructionHeuristic;


const INPUT: &str = include_str!("../../instances/test_instances/heur003_n_120_m_2588.txt");
// const INPUT: &str = include_str!("2conncomp.txt");

fn main() {
    let graph = parser::parse(INPUT).unwrap();

    let solution = construction::Greedy::new(0.5).construct(&graph);

    println!("{:?}", graph);

    println!("{:?}", solution);

    let modified_graph = graph::Graph::from_solution(&graph, &solution);

    println!("{:?}", modified_graph);
}
