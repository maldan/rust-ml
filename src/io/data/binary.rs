pub fn utf8_from_slice(b: &[u8]) -> f32 {
    return f32::from_le_bytes([b[0], b[1], b[2], b[3]]);
}
