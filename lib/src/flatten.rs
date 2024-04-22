use glm::Mat4;

pub fn flatten(mat: Mat4) -> Vec<f32> {
    // for col in mat.as_array() {
    //     for row in col.as_array() {
    //         out.push(*row);
    //     }
    // }
    // out = mat.as_slice().to_vec();
    mat.as_slice().to_vec()

    // out
}
