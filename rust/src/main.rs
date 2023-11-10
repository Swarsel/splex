mod solution;
mod graph;
mod parser;
mod construction;


const INPUT: &str = include_str!("../../instances/test_instances/heur003_n_120_m_2588.txt");

fn main() {
    let graph = parser::parse(INPUT).unwrap();

    let connection_components = graph.get_connection_components();

    for comp in connection_components.iter() {
        println!("{}", comp.indices.iter().map(|v| v.to_string() + " ").collect::<String>());
    }
}
