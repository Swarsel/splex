#[derive(Default)]
pub struct Vertex {

}

#[derive(Default, Debug)]
pub struct ConnectionComponent {
    pub indices: Vec<usize>
}

#[derive(Default)]
pub struct Solution {
    pub cost: u32,
    pub vertices: Vec<Vertex>,
    pub connection_components: Vec<ConnectionComponent>,
}