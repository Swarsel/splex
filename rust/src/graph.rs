use crate::solution::{ConnectionComponent, Solution};
use std::fmt::{Debug, Formatter};

pub struct Graph {
    pub s: u32,
    pub adjacency: Vec<Vec<bool>>,
    pub initial: Vec<Vec<bool>>,
    pub weights: Vec<Vec<u32>>,
}

impl Debug for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // output adjacency as 0/1 matrix
        for row in self.adjacency.iter() {
            for col in row.iter() {
                if *col {
                    write!(f, "█")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
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
        self.adjacency[id]
            .iter()
            .enumerate()
            .filter_map(|(index, connected)| {
                if *connected {
                    return Some(index);
                }
                None
            })
            .collect()
    }

    pub fn from_solution(graph: &Graph, solution: &Solution) -> Self {
        let mut adjacency = vec![vec![false; graph.adjacency.len()]; graph.adjacency.len()];

        for i in 0..graph.adjacency.len() {
            for j in 0..graph.adjacency.len() {
                adjacency[i][j] = graph.adjacency[i][j] ^ solution.modified_edges[i][j];
                adjacency[j][i] = adjacency[i][j];
            }
        }

        Self {
            s: graph.s,
            adjacency,
            initial: graph.initial.clone(),
            weights: graph.weights.clone(),
        }
    }
}
