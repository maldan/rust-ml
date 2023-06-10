pub mod geom;
pub mod helper;
pub mod la;
pub mod number;
pub mod random;

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    return (1.0 - t) * start + t * end;
}

pub fn remap(value: f32, low1: f32, high1: f32, low2: f32, high2: f32) -> f32 {
    return low2 + (high2 - low2) * (value - low1) / (high1 - low1);
}
