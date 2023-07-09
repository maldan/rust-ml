use crate::math::la::matrix4::Matrix4x4;
use crate::math::la::quaternion::Quaternion;
use crate::math::la::vector3::Vector3;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Bone {
    pub name: String,
    pub id: u32,
    pub is_changed: bool,

    pub position: Vector3,
    pub rotation: Quaternion,
    pub local_rotation: Quaternion,
    pub local_position: Vector3,
    pub local_scale: Vector3,
    pub matrix: Matrix4x4,
    pub inverse_bind_matrix: Matrix4x4,

    pub out_position: Vector3,

    pub children: Vec<u32>,
}

impl Bone {
    pub const fn new() -> Bone {
        Bone {
            name: String::new(),
            id: 0,
            is_changed: false,

            position: Vector3::zero(),
            rotation: Quaternion::identity(),
            local_position: Vector3::zero(),
            local_rotation: Quaternion::identity(),
            local_scale: Vector3::one(),
            matrix: Matrix4x4::new(),
            inverse_bind_matrix: Matrix4x4::new(),

            out_position: Vector3::zero(),

            children: vec![],
        }
    }

    /*pub fn calculate(&mut self, parent: Matrix4x4) {

    }*/

    /*pub fn calculate(&mut self, parent: Matrix4x4) {
        self.matrix.identity();

        //self.matrix
        //    .translate(self.position.x, self.position.y, self.position.z);
        self.matrix.rotate_quaternion(self.local_rotation);
        self.matrix *= parent;

        //self.matrix
        //    .translate(self.position.x, self.position.y, self.position.z);
        //self.matrix
        //    .scale(self.local_scale.x, self.local_scale.y, self.local_scale.z);

        for i in 0..self.children.len() {
            self.children[i].calculate(self.matrix);
        }
    }

    pub fn for_each(&self, f: &mut impl FnMut(&Bone, &Bone)) {
        for i in 0..self.children.len() {
            (*f)(self, &self.children[i]);
            self.children[i].for_each(f);
        }
    }*/
}
