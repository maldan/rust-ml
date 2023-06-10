use crate::math::la::vector3::Vector3;

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct Point {
    pub position: Vector3,
    pub size: f32,
}

#[derive(Clone, Default)]
pub struct PointLayer {
    pub vertex_list: Vec<f32>,
    pub point_list: Vec<Point>,
}

impl PointLayer {
    pub const fn new() -> Self {
        PointLayer {
            vertex_list: vec![],
            point_list: vec![],
        }
    }
    pub fn add(&mut self, p: Point) {
        self.point_list.push(p)
    }

    pub fn draw(&self) {
        print!("{}", "sas")
    }
}
