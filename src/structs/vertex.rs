use glium::implement_vertex;

/// Single point in 3d space.
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
}

implement_vertex!(Vertex, position);

/// Single 3d vector.
#[derive(Clone, Copy, Debug)]
pub struct Normal {
    pub normal: [f32; 3],
}

implement_vertex!(Normal, normal);