use crate::solution::ConnectionComponent;
use itertools::Itertools;

#[derive(Debug)]
pub struct Graph {
    pub s: u32,
    pub adjacency: Vec<Vec<bool>>,
    pub initial: Vec<Vec<bool>>,
    pub weights: Vec<Vec<u32>>,
}


impl Graph {
    pub fn get_connection_components(&self) -> Vec<ConnectionComponent> {
        let mut visited = vec![false; self.adjacency.len()];
        let mut components: Vec<ConnectionComponent> = Vec::new();

        for i in 0..self.adjacency.len() {
            if visited[i] {continue}

            let component = ConnectionComponent { indices: self.get_connected_to(i, Vec::new()).into_iter().unique().collect::<Vec<_>>()};

            for index in component.indices.iter() {
                visited[*index] = true;
            }

            components.push(component);
        }

        components
    }

    fn get_connected_to(&self, id: usize, mut already_visited: Vec<usize>) -> Vec<usize> {
        let mut connected = vec![id];

        for adjacent in self.adjacent(id) {
            if already_visited.contains(&adjacent) {
                continue;
            }
            
            already_visited.push(id);
            
            let conn = self.get_connected_to(adjacent, already_visited.clone());

            already_visited.extend(conn.iter());
            connected.extend(conn.iter());
        }

        connected
    }

    fn adjacent(&self, id: usize) -> Vec<usize> {
        self.adjacency[id].iter().enumerate().filter_map(|(index, connected)| {
            if *connected {
                return Some(index);
            }
            None
        }).collect()
    }
}