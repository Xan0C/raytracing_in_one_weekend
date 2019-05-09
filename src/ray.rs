use super::vec;

pub struct Ray {
    pub a: vec::Vec3,
    pub b: vec::Vec3
}

impl Ray {
    pub fn origin(&self) -> vec::Vec3 {
        return self.a;
    }

    pub fn direction(&self) -> vec::Vec3 {
        return self.b;
    }

    pub fn point_at_parameter(&self, t: f64) -> vec::Vec3 {
        return self.a + self.b * t;
    }
}