use crate::math::la::matrix4::Matrix4x4;
use crate::math::la::vector3::Vector3;
use std::fmt;
use std::ops;

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

// Print
impl fmt::Display for Vector4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector4({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

// Add Vector4 + Vector4
impl ops::Add<Vector4> for Vector4 {
    type Output = Vector4;

    fn add(self, _rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w: self.w + _rhs.w,
        }
    }
}

// Sub Vector4 - Vector4
impl ops::Sub<Vector4> for Vector4 {
    type Output = Vector4;

    fn sub(self, _rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            w: self.w - _rhs.w,
        }
    }
}

// Mul Vector4 * f32
impl ops::Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, _rhs: f32) -> Vector4 {
        Vector4 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
            w: self.w * _rhs,
        }
    }
}

#[allow(dead_code)]
impl Vector4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 { x, y, z, w }
    }

    pub const fn zero() -> Vector4 {
        Vector4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn length(self) -> f32 {
        f32::sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w))
    }

    pub fn normalize(self) -> Vector4 {
        let l = self.length();
        if l == 0.0 {
            return Vector4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            };
        }
        Vector4 {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
            w: self.w / l,
        }
    }

    pub fn scale(mut self, v: f32) -> Vector4 {
        self.x *= v;
        self.y *= v;
        self.z *= v;
        self.w *= v;
        self
    }

    pub fn mul(mut self, v: Vector4) -> Vector4 {
        self.x *= v.x;
        self.y *= v.y;
        self.z *= v.z;
        self.w *= v.w;
        self
    }

    pub fn lerp(a: Vector4, b: Vector4, t: f32) -> Vector4 {
        let x = a.x + (b.x - a.x) * t;
        let y = a.y + (b.y - a.y) * t;
        let z = a.z + (b.z - a.z) * t;
        let w = a.w + (b.w - a.w) * t;
        Vector4 { x, y, z, w }
    }

    pub fn from_bytes(b: &[u8]) -> Vector4 {
        Vector4 {
            x: f32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            y: f32::from_le_bytes([b[4], b[5], b[6], b[7]]),
            z: f32::from_le_bytes([b[8], b[9], b[10], b[11]]),
            w: f32::from_le_bytes([b[12], b[13], b[14], b[15]]),
        }
    }

    pub fn from_vector3(v: Vector3) -> Vector4 {
        Vector4::new(v.x, v.y, v.z, 0.0)
    }
}
