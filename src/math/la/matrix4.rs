use crate::math::la::vector3::Vector3;
use std::fmt;
use std::ops;

#[derive(Copy, Clone, Default)]
pub struct Matrix4x4 {
    pub raw: [f32; 16],
}

impl fmt::Display for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Matrix4x4(\n\t{}, {}, {}, {}\n\t{}, {}, {}, {}\n\t{}, {}, {}, {}\n\t{}, {}, {}, {}\n)",
            self.raw[0],
            self.raw[1],
            self.raw[2],
            self.raw[3],
            self.raw[4],
            self.raw[5],
            self.raw[6],
            self.raw[7],
            self.raw[8],
            self.raw[9],
            self.raw[10],
            self.raw[11],
            self.raw[12],
            self.raw[13],
            self.raw[14],
            self.raw[15],
        )
    }
}

impl ops::Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, b: Matrix4x4) -> Matrix4x4 {
        let mut mx = Matrix4x4::default();

        let a00 = self.raw[0];
        let a01 = self.raw[1];
        let a02 = self.raw[2];
        let a03 = self.raw[3];
        let a10 = self.raw[4];
        let a11 = self.raw[5];
        let a12 = self.raw[6];
        let a13 = self.raw[7];
        let a20 = self.raw[8];
        let a21 = self.raw[9];
        let a22 = self.raw[10];
        let a23 = self.raw[11];
        let a30 = self.raw[12];
        let a31 = self.raw[13];
        let a32 = self.raw[14];
        let a33 = self.raw[15];

        // Cache only the current line of the second matrix
        let mut b0 = b.raw[0];
        let mut b1 = b.raw[1];
        let mut b2 = b.raw[2];
        let mut b3 = b.raw[3];
        mx.raw[0] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
        mx.raw[1] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
        mx.raw[2] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
        mx.raw[3] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

        b0 = b.raw[4];
        b1 = b.raw[5];
        b2 = b.raw[6];
        b3 = b.raw[7];
        mx.raw[4] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
        mx.raw[5] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
        mx.raw[6] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
        mx.raw[7] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

        b0 = b.raw[8];
        b1 = b.raw[9];
        b2 = b.raw[10];
        b3 = b.raw[11];
        mx.raw[8] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
        mx.raw[9] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
        mx.raw[10] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
        mx.raw[11] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

        b0 = b.raw[12];
        b1 = b.raw[13];
        b2 = b.raw[14];
        b3 = b.raw[15];
        mx.raw[12] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
        mx.raw[13] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
        mx.raw[14] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
        mx.raw[15] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

        mx
    }
}

