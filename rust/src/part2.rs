#![allow(dead_code)]
#![allow(unused_imports)]

mod construction;
mod graph;
mod grasp;
mod gvns;
mod hybrid;
mod neighborhood;
mod parser;
mod solution;
mod stoppingcriterion;
mod symmat;
mod vnd;

use std::fs;
use std::io::Write;
use std::time::Duration;

use hybrid::Hybrid;

use clap::Parser;

use crate::neighborhood::stepfunction::StepFunction;

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

#[derive(Parser, Debug)]
struct Args {
    /// Graph ID to load
    #[arg(short, long)]
    graph_id: usize,

    /// one_flip_step_function
    #[arg(short, long, default_value = "best_improvement")]
    one_flip_step_function: Option<String>,

    /// 2_flip_step_function
    #[arg(short, long, default_value = "best_improvement")]
    two_flip_step_function: Option<String>,

    /// phase 1 duration as seconds
    #[arg(short, long, default_value = "60")]
    phase1_duration: u64,

    /// log file
    #[arg(short, long, default_value = "out.txt")]
    log_file: String,

    /// automated run
    #[arg(short, long, default_value = "false")]
    automated: bool,
}

fn main() {
    let args: Args = Args::parse();

    let mut out = fs::File::create(args.log_file).unwrap();

    let total = std::time::Instant::now();

    let graph = load_graph(args.graph_id);

    let mut hybrid = Hybrid::new(
        graph,
        match args.one_flip_step_function.unwrap_or("".into()).as_str() {
            "best_improvement" => StepFunction::BestImprovement,
            "first_improvement" => StepFunction::FirstImprovement,
            "" | "skip" => StepFunction::Skip,
            s => panic!("invalid step function: '{s}'"),
        },
        match args.two_flip_step_function.unwrap_or("".into()).as_str() {
            "best_improvement" => StepFunction::BestImprovement,
            "first_improvement" => StepFunction::FirstImprovement,
            "" | "skip" => StepFunction::Skip,
            s => panic!("invalid step function: '{s}'"),
        },
        Duration::from_secs(args.phase1_duration),
    );

    if !args.automated {
        println!("running hybrid for graph {}", args.graph_id);
    }

    let solution = hybrid.run();

    if !args.automated {
        println!("{}: {}", args.graph_id, solution.cost);
        println!("total time: {:?}", total.elapsed());
    }

    writeln!(out, "{}: {}", args.graph_id, solution.cost).unwrap();
    writeln!(out, "{:?}", solution).unwrap();

    if args.automated {
        println!("{}", solution.cost);
    }
}
