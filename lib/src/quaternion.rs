use glm::{ Mat4, Mat3, Vec3 };
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
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

    pub fn from_two(l: f32, v: Vec3) -> Self {
        let mut ret = Self {
            l,
            i: v.x,
            j: v.y,
            k: v.z,
        };

        ret.normalize();
        ret
    }

    pub fn to_matrix3(&self) -> Mat3 {
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

        Mat3::new(r00, r01, r02,
            r10, r11, r12,
            r20, r21, r22)
    }

    pub fn to_matrix(&self) -> Mat4 {
        let mat = self.to_matrix3();

        Mat4::new(mat[(0, 0)], mat[(1, 0)], mat[(2, 0)], 0.0,
            mat[(0, 1)], mat[(1, 1)], mat[(2, 1)], 0.0,
            mat[(0, 2)], mat[(1, 2)], mat[(2, 2)], 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    pub fn normalize(&mut self) -> &mut Self {
        let mag = (self.i * self.i + self.j * self.j + self.k * self.k + self.l * self.l).sqrt();
        self.i /= mag;
        self.j /= mag;
        self.k /= mag;
        self.l /= mag;
        self
    }

    pub fn from_euler(roll: f32, pitch: f32, yaw: f32) -> Self {
        let cr = (roll / 2.0).cos();
        let sr = (roll / 2.0).sin();
        let cp = (pitch / 2.0).cos();
        let sp = (pitch / 2.0).sin();
        let cy = (yaw / 2.0).cos();
        let sy = (yaw / 2.0).sin();

        Self {
            l: cr * cp * cy + sr * sp * sy,
            i: sr * cp * cy - cr * sp * sy,
            j: cr * sp * cy + sr * cp * sy,
            k: cr * cp * sy - sr * sp * cy,
        }
    }
}

impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            l: self.l * rhs.l - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
            i: self.l * rhs.i + self.i * rhs.l + self.j * rhs.k - self.k * rhs.j,
            j: self.l * rhs.j - self.i * rhs.k + self.j * rhs.l + self.k * rhs.i,
            k: self.l * rhs.k + self.i * rhs.j - self.j * rhs.i + self.k * rhs.l,
        }
    }
}
