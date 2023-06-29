use crate::image::color::ColorRGBA;
use crate::math::la::matrix4::Matrix4x4;
use crate::math::la::quaternion::Quaternion;
use crate::math::la::vector2::Vector2;
use crate::math::la::vector3::Vector3;
use crate::math::la::vector4::Vector4;
use crate::math::number;
use crate::render::mesh::bone::Bone;
use log::log;
use std::cell::RefMut;
use std::collections::HashMap;
use std::ffi::c_char;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct MeshData {
    pub id: u32,
    pub vertex: Vec<Vector3>,
    pub index: Vec<u32>,
    pub normal: Vec<Vector3>,
    pub uv0: Vec<Vector2>,
    pub color0: Vec<Vector3>,
    pub bone_weight: Vec<Vector4>,
    pub bone_index: Vec<Vector4>,
    pub bone_list: Vec<Bone>,
    pub bone_root_id: usize,
    // pub bone_list2: Vec<Rc<Bone>>,
    // bone_parent_map: Vec<Vec<u32>>,
    // flat_bone_list: Vec<&'a Bone>,
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
            bone_weight: vec![],
            bone_index: vec![],
            bone_list: vec![],
            bone_root_id: 0,
            // bone_list2: vec![],
            //flat_bone_list: vec![],
            //bone_parent_map: vec![],
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

        // Pre init bone parent map
        let mut bone_parent_map: Vec<Vec<u32>> = vec![];
        for _ in 0..255 {
            bone_parent_map.push(vec![]);
            mesh.bone_list.push(Bone::new());
        }

        fn parse_hierarchy(
            b: &[u8],
            mut offset: usize,
            v: &mut Vec<Vec<u32>>,
            ident: i32,
        ) -> (usize, u32) {
            // Read bone name
            let bone_index = b[offset] as u32;
            offset += 1;

            // log::info!("Bone {} {}", "-".repeat(ident as usize), bone_index);

            // Childs
            let amount = b[offset];
            offset += 1;
            for _ in 0..amount {
                let r = parse_hierarchy(b, offset, v, ident + 1);
                offset = r.0;
                v[bone_index as usize].push(r.1);
            }
            (offset, bone_index)
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
                "BONE_WEIGHT" => {
                    let amount = number::le_slice_to_u32(&bytes[offset..offset + 4]);
                    offset += 4;

                    for _ in 0..amount {
                        let position = Vector4::from_bytes(&bytes[offset..offset + 4 * 4]);
                        offset += 4 * 4;
                        mesh.bone_weight.push(position);
                    }
                }
                "BONE_INDEX" => {
                    let amount = number::le_slice_to_u32(&bytes[offset..offset + 4]);
                    offset += 4;

                    for _ in 0..amount {
                        let position = Vector4::from_bytes(&bytes[offset..offset + 4 * 4]);
                        offset += 4 * 4;
                        mesh.bone_index.push(position);
                    }
                }
                "VERTEX" => {
                    let amount = number::le_slice_to_u32(&bytes[offset..offset + 4]);
                    offset += 4;

                    for _ in 0..amount {
                        let position = Vector3::from_bytes(&bytes[offset..offset + 4 * 3]);
                        offset += 4 * 3;
                        mesh.vertex.push(position);
                    }

                    log::info!("VERTEX DONE")
                }
                "UV" => {
                    let amount = number::le_slice_to_u32(&bytes[offset..offset + 4]);
                    offset += 4;

                    for _ in 0..amount {
                        let position = Vector2::from_bytes(&bytes[offset..offset + 4 * 2]);
                        offset += 4 * 2;
                        mesh.uv0.push(position);
                    }

                    log::info!("UV DONE")
                }
                "NORMAL" => {
                    let amount = number::le_slice_to_u32(&bytes[offset..offset + 4]);
                    offset += 4;

                    for _ in 0..amount {
                        let position = Vector3::from_bytes(&bytes[offset..offset + 4 * 3]);
                        offset += 4 * 3;
                        mesh.normal.push(position);
                    }

                    log::info!("NORMAL DONE")
                }
                "INDEX" => {
                    let amount = number::le_slice_to_u32(&bytes[offset..offset + 4]);
                    offset += 4;

                    for _ in 0..amount {
                        let index = number::le_slice_to_u32(&bytes[offset..offset + 4]);
                        offset += 4;
                        mesh.index.push(index);
                    }

                    log::info!("INDEX DONE")
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

                        // Bone index
                        let bone_index = bytes[offset];
                        offset += 1;

                        log::info!("Bone {} - {} - {}", name, bone_index, i);

                        // Read position
                        let position = Vector3::from_bytes(&bytes[offset..offset + 4 * 3]);
                        offset += 4 * 3;
                        //log::info!("Position {}", position);

                        // Read rotation
                        let rotation = Quaternion::from_bytes(&bytes[offset..offset + 4 * 4]);
                        offset += 4 * 4;
                        //log::info!("Rotation {}", rotation);

                        let inverse_bind_matrix =
                            Matrix4x4::from_bytes(&bytes[offset..offset + 4 * 4 * 4]);
                        offset += 4 * 4 * 4;

                        // Add bone
                        let mut bone = Bone::new();
                        bone.id = bone_index as u32;
                        bone.name = String::from(name);
                        bone.position = position;
                        //bone.position = Vector3::zero() * inverse_bind_matrix.invert();
                        bone.rotation = rotation;
                        bone.inverse_bind_matrix = inverse_bind_matrix;
                        log::info!(
                            "Position {} | Rotation {}",
                            bone.position,
                            bone.rotation.to_euler().to_degrees()
                        );
                        log::info!(
                            "Inversed Position {} | Inversed Rotation {}",
                            bone.inverse_bind_matrix.get_position(),
                            bone.inverse_bind_matrix
                                .get_rotation()
                                .to_euler()
                                .to_degrees(),
                        );
                        mesh.bone_list[bone_index as usize] = bone;

                        // bone_map.insert(String::from(name), Box::new(bone));
                    }

                    // Set root id
                    mesh.bone_root_id = bytes[offset] as usize; // first id

                    // Read hierarchy
                    let r = parse_hierarchy(bytes, offset, &mut bone_parent_map, 0);
                    offset = r.0;

                    for i in 0..mesh.bone_list.len() {
                        let id = mesh.bone_list[i].id;
                        mesh.bone_list[i].children = bone_parent_map[id as usize].clone();
                    }

                    // mesh.flat_bone_list.push(&bone_list[0]);

                    // Build bone map
                    // mesh.bone_map.as_mut().unwrap().insert(String::from("Sex"), &mesh.bone_list[0]);

                    /*fn build_bone_map<'a>(m: &'a mut HashMap<String, &'a Bone>, bl: &'a Vec<Bone>) {
                        for i in 0..bl.len() {
                            m.insert(bl[i].name.clone(), &bl[i]);
                            build_bone_map(m, &bl[i].children);
                        }
                    }
                    build_bone_map(&mut mesh.bone_map.unwrap(), &mesh.bone_list);*/
                }
                _ => {
                    offset += size as usize;
                }
            }

            if name == "END" {
                break;
            }
        }

        mesh
    }

    pub fn calculate_bones(&mut self, start_index: usize) {
        fn do_x(start_index: usize, bone_list: &mut Vec<Bone>, parent: Matrix4x4) {
            let bone = &mut bone_list[start_index];

            let mut mx = Matrix4x4::new();
            mx.translate_vec3(bone.position);
            mx *= (bone.rotation * bone.local_rotation).to_matrix4x4();
            //mx.rotate_quaternion(bone.rotation);

            bone.matrix = parent * mx;

            let children = bone_list[start_index].children.clone();
            let matrix = bone_list[start_index].matrix;

            // Calculate childrens
            for id in children.iter() {
                do_x(*id as usize, bone_list, matrix);
            }
        }

        do_x(start_index, &mut self.bone_list, Matrix4x4::new());
    }

    /*pub fn set_bone_rotation(&mut self, name: &str, q: Quaternion) {
        fn sex(name: &str, list: &mut Vec<Bone>, q: Quaternion) {
            for i in 0..list.len() {
                if list[i].name == name {
                    list[i].local_rotation = q;
                } else {
                    sex(name, &mut list[i].children, q);
                }
            }
        }
        sex(name, &mut self.bone_list, q)
    }

    pub fn get_bone_by_id(&self, id: u32) -> Option<Bone> {
        fn sex(id: u32, list: &Vec<Bone>) -> Option<Bone> {
            for i in 0..list.len() {
                if list[i].id == id {
                    return Some(list[i].clone());
                } else {
                    let r = sex(id, &list[i].children);
                    if r.is_some() {
                        return r;
                    }
                }
            }
            None
        }
        sex(id, &self.bone_list)
    }*/

    /*pub fn get_bone_by_name(&mut self, name: &str) -> Option<&mut Bone> {
        fn sex<'a>(name: &str, list: &'a mut Vec<Bone>) -> Option<&'a mut Bone> {
            for i in 0..list.len() {
                if list[i].name == name {
                    return Some(&mut list[i]);
                } else {
                    let r = sex(name, &mut list[i].children);
                    if r.is_some() {
                        return r;
                    }
                }
                // return sex(name, &list[i].children);
            }
            None
        }
        sex(name, &mut self.bone_list)
    }*/
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
