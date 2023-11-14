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
    pub connection_components: Vec<ConnectionComponent>,
    pub modified_edges: SymMat<bool>,
}

impl Solution {
    pub fn new(graph: &Graph) -> Self {
        let vertices = vec![Vertex::default(); graph.adjacency.len()];

        Self {
            cost: 0,
            vertices,
            connection_components: vec![],
            modified_edges: SymMat::new(graph.adjacency.len()),
        }
    }
}

impl Debug for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "cost: {}", self.cost)?;
        writeln!(f, "connection_components: {:?}", self.connection_components.len())?;

        writeln!(f, "modified_edges:")?;
        self.modified_edges.print_block(f)?;
        Ok(())
    }
}