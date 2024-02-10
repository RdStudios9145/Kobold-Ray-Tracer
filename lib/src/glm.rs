use glm::Matrix4;

pub fn flatten(mat: Matrix4<f32>) -> Vec<f32> {
    let mut out = vec![0.0; 16];

    for col in mat.as_array() {
        for row in col.as_array() {
            out.push(*row);
        }
    }

    out
}
