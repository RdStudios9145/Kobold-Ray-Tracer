use crate::quaternion::Quaternion;

pub type Vertex = [f32; 3];
pub type TriIndexes = [u32; 3];

#[derive(Debug)]
pub enum ObjectType {
    Custom,
    Sphere,
    Cube,
}

#[derive(Debug)]
pub struct Object {
    pub(super) verts: Vec<Vertex>,
    pub(super) indicies: Vec<TriIndexes>,
    pub(super) vao_id: u32,
    pub(super) vbo_id: u32,
    pub(super) ebo_id: u32,
    pub(super) ids_generated: bool,
    pub(super) orientation: Quaternion,
    pub obj_type: ObjectType,
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
            orientation: Quaternion::new(0.0, 0.0, 0.0, 0.0),
            obj_type: ObjectType::Custom,
        }
    }

    pub fn new_sphere(center: glm::Vec3, radius: f32) -> Self {
        let t = (1. + (5_f32).sqrt()) / 2.;
        let initial_verts = vec![
            [-1.,  t,  0.],
            [ 1.,  t,  0.],
            [-1., -t,  0.],
            [ 1., -t,  0.],

            [ 0., -1.,  t],
            [ 0.,  1.,  t],
            [ 0., -1., -t],
            [ 0.,  1., -t],

            [ t,  0., -1.],
            [ t,  0.,  1.],
            [-t,  0., -1.],
            [-t,  0.,  1.]
        ];

        let mut verts = Vec::new();
        for vert in initial_verts.iter() {
            verts.push([vert[0] * radius + center.x, vert[1] * radius + center.y, vert[2] * radius + center.z]);
        }

        let indicies = vec![
            [0, 11, 5],
            [0, 5, 1],
            [0, 1, 7],
            [0, 7, 10],
            [0, 10, 11],
            [1, 5, 9],
            [5, 11, 4],
            [11, 10, 2],
            [10, 7, 6],
            [7, 1, 8],
            [3, 9, 4],
            [3, 4, 2],
            [3, 2, 6],
            [3, 6, 8],
            [3, 8, 9],
            [4, 9, 5],
            [2, 4, 11],
            [6, 2, 10],
            [8, 6, 7],
            [9, 8, 1],
        ];

        Self {
            verts,
            indicies,
            vao_id: 0,
            vbo_id: 0,
            ebo_id: 0,
            ids_generated: false,
            orientation: Quaternion::new(0.0, 0.0, 0.0, 0.0),
            obj_type: ObjectType::Sphere,
        }
    }

    pub fn new_cube(center: glm::Vec3, size: glm::Vec3) -> Self {
        let og_verts = vec![
            [-0.5,  0.5, -0.5],
            [ 0.5,  0.5, -0.5],
            [ 0.5, -0.5, -0.5],
            [-0.5, -0.5, -0.5],
            [-0.5,  0.5,  0.5],
            [ 0.5,  0.5,  0.5],
            [ 0.5, -0.5,  0.5],
            [-0.5, -0.5,  0.5],
        ];

        let mut verts = Vec::new();

        for vert in og_verts.iter() {
            verts.push([vert[0] * size.x + center.x, vert[1] * size.y + center.y, vert[2] * size.z + center.z]);
        }

        println!("{:?}", &verts);

        let indicies = vec![[0, 1, 3], [1, 2, 3], [0, 7, 4], [0, 3, 7], [5, 4, 6], [4, 7, 6], [1, 5, 2], [5, 6, 2], [4, 5, 0], [5, 1, 0], [3, 2, 7], [2, 6, 7]];

        Self {
            verts,
            indicies,
            vao_id: 0,
            vbo_id: 0,
            ebo_id: 0,
            ids_generated: false,
            orientation: Quaternion::new(0.0, 0.0, 0.0, 0.0),
            obj_type: ObjectType::Cube,
        }
    }
}
