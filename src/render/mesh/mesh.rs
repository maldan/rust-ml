use crate::math::la::vector2::Vector2;
use crate::math::la::vector3::Vector3;
use crate::math::number;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct MeshData {
    pub id: u32,
    pub vertex: Vec<Vector3>,
    pub index: Vec<u32>,
    pub normal: Vec<Vector3>,
    pub uv0: Vec<Vector2>,
    pub color0: Vec<Vector3>,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MeshInstance {
    pub mesh_id: u32,
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,
}

impl MeshData {
    pub const fn new() -> Self {
        MeshData {
            id: 0,
            vertex: vec![],
            index: vec![],
            normal: vec![],
            uv0: vec![],
            color0: vec![],
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> MeshData {
        let mut mesh = MeshData::new();
        let mut offset = 0;

        // Read vertex
        let amount = number::le_slice_to_u16(&bytes[offset..offset + 2]);
        offset += 2;
        for _ in 0..amount {
            let x = number::le_slice_to_f32(&bytes[offset..offset + 4]);
            offset += 4;
            let y = number::le_slice_to_f32(&bytes[offset..offset + 4]);
            offset += 4;
            let z = number::le_slice_to_f32(&bytes[offset..offset + 4]);
            offset += 4;

            mesh.vertex.push(Vector3::new(x, y, z));
        }

        // Read normal
        let amount = number::le_slice_to_u16(&bytes[offset..offset + 2]);
        offset += 2;
        for _ in 0..amount {
            let x = number::le_slice_to_f32(&bytes[offset..offset + 4]);
            offset += 4;
            let y = number::le_slice_to_f32(&bytes[offset..offset + 4]);
            offset += 4;
            let z = number::le_slice_to_f32(&bytes[offset..offset + 4]);
            offset += 4;

            mesh.normal.push(Vector3::new(x, y, z));
        }

        // Read index
        let amount = number::le_slice_to_u16(&bytes[offset..offset + 2]);
        offset += 2;
        for _ in 0..amount {
            let index = number::le_slice_to_u16(&bytes[offset..offset + 2]);
            offset += 2;

            mesh.index.push(index as u32);
        }

        // Read uv
        let amount = number::le_slice_to_u16(&bytes[offset..offset + 2]);
        offset += 2;
        for _ in 0..amount {
            let x = number::le_slice_to_f32(&bytes[offset..offset + 4]);
            offset += 4;
            let y = number::le_slice_to_f32(&bytes[offset..offset + 4]);
            offset += 4;

            mesh.uv0.push(Vector2::new(x, y));
        }

        return mesh;
    }
}

impl MeshInstance {
    pub fn new() -> Self {
        MeshInstance {
            mesh_id: 0,
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

/*impl PointLayer {
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
*/
