use crate::math::la::vector2::Vector2;
use crate::math::la::vector3::Vector3;
use crate::render::mesh::mesh::MeshData;

pub fn new(size: Vector3, side: u8) -> MeshData {
    let mut m = MeshData::new();

    // Front
    if side & 0b1000_0000 == 0b1000_0000 {
        m.vertex.push(Vector3::new(-1.0, -1.0, 1.0));
        m.vertex.push(Vector3::new(1.0, -1.0, 1.0));
        m.vertex.push(Vector3::new(1.0, 1.0, 1.0));
        m.vertex.push(Vector3::new(-1.0, 1.0, 1.0));

        for _ in 0..4 {
            m.normal.push(Vector3::new(0.0, 0.0, 1.0))
        }
    }

    // Back
    if side & 0b0100_0000 == 0b0100_0000 {
        m.vertex.push(Vector3::new(-1.0, -1.0, -1.0));
        m.vertex.push(Vector3::new(-1.0, 1.0, -1.0));
        m.vertex.push(Vector3::new(1.0, 1.0, -1.0));
        m.vertex.push(Vector3::new(1.0, -1.0, -1.0));

        for _ in 0..4 {
            m.normal.push(Vector3::new(0.0, 0.0, -1.0))
        }
    }

    // Top
    if side & 0b0010_0000 == 0b0010_0000 {
        m.vertex.push(Vector3::new(-1.0, 1.0, -1.0));
        m.vertex.push(Vector3::new(-1.0, 1.0, 1.0));
        m.vertex.push(Vector3::new(1.0, 1.0, 1.0));
        m.vertex.push(Vector3::new(1.0, 1.0, -1.0));

        for _ in 0..4 {
            m.normal.push(Vector3::new(0.0, 1.0, 0.0))
        }
    }

    // Bottom
    if side & 0b0001_0000 == 0b0001_0000 {
        m.vertex.push(Vector3::new(-1.0, -1.0, -1.0));
        m.vertex.push(Vector3::new(1.0, -1.0, -1.0));
        m.vertex.push(Vector3::new(1.0, -1.0, 1.0));
        m.vertex.push(Vector3::new(-1.0, -1.0, 1.0));

        for _ in 0..4 {
            m.normal.push(Vector3::new(0.0, -1.0, 0.0))
        }
    }

    // Left
    if side & 0b0000_1000 == 0b0000_1000 {
        m.vertex.push(Vector3::new(-1.0, -1.0, -1.0));
        m.vertex.push(Vector3::new(-1.0, -1.0, 1.0));
        m.vertex.push(Vector3::new(-1.0, 1.0, 1.0));
        m.vertex.push(Vector3::new(-1.0, 1.0, -1.0));

        for _ in 0..4 {
            m.normal.push(Vector3::new(-1.0, 0.0, 0.0))
        }
    }

    // Right
    if side & 0b0000_0100 == 0b0000_0100 {
        m.vertex.push(Vector3::new(1.0, -1.0, -1.0));
        m.vertex.push(Vector3::new(1.0, 1.0, -1.0));
        m.vertex.push(Vector3::new(1.0, 1.0, 1.0));
        m.vertex.push(Vector3::new(1.0, -1.0, 1.0));

        for _ in 0..4 {
            m.normal.push(Vector3::new(1.0, 0.0, 0.0))
        }
    }

    // Scale
    for i in 0..m.vertex.len() {
        m.vertex[i] = m.vertex[i].mul(size)
    }

    // UV
    for _ in 0..side.count_ones() {
        m.uv0.push(Vector2::new(0.0, 0.0));
        m.uv0.push(Vector2::new(1.0, 0.0));
        m.uv0.push(Vector2::new(1.0, 1.0));
        m.uv0.push(Vector2::new(0.0, 1.0));
    }

    // Index
    for i in 0..side.count_ones() {
        let next = i * 4;
        m.index.push(next);
        m.index.push(1 + next);
        m.index.push(2 + next);
        m.index.push(next);
        m.index.push(2 + next);
        m.index.push(3 + next);
    }

    return m;
}
