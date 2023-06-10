use crate::image::color::ColorRGBA;
use crate::math::la::vector3::Vector3;

pub struct Line3D {
    pub from: Vector3,
    pub to: Vector3,
    pub color: ColorRGBA,
}
