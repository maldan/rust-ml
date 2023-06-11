use crate::math::la::matrix4::Matrix4x4;
use std::fmt;
use std::ops;

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Print
impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector3({}, {}, {})", self.x, self.y, self.z)
    }
}

// Add Vector3 + Vector3
impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

// Sub Vector3 - Vector3
impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

// Mul Vector3 * f32
impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

// Apply matrix4x4 on Vector3
impl ops::Mul<Matrix4x4> for Vector3 {
    type Output = Vector3;

    fn mul(self, mx: Matrix4x4) -> Vector3 {
        let mut v = Vector3::default();

        let mut w = mx.raw[3] * self.x + mx.raw[7] * self.y + mx.raw[11] * self.z + mx.raw[15];
        if w == 0.0 {
            w = 1.0;
        }
        v.x = (mx.raw[0] * self.x + mx.raw[4] * self.y + mx.raw[8] * self.z + mx.raw[12]) / w;
        v.y = (mx.raw[1] * self.x + mx.raw[5] * self.y + mx.raw[9] * self.z + mx.raw[13]) / w;
        v.z = (mx.raw[2] * self.x + mx.raw[6] * self.y + mx.raw[10] * self.z + mx.raw[14]) / w;

        return v;
    }
}
impl ops::MulAssign<Matrix4x4> for Vector3 {
    fn mul_assign(&mut self, mx: Matrix4x4) {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        let mut w = mx.raw[3] * x + mx.raw[7] * y + mx.raw[11] * z + mx.raw[15];
        if w == 0.0 {
            w = 1.0;
        }
        self.x = (mx.raw[0] * x + mx.raw[4] * y + mx.raw[8] * z + mx.raw[12]) / w;
        self.y = (mx.raw[1] * x + mx.raw[5] * y + mx.raw[9] * z + mx.raw[13]) / w;
        self.z = (mx.raw[2] * x + mx.raw[6] * y + mx.raw[10] * z + mx.raw[14]) / w;
    }
}

// Add Vector3 += Vector3
impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[allow(dead_code)]
impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub const fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub const fn one() -> Vector3 {
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn add_xyz(mut self, x: f32, y: f32, z: f32) -> Vector3 {
        self.x += x;
        self.y += y;
        self.z += z;
        self
    }

    pub fn length(self) -> f32 {
        f32::sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z))
    }

    pub fn normalize(self) -> Vector3 {
        let l = self.length();
        if l == 0.0 {
            return Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
        Vector3 {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }

    pub fn dot(self, v2: Vector3) -> f32 {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }

    pub fn distance_to(self, to: Vector3) -> f32 {
        let a = self.x - to.x;
        let b = self.y - to.y;
        let c = self.z - to.z;
        f32::sqrt(a * a + b * b + c * c)
    }

    pub fn scale(mut self, v: f32) -> Vector3 {
        self.x *= v;
        self.y *= v;
        self.z *= v;
        self
    }

    pub fn mul(mut self, v: Vector3) -> Vector3 {
        self.x *= v.x;
        self.y *= v.y;
        self.z *= v.z;
        self
    }

    pub fn to_degrees(mut self) -> Vector3 {
        self.x = self.x.to_degrees();
        self.y = self.y.to_degrees();
        self.z = self.z.to_degrees();
        self
    }

    pub fn to_radians(mut self) -> Vector3 {
        self.x = self.x.to_radians();
        self.y = self.y.to_radians();
        self.z = self.z.to_radians();
        self
    }

    pub fn from_bytes(b: &[u8]) -> Vector3 {
        Vector3 {
            x: f32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            y: f32::from_le_bytes([b[4], b[5], b[6], b[7]]),
            z: f32::from_le_bytes([b[8], b[9], b[10], b[11]]),
        }
    }
}
