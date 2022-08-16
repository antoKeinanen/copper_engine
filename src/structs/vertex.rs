#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
}

implement_vertex!(Vertex, position);

#[derive(Clone, Copy, Debug)]
pub struct Normal {
    pub normal: [f32; 3],
}

implement_vertex!(Normal, normal);