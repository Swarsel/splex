use crate::solution::ConnectionComponent;

#[derive(Debug)]
pub struct Graph {
    pub s: u32,
    pub adjacency: Vec<Vec<bool>>,
    pub initial: Vec<Vec<bool>>,
    pub weights: Vec<Vec<u32>>,
}


impl Graph {
    pub fn get_connection_components(&self) -> Vec<ConnectionComponent> {
        // find the connected components
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
        self.adjacency[id].iter().enumerate().filter_map(|(index, connected)| {
            if *connected {
                return Some(index);
            }
            None
        }).collect()
    }
}