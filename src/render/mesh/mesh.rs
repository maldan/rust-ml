use crate::image::color::ColorRGBA;
use crate::math::la::quaternion::Quaternion;
use crate::math::la::vector2::Vector2;
use crate::math::la::vector3::Vector3;
use crate::math::la::vector4::Vector4;
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

#[derive(Clone, Debug)]
#[repr(C)]
pub struct SkinnedMeshData {
    pub id: u32,
    pub vertex: Vec<Vector3>,
    pub index: Vec<u32>,
    pub normal: Vec<Vector3>,
    pub uv0: Vec<Vector2>,
    pub weight: Vec<Vector4>,
    pub bone_index: Vec<ColorRGBA>,
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

    pub fn from_mm_bytes(bytes: &[u8]) -> MeshData {
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

    pub fn from_mm2_bytes(bytes: &[u8]) -> MeshData {
        let mut mesh = MeshData::new();
        let mut offset = 0;

        fn sex(b: &[u8], mut offset: usize, ident: usize) -> usize {
            // Read bone name
            let l = b[offset];
            offset += 1;
            let name = std::str::from_utf8(&b[offset..offset + l as usize]).unwrap();
            offset += l as usize;
            log::info!("H Bone {} {}", "-".repeat(ident), name);
            // Childs
            let amount = b[offset];
            offset += 1;
            for i in 0..amount {
                offset = sex(&b, offset, ident + 2);
            }
            return offset;
        }

        loop {
            // Read section name
            let l = bytes[offset];
            offset += 1;
            let name = std::str::from_utf8(&bytes[offset..offset + l as usize]).unwrap();
            offset += l as usize;

            // Read size
            let size = number::le_slice_to_u32(&bytes[offset..offset + 4]);
            offset += 4;

            // println!("{}", name);

            match name {
                "VERTEX" => {
                    offset += size as usize;
                }
                "BONE" => {
                    let amount = bytes[offset];
                    offset += 1;

                    // Read bone info
                    for i in 0..amount {
                        // Read bone name
                        let l = bytes[offset];
                        offset += 1;
                        let name =
                            std::str::from_utf8(&bytes[offset..offset + l as usize]).unwrap();
                        offset += l as usize;

                        log::info!("Bone {}", name);

                        // Read position
                        let position = Vector3::from_bytes(&bytes[offset..offset + 4 * 3]);
                        offset += 4 * 3;
                        log::info!("Position {}", position);

                        // Read rotation
                        let rotation = Quaternion::from_bytes(&bytes[offset..offset + 4 * 4]);
                        offset += 4 * 4;
                        log::info!("Rotation {}", rotation);
                    }

                    // Read hierarchy
                    offset = sex(bytes, offset, 0);
                }
                _ => {
                    offset += size as usize;
                }
            }

            if name == "END" {
                break;
            }
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
