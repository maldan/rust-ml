pub fn le_slice_to_u16(b: &[u8]) -> u16 {
    (b[0] as u16) | (b[1] as u16) << 8
}

pub fn le_slice_to_i16(b: &[u8]) -> i16 {
    (b[0] as i16) | (b[1] as i16) << 8
}

pub fn le_slice_to_u32(b: &[u8]) -> u32 {
    (b[0] as u32) | (b[1] as u32) << 8 | (b[2] as u32) << 16 | (b[3] as u32) << 24
}

pub fn le_slice_to_f32(b: &[u8]) -> f32 {
    f32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

pub trait F32Ext {
    fn denormalize(&self, min: f32, max: f32) -> Self;
}

impl F32Ext for f32 {
    fn denormalize(&self, min: f32, max: f32) -> Self {
        let range_val = max - min;
        (*self * range_val) + min
    }
}
