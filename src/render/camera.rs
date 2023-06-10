use crate::math::la::matrix4::Matrix4x4;
use crate::math::la::vector3::Vector3;

pub struct PerspectiveCamera {
    pub position: Vector3,
    pub rotation: Vector3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
    pub matrix: Matrix4x4,
}

impl PerspectiveCamera {
    pub const fn new() -> PerspectiveCamera {
        PerspectiveCamera {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            fov: 45.0,
            aspect: 1.0,
            near: 0.01,
            far: 40.0,
            matrix: Matrix4x4::new(),
        }
    }

    pub fn calculate(&mut self) {
        // Projection
        let mut proj = Matrix4x4::new();
        proj.perspective(self.fov, self.aspect, self.near, self.far);

        // Calculate view
        self.matrix.identity();
        let pos = self.position.scale(-1.0);

        self.matrix.rotate_x(self.rotation.x);
        self.matrix.rotate_y(self.rotation.y);
        self.matrix.rotate_z(self.rotation.z);
        self.matrix.translate(pos.x, pos.y, pos.z);

        // Set final matrrx
        self.matrix = proj * self.matrix;
    }
}
