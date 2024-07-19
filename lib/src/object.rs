use glm::Vec3;
use glm::Vec4;

use crate::Quaternion;

use crate::r#macro;

r#macro::use_backend!(ObjectInformation);

pub type Vertex = [f32; 3];
pub type TriangleIndecies = [i32; 3];

#[allow(non_snake_case)]
pub mod Primitive {
    pub static SPHERE: usize = 0;
    pub static CUBE: usize = 1;
}

#[derive(Debug)]
pub(crate) struct ObjectType {
    verts: Vec<Vertex>,
    pub(crate) tris: Vec<TriangleIndecies>,
    name: String,
    pub(crate) info: ObjectInformation,
}

#[derive(Debug)]
pub struct ObjectManager {
    registered_objects: Vec<ObjectType>,
}

pub(crate) struct Object {
    // Index into list of all registered object types
    pub(crate) object_type: usize,
    pub position: Vec3,
    pub orientation: Quaternion,
    pub scale: Vec3,
    pub color: Vec4,
}

impl ObjectManager {
    pub(crate) fn new() -> Self {
        let registered_objects = vec![Self::generate_sphere(), Self::generate_cube()];

        Self { registered_objects }
    }

    pub fn register_object<'a>(
        &mut self,
        name: &'a str,
        verts: &'a [Vertex],
        tris: &'a [TriangleIndecies],
    ) -> Option<usize> {
        let id = self.registered_objects.len();
        let info = ObjectInformation::new((verts, tris))?;

        self.registered_objects.push(ObjectType {
            verts: verts.to_vec(),
            tris: tris.to_vec(),
            name: name.to_string(),
            info,
        });

        Some(id)
    }

    pub(crate) fn from_id(&self, id: usize) -> &ObjectType {
        &self.registered_objects[id]
    }

    fn generate_sphere() -> ObjectType {
        let t = (1. + (5_f32).sqrt()) / 2.;
        let verts = vec![
            [-1., t, 0.],
            [1., t, 0.],
            [-1., -t, 0.],
            [1., -t, 0.],
            [0., -1., t],
            [0., 1., t],
            [0., -1., -t],
            [0., 1., -t],
            [t, 0., -1.],
            [t, 0., 1.],
            [-t, 0., -1.],
            [-t, 0., 1.],
        ];

        let tris = vec![
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

        let info =
            ObjectInformation::new((&verts, &tris)).expect("Unable to generate primitive 'SPHERE'");

        ObjectType {
            name: "sphere".to_string(),
            verts,
            tris,
            info,
        }
    }

    fn generate_cube() -> ObjectType {
        ObjectType {
            name: "cube".to_string(),
            verts: vec![],
            tris: vec![],
            info: ObjectInformation::new((&[], &[])).expect("Unable to generate primitive 'CUBE'"),
        }
    }
}
