use crate::solution::{ConnectionComponent, Solution};
use crate::symmat::SymMat;
use std::fmt::{Debug, Formatter};

pub struct Graph {
    pub s: u32,
    pub adjacency: SymMat<bool>,
    pub initial: SymMat<bool>,
    pub weights: SymMat<u32>,
}

impl Debug for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.adjacency.print_block(f)
    }
}

impl Graph {
    pub fn get_connection_components(&self) -> Vec<ConnectionComponent> {
        let mut components = vec![];
        let mut visited = vec![false; self.adjacency.len()];
        let mut stack = vec![];

        // for (index, _) in self.adjacency.iter().enumerate() {
        for index in 0..self.adjacency.len() {
            if visited[index] {
                continue;
            }

            stack.push(index);

            let mut component = vec![];

            while let Some(id) = stack.pop() {
                if visited[id] {
                    continue;
                }

                visited[id] = true;
                component.push(id);

                for adjacent in self.adjacent(id) {
                    stack.push(adjacent);
                }
            }

            components.push(ConnectionComponent::new(component));
        }

        components
    }

    fn adjacent(&self, id: usize) -> Vec<usize> {
        let mut result = vec![];

        for index in 0..self.adjacency.len() {
            if *self.adjacency.get(id, index) {
                result.push(index);
            }
        }

        result
    }

    pub fn from_solution(graph: &Graph, solution: &Solution) -> Self {
        let adjacency = &graph.adjacency ^ &solution.modified_edges;

        Self {
            s: graph.s,
            adjacency,
            initial: graph.initial.clone(),
            weights: graph.weights.clone(),
        }
    }
}
