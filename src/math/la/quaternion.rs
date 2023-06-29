use crate::math::la::matrix4::Matrix4x4;
use crate::math::la::vector3::Vector3;
use std::{fmt, ops};

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

// Print
impl fmt::Display for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Quaternion({}, {}, {}, {})",
            self.x, self.y, self.z, self.w
        )
    }
}

// Add Quaternion + Quaternion
impl ops::Add<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn add(self, _rhs: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w: self.w + _rhs.w,
        }
    }
}

// Add Quaternion - Quaternion
impl ops::Sub<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn sub(self, _rhs: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            w: self.w - _rhs.w,
        }
    }
}

// Add Quaternion * Quaternion
impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, _rhs: Quaternion) -> Quaternion {
        let w1 = self.w;
        let x1 = self.x;
        let y1 = self.y;
        let z1 = self.z;

        let w2 = _rhs.w;
        let x2 = _rhs.x;
        let y2 = _rhs.y;
        let z2 = _rhs.z;

        Quaternion {
            w: w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2,
            x: w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2,
            y: w1 * y2 + y1 * w2 + z1 * x2 - x1 * z2,
            z: w1 * z2 + z1 * w2 + x1 * y2 - y1 * x2,
        }
    }
}

// Add Quaternion * Quaternion
impl ops::MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, other: Quaternion) {
        let w1 = self.w;
        let x1 = self.x;
        let y1 = self.y;
        let z1 = self.z;

        let w2 = other.w;
        let x2 = other.x;
        let y2 = other.y;
        let z2 = other.z;

        *self = Self {
            w: w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2,
            x: w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2,
            y: w1 * y2 + y1 * w2 + z1 * x2 - x1 * z2,
            z: w1 * z2 + z1 * w2 + x1 * y2 - y1 * x2,
        }
    }
}

#[allow(dead_code)]
impl Quaternion {
    pub const fn new() -> Quaternion {
        Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub const fn zero() -> Quaternion {
        Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn identity() -> Quaternion {
        Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn inverse(self) -> Quaternion {
        let w = self.w;
        let x = self.x;
        let y = self.y;
        let z = self.z;

        let mut norm_sq = w * w + x * x + y * y + z * z;
        if norm_sq == 0.0 {
            return Quaternion::zero();
        }

        norm_sq = 1.0 / norm_sq;
        return Quaternion {
            x: -x * norm_sq,
            y: -y * norm_sq,
            z: -z * norm_sq,
            w: w * norm_sq,
        };
    }

    pub fn to_matrix4x4(self) -> Matrix4x4 {
        let w = self.w;
        let x = self.x;
        let y = self.y;
        let z = self.z;

        /*let wx = w * x;
        let wy = w * y;
        let wz = w * z;
        let xx = x * x;
        let xy = x * y;
        let xz = x * z;
        let yy = y * y;
        let yz = y * z;
        let zz = z * z;*/

        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        let mut mx = Matrix4x4::new();

        mx.raw[0] = 1.0 - (yy + zz);
        mx.raw[4] = xy - wz;
        mx.raw[8] = xz + wy;

        mx.raw[1] = xy + wz;
        mx.raw[5] = 1.0 - (xx + zz);
        mx.raw[9] = yz - wx;

        mx.raw[2] = xz - wy;
        mx.raw[6] = yz + wx;
        mx.raw[10] = 1.0 - (xx + yy);

        // last column
        mx.raw[3] = 0.0;
        mx.raw[7] = 0.0;
        mx.raw[11] = 0.0;

        // bottom row
        mx.raw[12] = 0.0;
        mx.raw[13] = 0.0;
        mx.raw[14] = 0.0;
        mx.raw[15] = 1.0;

        mx

        /*Matrix4x4 {
            raw: [
                1.0 - 2.0 * (yy + zz),
                2.0 * (xy - wz),
                2.0 * (xz + wy),
                0.0,
                2.0 * (xy + wz),
                1.0 - 2.0 * (xx + zz),
                2.0 * (yz - wx),
                0.0,
                2.0 * (xz - wy),
                2.0 * (yz + wx),
                1.0 - 2.0 * (xx + yy),
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        }*/
    }

    pub fn from_euler(v: Vector3) -> Quaternion {
        let _x = v.x * 0.5;
        let _y = v.y * 0.5;
        let _z = v.z * 0.5;

        let c_x = _x.cos();
        let c_y = _y.cos();
        let c_z = _z.cos();

        let s_x = _x.sin();
        let s_y = _y.sin();
        let s_z = _z.sin();

        Quaternion {
            w: c_x * c_y * c_z - s_x * s_y * s_z,
            x: c_y * c_z * s_x + c_x * s_y * s_z,
            y: c_x * c_z * s_y - c_y * s_x * s_z,
            z: c_x * c_y * s_z + c_z * s_x * s_y,
        }
    }

    pub fn to_euler(self) -> Vector3 {
        let t = 2.0 * (self.w * self.y - self.z * self.x);
        let mut v = Vector3::zero();

        // Set X
        let a = 2.0 * (self.w * self.x + self.y * self.z);
        v.x = a.atan2(1.0 - 2.0 * (self.x * self.x + self.y * self.y));

        // Set Y
        if t >= 1.0 {
            v.y = std::f32::consts::PI / 2.0;
        } else {
            if t <= -1.0 {
                v.y = -std::f32::consts::PI / 2.0;
            } else {
                v.y = t.asin();
            }
        }

        // Set Z
        let a = 2.0 * (self.w * self.z + self.x * self.y);
        v.z = a.atan2(1.0 - 2.0 * (self.y * self.y + self.z * self.z));

        return v;
    }

    pub fn from_bytes(b: &[u8]) -> Quaternion {
        Quaternion {
            x: f32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            y: f32::from_le_bytes([b[4], b[5], b[6], b[7]]),
            z: f32::from_le_bytes([b[8], b[9], b[10], b[11]]),
            w: f32::from_le_bytes([b[12], b[13], b[14], b[15]]),
        }
    }
}
