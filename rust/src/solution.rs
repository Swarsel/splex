use std::fmt::Debug;
use std::fmt::Formatter;

use crate::graph::Graph;
use crate::symmat::SymMat;

#[derive(Default, Clone, Copy, Debug)]
pub struct Vertex {
    pub degree: u32,
}

#[derive(Default, Debug)]
pub struct ConnectionComponent {
    pub indices: Vec<usize>
}

impl ConnectionComponent {
    pub fn new(mut indices: Vec<usize>) -> Self {
        indices.sort();
        Self { indices }
    }
}


pub struct Solution {
    pub cost: u32,
    pub vertices: Vec<Vertex>,
    pub edges: SymMat<bool>,
    pub connection_components: Vec<ConnectionComponent>,
}

impl Solution {
    pub fn new(graph: &Graph) -> Self {
        let vertices = vec![Vertex::default(); graph.initial.len()];

        Self {
            cost: 0,
            vertices,
            connection_components: vec![],
            edges: graph.initial.clone(),
        }
    }

    pub fn is_connection_component_splex(&self, component: &ConnectionComponent, s: u32) -> bool {
        if component.indices.len() == 0 {
            println!("empty component");
            return true;
        }

        if component.indices.len() == 1 {
            return self.vertices[component.indices[0]].degree == 0;
        }

        let required_degree = component.indices.len() as i32 - s as i32;

        if required_degree < 0 {
            return true;
        }

        let required_degree = required_degree as u32;

        component.indices
            .iter()
            .all(|&index| self.vertices[index].degree >= required_degree)
    }
}

impl Debug for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "cost: {}", self.cost)?;
        writeln!(f, "connection_components: {:?}", self.connection_components.len())?;

        writeln!(f, "edges:")?;
        self.edges.print_block(f)?;
        Ok(())
    }
}