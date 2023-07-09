use crate::math::la::matrix4::Matrix4x4;
use crate::math::la::quaternion::Quaternion;
use crate::math::la::vector3::Vector3;

pub struct PerspectiveCamera {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
    pub projection_matrix: Matrix4x4,
    pub view_matrix: Matrix4x4,
}

impl PerspectiveCamera {
    pub const fn new() -> PerspectiveCamera {
        PerspectiveCamera {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::identity(),
            fov: 45.0,
            aspect: 1.0,
            near: 0.001,
            far: 120.0,
            projection_matrix: Matrix4x4::new(),
            view_matrix: Matrix4x4::new(),
        }
    }

    pub fn calculate(&mut self) {
        // Projection
        self.projection_matrix.identity();
        self.projection_matrix
            .perspective(self.fov, self.aspect, self.near, self.far);

        // View matrix
        self.view_matrix.identity();
        self.view_matrix.rotate_quaternion(self.rotation);
        self.view_matrix.translate_vec3(self.position.scale(-1.0));

        // Projection
        /*let mut proj = Matrix4x4::new();
        proj.perspective(self.fov, self.aspect, self.near, self.far);

        // Calculate view
        self.matrix.identity();
        let pos = self.position.scale(-1.0);

        self.matrix.rotate_x(self.rotation.x);
        self.matrix.rotate_y(self.rotation.y);
        self.matrix.rotate_z(self.rotation.z);
        self.matrix.translate(pos.x, pos.y, pos.z);

        // Set final matrix
        self.matrix = proj * self.matrix;*/
    }
}
