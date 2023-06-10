use crate::math::la::vector4::Vector4;

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct ColorRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRGBA {
    pub const fn new() -> Self {
        ColorRGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
    pub const fn white() -> Self {
        ColorRGBA {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
    pub fn from_float(r: f32, g: f32, b: f32, a: f32) -> ColorRGBA {
        ColorRGBA {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
            a: (a * 255.0) as u8,
        }
    }
    pub fn to_u32(self) -> u32 {
        return u32::from_le_bytes([self.r, self.g, self.b, self.a]);
    }
    pub fn from_vector4(self, v: Vector4) -> ColorRGBA {
        ColorRGBA {
            r: (v.x * 255.0) as u8,
            g: (v.y * 255.0) as u8,
            b: (v.z * 255.0) as u8,
            a: (v.w * 255.0) as u8,
        }
    }
    pub fn to_vector4(self) -> Vector4 {
        Vector4 {
            x: self.r as f32 / 255.0,
            y: self.g as f32 / 255.0,
            z: self.b as f32 / 255.0,
            w: self.a as f32 / 255.0,
        }
    }
}