impl Matrix4x4 {
    pub const fn new() -> Self {
        Matrix4x4 {
            raw: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn identity(&mut self) {
        self.raw[0] = 1.0;
        self.raw[1] = 0.0;
        self.raw[2] = 0.0;
        self.raw[3] = 0.0;

        self.raw[4] = 0.0;
        self.raw[5] = 1.0;
        self.raw[6] = 0.0;
        self.raw[7] = 0.0;

        self.raw[8] = 0.0;
        self.raw[9] = 0.0;
        self.raw[10] = 1.0;
        self.raw[11] = 0.0;

        self.raw[12] = 0.0;
        self.raw[13] = 0.0;
        self.raw[14] = 0.0;
        self.raw[15] = 1.0;
    }

    pub fn invert(self) -> Matrix4x4 {
        let mut mx = Matrix4x4::default();

        let a00 = self.raw[0];
        let a01 = self.raw[1];
        let a02 = self.raw[2];
        let a03 = self.raw[3];
        let a10 = self.raw[4];
        let a11 = self.raw[5];
        let a12 = self.raw[6];
        let a13 = self.raw[7];
        let a20 = self.raw[8];
        let a21 = self.raw[9];
        let a22 = self.raw[10];
        let a23 = self.raw[11];
        let a30 = self.raw[12];
        let a31 = self.raw[13];
        let a32 = self.raw[14];
        let a33 = self.raw[15];

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        // Calculate the determinant
        let mut det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

        if det == 0.0 {
            return mx;
        }
        det = 1.0 / det;

        mx.raw[0] = (a11 * b11 - a12 * b10 + a13 * b09) * det;
        mx.raw[1] = (a02 * b10 - a01 * b11 - a03 * b09) * det;
        mx.raw[2] = (a31 * b05 - a32 * b04 + a33 * b03) * det;
        mx.raw[3] = (a22 * b04 - a21 * b05 - a23 * b03) * det;
        mx.raw[4] = (a12 * b08 - a10 * b11 - a13 * b07) * det;
        mx.raw[5] = (a00 * b11 - a02 * b08 + a03 * b07) * det;
        mx.raw[6] = (a32 * b02 - a30 * b05 - a33 * b01) * det;
        mx.raw[7] = (a20 * b05 - a22 * b02 + a23 * b01) * det;
        mx.raw[8] = (a10 * b10 - a11 * b08 + a13 * b06) * det;
        mx.raw[9] = (a01 * b08 - a00 * b10 - a03 * b06) * det;
        mx.raw[10] = (a30 * b04 - a31 * b02 + a33 * b00) * det;
        mx.raw[11] = (a21 * b02 - a20 * b04 - a23 * b00) * det;
        mx.raw[12] = (a11 * b07 - a10 * b09 - a12 * b06) * det;
        mx.raw[13] = (a00 * b09 - a01 * b07 + a02 * b06) * det;
        mx.raw[14] = (a31 * b01 - a30 * b03 - a32 * b00) * det;
        mx.raw[15] = (a20 * b03 - a21 * b01 + a22 * b00) * det;

        return mx;
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        let a00 = self.raw[0];
        let a01 = self.raw[1];
        let a02 = self.raw[2];
        let a03 = self.raw[3];
        let a10 = self.raw[4];
        let a11 = self.raw[5];
        let a12 = self.raw[6];
        let a13 = self.raw[7];
        let a20 = self.raw[8];
        let a21 = self.raw[9];
        let a22 = self.raw[10];
        let a23 = self.raw[11];

        self.raw[0] = a00;
        self.raw[1] = a01;
        self.raw[2] = a02;
        self.raw[3] = a03;
        self.raw[4] = a10;
        self.raw[5] = a11;
        self.raw[6] = a12;
        self.raw[7] = a13;
        self.raw[8] = a20;
        self.raw[9] = a21;
        self.raw[10] = a22;
        self.raw[11] = a23;

        self.raw[12] = a00 * x + a10 * y + a20 * z + self.raw[12];
        self.raw[13] = a01 * x + a11 * y + a21 * z + self.raw[13];
        self.raw[14] = a02 * x + a12 * y + a22 * z + self.raw[14];
        self.raw[15] = a03 * x + a13 * y + a23 * z + self.raw[15];
    }

    pub fn rotate_x(&mut self, rad: f32) {
        let s = rad.sin();
        let c = rad.cos();

        let a10 = self.raw[4];
        let a11 = self.raw[5];
        let a12 = self.raw[6];
        let a13 = self.raw[7];
        let a20 = self.raw[8];
        let a21 = self.raw[9];
        let a22 = self.raw[10];
        let a23 = self.raw[11];

        // Perform axis-specific matrix multiplication
        self.raw[4] = a10 * c + a20 * s;
        self.raw[5] = a11 * c + a21 * s;
        self.raw[6] = a12 * c + a22 * s;
        self.raw[7] = a13 * c + a23 * s;
        self.raw[8] = a20 * c - a10 * s;
        self.raw[9] = a21 * c - a11 * s;
        self.raw[10] = a22 * c - a12 * s;
        self.raw[11] = a23 * c - a13 * s;
    }

    pub fn rotate_y(&mut self, rad: f32) {
        let s = rad.sin();
        let c = rad.cos();

        let a00 = self.raw[0];
        let a01 = self.raw[1];
        let a02 = self.raw[2];
        let a03 = self.raw[3];
        let a20 = self.raw[8];
        let a21 = self.raw[9];
        let a22 = self.raw[10];
        let a23 = self.raw[11];

        // Perform axis-specific matrix multiplication
        self.raw[0] = a00 * c - a20 * s;
        self.raw[1] = a01 * c - a21 * s;
        self.raw[2] = a02 * c - a22 * s;
        self.raw[3] = a03 * c - a23 * s;
        self.raw[8] = a00 * s + a20 * c;
        self.raw[9] = a01 * s + a21 * c;
        self.raw[10] = a02 * s + a22 * c;
        self.raw[11] = a03 * s + a23 * c;
    }

    pub fn rotate_z(&mut self, rad: f32) {
        let s = rad.sin();
        let c = rad.cos();

        let a00 = self.raw[0];
        let a01 = self.raw[1];
        let a02 = self.raw[2];
        let a03 = self.raw[3];
        let a10 = self.raw[4];
        let a11 = self.raw[5];
        let a12 = self.raw[6];
        let a13 = self.raw[7];

        // Perform axis-specific matrix multiplication
        self.raw[0] = a00 * c + a10 * s;
        self.raw[1] = a01 * c + a11 * s;
        self.raw[2] = a02 * c + a12 * s;
        self.raw[3] = a03 * c + a13 * s;
        self.raw[4] = a10 * c - a00 * s;
        self.raw[5] = a11 * c - a01 * s;
        self.raw[6] = a12 * c - a02 * s;
        self.raw[7] = a13 * c - a03 * s;
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.raw[0] *= x;
        self.raw[1] *= x;
        self.raw[2] *= x;
        self.raw[3] *= x;

        self.raw[4] *= y;
        self.raw[5] *= y;
        self.raw[6] *= y;
        self.raw[7] *= y;

        self.raw[8] *= z;
        self.raw[9] *= z;
        self.raw[10] *= z;
        self.raw[11] *= z;
    }

    pub fn perspective(&mut self, fov: f32, aspect: f32, near: f32, far: f32) {
        let f = 1.0 / (fov / 2.0).tan();

        self.raw[0] = f / aspect;
        self.raw[1] = 0.0;
        self.raw[2] = 0.0;
        self.raw[3] = 0.0;
        self.raw[4] = 0.0;
        self.raw[5] = f;
        self.raw[6] = 0.0;
        self.raw[7] = 0.0;
        self.raw[8] = 0.0;
        self.raw[9] = 0.0;
        self.raw[11] = -1.0;
        self.raw[12] = 0.0;
        self.raw[13] = 0.0;
        self.raw[15] = 0.0;

        let nf = 1.0 / (near - far);
        self.raw[10] = (far + near) * nf;
        self.raw[14] = 2.0 * far * near * nf;
    }

    pub fn get_position(&self) -> Vector3 {
        Vector3 {
            x: self.raw[12],
            y: self.raw[13],
            z: self.raw[14],
        }
    }
}
