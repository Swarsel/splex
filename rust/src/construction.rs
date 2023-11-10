use crate::{
    graph::Graph,
    solution::{Solution, Vertex, ConnectionComponent},
};

pub trait ConstructionHeuristic {
    fn construct(&self, graph: &Graph) -> Solution;
}

pub struct Greedy {
    threshold: f32,
}

impl ConstructionHeuristic for Greedy {
    fn construct(&self, graph: &Graph) -> Solution {
        // 1. Identify connection components
        // 2. For each component: determine if better to add or remove edges
        // 3. Identify edges with minimum weight to add or remove
        let mut solution = Solution::new(graph);

        solution.vertices = graph
            .adjacency
            .iter()
            .map(|adjacent_to| {
                let degree = adjacent_to.iter().filter(|&connected| *connected).count() as u32;
                Vertex { degree }
            })
            .collect();

        let connected_components = graph.get_connection_components();

        for comp in connected_components.iter() {
            self.repair_component(graph, &mut solution, comp);
        }

        solution
    }
}

impl Greedy {
    pub fn new(threshold: f32) -> Self {
        Self { threshold }
    }

    fn repair_component(&self, graph: &Graph, solution: &mut Solution, comp: &ConnectionComponent) {
        // calculate average degree
        let avg_degree = comp
            .indices
            .iter()
            .map(|&index| solution.vertices[index].degree)
            .sum::<u32>() as f32
            / comp.indices.len() as f32;

        if avg_degree > self.threshold * graph.s as f32 {
            self.add_edges(graph, solution, comp);
        } else {
            self.add_edges(graph, solution, comp)
            // self.remove_edges(graph, solution, &comp.indices);
        }
    }

    fn add_edges(&self, graph: &Graph, solution: &mut Solution, component: &ConnectionComponent) {
        // find edges with minimum weight to add so all degrees are at least comp.len() - s
        let mut edges = vec![];
        let comp = &component.indices;

        for i in 0..comp.len() {
            for j in i + 1..comp.len() {
                // check if edge exists
                if !graph.initial[comp[i]][comp[j]] {
                    let weight = graph.weights[comp[i]][comp[j]];
                    edges.push((weight, comp[i], comp[j]));
                }
            }
        }

        edges.sort_by(|a, b| a.0.cmp(&b.0));

        for (_, i, j) in edges {
            if solution.vertices[i].degree < comp.len() as u32 - graph.s {
                solution.vertices[i].degree += 1;
                solution.vertices[j].degree += 1;
                solution.cost += graph.weights[i][j];
                solution.modified_edges[i][j] = true;
                solution.modified_edges[j][i] = true;
            }
        }
    }

    fn remove_edges(&self, graph: &Graph, solution: &mut Solution, comp: &Vec<usize>) {
        // find edges with minimum weight to remove so all degrees are at most comp.len() - s
        let mut edges = vec![];

        for i in 0..comp.len() {
            for j in i + 1..comp.len() {
                if graph.initial[comp[i]][comp[j]] {
                    let weight = graph.weights[comp[i]][comp[j]];
                    edges.push((weight, comp[i], comp[j]));
                }
            }
        }

        edges.sort_by(|a, b| b.0.cmp(&a.0));

        // remove edges until the component is no longer connected
        todo!("remove edges until component is no longer connected")
    }
}
