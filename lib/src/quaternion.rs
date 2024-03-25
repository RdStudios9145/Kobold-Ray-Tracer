use glm::Mat4;

#[derive(Debug, Clone)]
pub struct Quaternion {
    i: f32,
    j: f32,
    k: f32,
    l: f32,
}

impl Quaternion {
    pub fn new(i: f32, j: f32, k: f32, l: f32) -> Self {
        Self {
            i, j, k, l
        }
    }

    // pub fn from

    pub fn to_matrix(&self) -> Mat4 {
        let q0 = self.l;
        let q1 = self.i;
        let q2 = self.j;
        let q3 = self.k;

        let r00 = 2.0 * (q0 * q0 + q1 * q1) - 1.0;
        let r01 = 2.0 * (q1 * q2 - q0 * q3);
        let r02 = 2.0 * (q1 * q3 + q0 * q2);

        let r10 = 2.0 * (q1 * q2 + q0 * q3);
        let r11 = 2.0 * (q0 * q0 + q2 * q2) - 1.0;
        let r12 = 2.0 * (q2 * q3 - q0 * q1);

        let r20 = 2.0 * (q1 * q3 - q0 * q2);
        let r21 = 2.0 * (q2 * q3 + q0 * q1);
        let r22 = 2.0 * (q0 * q0 + q3 * q3) - 1.0;

        Mat4::new(r00, r01, r02, 0.0,
            r10, r11, r12, 0.0,
            r20, r21, r22, 0.0,
            0.0, 0.0, 0.0, 1.0)
    }
}
