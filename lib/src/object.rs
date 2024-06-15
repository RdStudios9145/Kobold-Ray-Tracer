use glm::Vec3;

use crate::Quaternion;

pub type Vertex = [f32; 3];
pub type TriangleIndecies = Vertex;

pub(crate) struct ObjectType {
    verts: Vec<Vertex>,
    tris: Vec<TriangleIndecies>,
    name: String,
}

pub struct ObjectManager {
    registered_objects: Vec<ObjectType>,
}

pub(crate) struct Object {
    // Index into list of all registered object types
    pub(crate) object_type: usize,
    pub position: Vec3,
    pub orientation: Quaternion,
    pub scale: Vec3,
}

impl ObjectManager {
    pub(crate) fn new() -> Self {
        Self {
            registered_objects: Vec::new(),
        }
    }

    pub fn register_object<'a>(
        &mut self,
        name: &'a str,
        verts: &'a [Vertex],
        tris: &'a [TriangleIndecies],
    ) -> usize {
        let id = self.registered_objects.len();
        self.registered_objects.push(ObjectType {
            verts: verts.to_vec(),
            tris: tris.to_vec(),
            name: name.to_string(),
        });
        id
    }
}
