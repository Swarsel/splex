use std::fmt::Debug;
use std::fmt::Formatter;

use crate::graph::Graph;
use crate::symmat::SymMat;

#[derive(Default, Clone, Copy, Debug)]
pub struct Vertex {
    pub degree: u32,
}

#[derive(Default, Debug, Clone)]
pub struct ConnectionComponent {
    pub indices: Vec<usize>,
}

impl ConnectionComponent {
    pub fn new(mut indices: Vec<usize>) -> Self {
        indices.sort();
        Self { indices }
    }
}

#[derive(Clone)]
pub struct Solution<'a> {
    pub cost: u32,
    pub vertices: Vec<Vertex>,
    pub edges: SymMat<bool>,
    pub connection_components: Vec<ConnectionComponent>,
    pub graph: &'a Graph,
}

impl<'a> Solution<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        let vertices = vec![Vertex::default(); graph.initial.len()];

        Self {
            cost: 0,
            vertices,
            connection_components: vec![],
            edges: graph.initial.clone(),
            graph,
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

        component
            .indices
            .iter()
            .all(|&index| self.vertices[index].degree >= required_degree)
    }

    pub fn recalculate_connection_components(&mut self) {
        self.connection_components = Graph::get_connection_components(&self.edges);
    }

    pub fn remove_edge(&mut self, row: usize, col: usize) {
        self.vertices[row].degree -= 1;
        self.vertices[col].degree -= 1;

        if *self.graph.initial.get(row, col) {
            self.cost += self.graph.weights.get(row, col);
        } else {
            self.cost -= self.graph.weights.get(row, col);
        }

        self.edges.set(row, col, false);
    }

    pub fn add_edge(&mut self, row: usize, col: usize) {
        self.vertices[row].degree += 1;
        self.vertices[col].degree += 1;

        if *self.graph.initial.get(row, col) {
            self.cost -= self.graph.weights.get(row, col);
        } else {
            self.cost += self.graph.weights.get(row, col);
        }

        self.edges.set(row, col, true);
    }

    pub fn flip_edge(&mut self, row: usize, col: usize) {
        if *self.edges.get(row, col) {
            self.remove_edge(row, col);
        } else {
            self.add_edge(row, col);
        }
    }

    pub fn is_valid(&self) -> bool {
        self.connection_components
            .iter()
            .all(|comp| self.is_connection_component_splex(comp, self.graph.s))
    }
}

impl<'a> Debug for Solution<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "cost: {}", self.cost)?;
        writeln!(
            f,
            "connection_components: {:?}",
            self.connection_components.len()
        )?;

        writeln!(f, "edges:")?;
        self.edges.print_block(f)?;
        Ok(())
    }
}
