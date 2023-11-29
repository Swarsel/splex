use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::{
    graph::Graph,
    solution::{ConnectionComponent, Solution, Vertex},
};

pub trait ConstructionHeuristic<'a> {
    fn construct(&self, graph: &'a Graph, random: bool) -> Solution<'a>;
}

pub struct Greedy {
    threshold: f32,
}

impl<'a> ConstructionHeuristic<'a> for Greedy {
    fn construct(&self, graph: &'a Graph, random: bool) -> Solution<'a> {
        // 1. Identify connection components
        // 2. For each component: determine if better to add or remove edges
        // 3. Identify edges with minimum weight to add or remove
        let mut solution = Solution::new(graph);

        solution.vertices = (0..graph.initial.len())
            .map(|index| {
                let degree = graph
                    .initial
                    .get_col(index)
                    .iter()
                    .filter(|&connected| *connected)
                    .count() as u32;

                Vertex { degree }
            })
            .collect();

        let mut connected_components = Graph::get_connection_components(&graph.initial);

        // implement random construction heuristic by shuffling the connected components, resulting in a random order of repairing
        if random == true {
            connected_components.shuffle(&mut thread_rng());
        }
        for comp in connected_components.iter() {
            self.repair_component(graph, &mut solution, comp);
        }

        solution.connection_components = Graph::get_connection_components(&solution.edges);

        solution
    }
}

impl Greedy {
    pub fn new(threshold: f32) -> Self {
        Self { threshold }
    }

    fn repair_component(&self, graph: &Graph, solution: &mut Solution, comp: &ConnectionComponent) {
        if solution.is_connection_component_splex(comp, graph.s) {
            return;
        }

        // calculate average degree
        let avg_degree = comp
            .indices
            .iter()
            .map(|&index| solution.vertices[index].degree)
            .sum::<u32>() as f32
            / comp.indices.len() as f32;

        let required_degree = comp.indices.len() as u32 - graph.s;

        if avg_degree > self.threshold * required_degree as f32 {
            self.add_edges(graph, solution, comp);
        } else {
            self.remove_edges(graph, solution, &comp.indices);

            // check if component has split
            let components =
                Graph::get_connection_component_including(&solution.edges, &comp.indices);

            for comp in components {
                self.repair_component(graph, solution, &comp);
            }
        }
    }

    fn add_edges(&self, graph: &Graph, solution: &mut Solution, component: &ConnectionComponent) {
        // find edges with minimum weight to add so all degrees are at least comp.len() - s
        let mut edges = vec![];
        let comp = &component.indices;

        for i in 0..comp.len() {
            for j in i + 1..comp.len() {
                // check if edge exists
                if !*solution.edges.get(comp[i], comp[j]) {
                    let weight = graph.weights.get(comp[i], comp[j]);
                    edges.push((weight, comp[i], comp[j]));
                }
            }
        }

        edges.sort_by(|a, b| a.0.cmp(&b.0));

        let min_degree = comp.len() as u32 - graph.s;

        for (_, i, j) in edges {
            if solution.vertices[i].degree < min_degree {
                solution.add_edge(i, j);
            }
        }
    }

    fn remove_edges(&self, graph: &Graph, solution: &mut Solution, comp: &Vec<usize>) {
        // find vertex with minimum degree >= 2
        let mut min_degree_index = None;
        let mut min_degree = u32::MAX;

        for index in comp {
            if solution.vertices[*index].degree < min_degree && solution.vertices[*index].degree > 1
            {
                min_degree_index = Some(*index);
                min_degree = solution.vertices[*index].degree;
            }
        }

        // remove all but the most expensive edge
        if let Some(min_degree_index) = min_degree_index {
            let mut edges = vec![];

            for i in 0..solution.edges.len() {
                if *solution.edges.get(min_degree_index, i) {
                    edges.push((graph.weights.get(min_degree_index, i), min_degree_index, i));
                }
            }

            edges.sort_by(|a, b| b.0.cmp(&a.0));
            edges.pop();

            for (_, i, j) in edges {
                solution.remove_edge(i, j);
            }
        }
    }
}
