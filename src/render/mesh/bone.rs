use crate::math::la::matrix4::Matrix4x4;
use crate::math::la::quaternion::Quaternion;
use crate::math::la::vector3::Vector3;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Bone {
    pub name: String,
    pub index: u32,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
    pub matrix: Matrix4x4,
    pub children: Vec<Bone>,
}

impl Bone {
    pub const fn new() -> Bone {
        Bone {
            name: String::new(),
            index: 0,
            position: Vector3::zero(),
            rotation: Quaternion::new(),
            scale: Vector3::one(),
            matrix: Matrix4x4::new(),
            children: vec![],
        }
    }

    pub fn calculate(&mut self, parent: Matrix4x4) {
        self.matrix.identity();
        self.matrix *= parent;

        self.matrix
            .translate(self.position.x, self.position.y, self.position.z);
        self.matrix.rotate_quaternion(self.rotation);
        self.matrix.scale(self.scale.x, self.scale.y, self.scale.z);

        for i in 0..self.children.len() {
            self.children[i].calculate(self.matrix);
        }
    }

    pub fn for_each(&mut self, f: &mut impl FnMut(&Bone, &Bone)) {
        for i in 0..self.children.len() {
            (*f)(self, &self.children[i]);
            self.children[i].for_each(f);
        }
    }
}
