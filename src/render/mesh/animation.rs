use crate::math::la::quaternion::Quaternion;
use crate::math::la::vector3::Vector3;
use crate::math::la::vector4::Vector4;
use crate::math::number;
use std::collections::HashMap;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct MeshAnimation {
    pub name: String,
    pub current_time: f32,
    pub duration: f32,
    pub sequence_list: Vec<MeshAnimationSequence>,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct MeshAnimationSequence {
    pub key: String,
    pub kind: u8, // 1 - translation, 2 - rotation, 3 - scale
    pub prev_frame: usize,
    pub next_frame: usize,
    pub time: Vec<f32>,
    pub value: Vec<Vector4>,
}

impl MeshAnimationSequence {
    pub const fn new() -> Self {
        MeshAnimationSequence {
            key: String::new(),
            kind: 0,
            prev_frame: 0,
            next_frame: 0,
            time: vec![],
            value: vec![],
        }
    }

    pub fn calculate_frames(&mut self, time: f32) {
        self.prev_frame = 0;
        self.next_frame = 0;

        for i in 0..self.time.len() - 1 {
            if time >= self.time[i] && time < self.time[i + 1] {
                self.prev_frame = i;
                self.next_frame = i + 1;
                break;
            }
        }
    }

    pub fn calculate_frame_value(&mut self, time: f32) -> Vector4 {
        let t = ((time - self.time[self.prev_frame])
            / (self.time[self.next_frame] - self.time[self.prev_frame]))
            .clamp(0.0, 1.0);

        // Translate or scale
        if self.kind == 1 || self.kind == 3 {
            let v = Vector4::lerp(self.value[self.prev_frame], self.value[self.next_frame], t);
            return v;
        }
        if self.kind == 2 {
            let q1 = Quaternion::from_vector4(self.value[self.prev_frame]);
            let q2 = Quaternion::from_vector4(self.value[self.next_frame]);
            let v = Quaternion::lerp(q1, q2, t);
            return v.to_vector4();
        }
        Vector4::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl MeshAnimation {
    pub const fn new() -> Self {
        MeshAnimation {
            name: String::new(),
            current_time: 0.0,
            duration: 0.0,
            sequence_list: vec![],
        }
    }

    pub fn tick(&mut self, delta: f32) {
        self.current_time += delta;
        if self.current_time > self.duration {
            self.current_time -= self.duration;
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> MeshAnimation {
        let mut animation = MeshAnimation::new();
        let mut offset = 0_usize;

        // Version
        let _version = bytes[offset];
        offset += 1;

        // Index map
        let index_map_length = bytes[offset];
        offset += 1;
        let mut index_map: HashMap<u8, String> = HashMap::new();
        for _ in 0..index_map_length {
            let key = bytes[offset];
            offset += 1;
            let len = bytes[offset] as usize;
            offset += 1;
            let name = std::str::from_utf8(&bytes[offset..offset + len]).unwrap();
            offset += len;

            index_map.insert(key, String::from(name));
        }

        // Name
        let len = bytes[offset] as usize;
        offset += 1;
        let name = std::str::from_utf8(&bytes[offset..offset + len]).unwrap();
        offset += len;
        animation.name = String::from(name);

        // Channels
        let size = number::le_slice_to_u32(&bytes[offset..offset + 4]);
        offset += 4;
        log::info!("Parse animation: '{}' frames - {}", name, size);

        // Data
        for _ in 0..size {
            let node_id = bytes[offset];
            offset += 1;

            let operation = bytes[offset];
            offset += 1;

            // New sequence
            let mut sequence = MeshAnimationSequence::new();
            sequence.key = String::from(index_map.get(&node_id).unwrap());
            sequence.kind = operation;

            // Parse frames
            let amount_keyframes = number::le_slice_to_u32(&bytes[offset..offset + 4]);
            offset += 4;
            for _ in 0..amount_keyframes {
                let time = number::le_slice_to_f32(&bytes[offset..offset + 4]);
                offset += 4;
                sequence.time.push(time);
                animation.duration = animation.duration.max(time);
            }

            // Parse values
            let amount_values = number::le_slice_to_u32(&bytes[offset..offset + 4]);
            offset += 4;
            if operation == 0 {
                for _ in 0..amount_values {
                    offset += 4;
                    sequence.value.push(Vector4::new(0.0, 0.0, 0.0, 0.0));
                }
            }

            // Translation or scale
            if operation == 1 || operation == 3 {
                for _ in 0..amount_values {
                    let value = Vector3::from_bytes(&bytes[offset..offset + 4 * 3]);
                    offset += 4 * 3;
                    sequence.value.push(Vector4::from_vector3(value));
                }
            }

            // Rotation
            if operation == 2 {
                for _ in 0..amount_values {
                    let value = Vector4::from_bytes(&bytes[offset..offset + 4 * 4]);
                    offset += 4 * 4;
                    sequence.value.push(value);
                }
            }

            animation.sequence_list.push(sequence);
        }

        animation
    }
}
