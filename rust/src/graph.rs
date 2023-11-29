use crate::solution::ConnectionComponent;
use crate::symmat::SymMat;
use std::fmt::{Debug, Formatter};

pub struct Graph {
    pub s: u32,
    pub initial: SymMat<bool>,
    pub weights: SymMat<u32>,
}

impl Debug for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.initial.print_block(f)
    }
}

impl Graph {
    pub fn get_connection_components(adjacency: &SymMat<bool>) -> Vec<ConnectionComponent> {
        let mut components = vec![];
        let mut visited = vec![false; adjacency.len()];
        let mut stack = vec![];

        for index in 0..adjacency.len() {
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

                for adjacent in Self::adjacent(adjacency, id) {
                    stack.push(adjacent);
                }
            }

            components.push(ConnectionComponent::new(component));
        }

        components
    }

    pub fn get_connection_component_including(
        adjacency: &SymMat<bool>,
        vertices: &Vec<usize>,
    ) -> Vec<ConnectionComponent> {
        let mut components = vec![];
        let mut visited = vec![false; adjacency.len()];
        let mut stack = vec![];

        for index in vertices {
            if visited[*index] {
                continue;
            }

            stack.push(*index);

            let mut component = vec![];

            while let Some(id) = stack.pop() {
                if visited[id] {
                    continue;
                }

                visited[id] = true;
                component.push(id);

                for adjacent in Self::adjacent(adjacency, id) {
                    stack.push(adjacent);
                }
            }

            components.push(ConnectionComponent::new(component));
        }

        components
    }

    fn adjacent(adjacency: &SymMat<bool>, id: usize) -> Vec<usize> {
        let mut result = vec![];

        for index in 0..adjacency.len() {
            if *adjacency.get(id, index) {
                result.push(index);
            }
        }

        result
    }
}
