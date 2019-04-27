use std::ops::{Add, Div, Mul, Neg, Sub};
use std::cmp;
use std::fmt;

#[derive(Clone,Copy,Default)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        return Vec3 { x:0., y:0., z:0. }
    }

    pub fn one() -> Vec3 {
        return Vec3 { x:1., y:1., z:1. }
    }

    pub fn lenSquared(&self) -> f64 {
        return self.x*self.x + self.y * self.y + self.z * self.z;
    }

    pub fn len(&self) -> f64 {
        return self.lenSquared().sqrt();
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.len();

        return Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len
        };
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f64) -> Vec3 {
        return Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other
        };
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        };
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f64) -> Vec3 {
        return Vec3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other
        };
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        return Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        };
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        };
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        return Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        };
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    
    fn neg(self) -> Vec3 {
        return Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        };
    }
}

impl cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        return self.x == other.x 
            && self.y == other.y
            && self.z == other.z;
    }
}

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({} {} {})", self.x, self.y, self.z);
    }
}