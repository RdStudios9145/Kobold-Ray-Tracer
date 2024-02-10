pub type Vertex = [f32; 3];
pub type TriIndexes = [u32; 3];

#[derive(Debug)]
pub struct Object {
    pub(super) verts: Vec<Vertex>,
    pub(super) indicies: Vec<TriIndexes>,
    pub(super) vao_id: u32,
    pub(super) vbo_id: u32,
    pub(super) ebo_id: u32,
    pub(super) ids_generated: bool,
}

impl Object {
    pub fn new(verts: Vec<Vertex>, tris: Vec<TriIndexes>) -> Self {
        Self {
            verts,
            indicies: tris,
            vao_id: 0,
            vbo_id: 0,
            ebo_id: 0,
            ids_generated: false,
        }
    }

    pub fn new_circle(center: glm::Vec3, radius: u32) -> Self {
        Self {
            verts: Vec::new(),
            indicies: Vec::new(),
            vao_id: 0,
            vbo_id: 0,
            ebo_id: 0,
            ids_generated: false,
        }
    }

    pub fn new_cube(center: glm::Vec3, size: glm::Vec3) -> Self {
        Self {
            verts: Vec::new(),
            indicies: Vec::new(),
            vao_id: 0,
            vbo_id: 0,
            ebo_id: 0,
            ids_generated: false,
        }
    }
}
