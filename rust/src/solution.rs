use std::fmt::Debug;
use std::fmt::Formatter;

use crate::graph::Graph;

#[derive(Default)]
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

#[derive(Default)]
pub struct Solution {
    pub cost: u32,
    pub vertices: Vec<Vertex>,
    pub connection_components: Vec<ConnectionComponent>,
    pub modified_edges: Vec<Vec<bool>>,
}

impl Solution {
    pub fn new(graph: &Graph) -> Self {
        let mut vertices = vec![];
        let mut modified_edges = vec![];

        for _ in 0..graph.adjacency.len() {
            vertices.push(Vertex::default());
            modified_edges.push(vec![false; graph.adjacency.len()]);
        }

        Self {
            cost: 0,
            vertices,
            connection_components: vec![],
            modified_edges,
        }
    }
}

impl Debug for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "cost: {}", self.cost)?;
        writeln!(f, "connection_components: {:?}", self.connection_components.len())?;

        writeln!(f, "modified_edges:")?;
        for row in self.modified_edges.iter() {
            for col in row.iter() {
                write!(f, "{}", (*col) as u32)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}