use rand::seq::SliceRandom;
use rand::Rng;

use crate::{
    graph::Graph,
    solution::{ConnectionComponent, Solution, Vertex},
};

pub trait ConstructionHeuristic<'a> {
    fn construct(&self, graph: &'a Graph) -> Solution<'a>;
}

pub struct Greedy {
    threshold: f32,
    random: bool,
}

impl<'a> ConstructionHeuristic<'a> for Greedy {
    fn construct(&self, graph: &'a Graph) -> Solution<'a> {
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

        let connected_components = Graph::get_connection_components(&graph.initial);

        for comp in connected_components.iter() {
            self.repair_component(graph, &mut solution, comp);
        }

        solution.connection_components = Graph::get_connection_components(&solution.edges);

        solution
    }
}

impl Greedy {
    pub fn new(threshold: f32, random: bool) -> Self {
        Self { threshold, random }
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

        if avg_degree > self.threshold * required_degree as f32 || (self.random && rand::thread_rng().gen_range(0..100) < 2) {
            self.add_edges(graph, solution, comp);
        } else {
            self.remove_edges(solution, &comp.indices);

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
            if solution.vertices[i].degree < min_degree || solution.vertices[j].degree < min_degree {
                solution.add_edge(i, j);
            }
        }
    }

    fn remove_edges(&self, solution: &mut Solution, comp: &Vec<usize>) {
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

        // remove all edges
        if let Some(min_degree_index) = min_degree_index {
            for i in 0..solution.edges.len() {
                if *solution.edges.get(min_degree_index, i) {
                    solution.flip_edge(min_degree_index, i);
                }
            }
        }
    }
}
