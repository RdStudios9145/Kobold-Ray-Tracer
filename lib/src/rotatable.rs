use crate::quaternion::Quaternion;

pub trait Rotatable {
    fn set_orientation(&mut self, _q: Quaternion) { }
    fn get_orientation(&self) -> Quaternion { Quaternion::new(1.0, 0.0, 0.0, 0.0) }
    fn normalize(&mut self) { }
    fn rotate(&mut self, quat: Quaternion) -> &mut Self {
        self.set_orientation(quat * self.get_orientation());
        self.normalize();
        self
    }
}
