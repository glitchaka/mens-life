//! Compact meshes derived directly from the user-provided `House_scene_01.fbx`.
//! The property mesh keeps the house, veranda, exterior ground/patio and vegetation.
use crate::geometry::Geometry;

const POSITION_SCALE: f32 = 1.0 / 2048.0;
const PROPERTY: &[&str] = &[
    include_str!("model_data/house_00.txt"),
    include_str!("model_data/house_01.txt"),
    include_str!("model_data/house_02.txt"),
    include_str!("model_data/house_03.txt"),
];
const ROOF: &[&str] = &[include_str!("model_data/house_roof.txt")];

fn sextet(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn decode64(chunks: &[&str]) -> Vec<u8> {
    let mut out = Vec::with_capacity(chunks.iter().map(|chunk| chunk.len()).sum::<usize>() * 3 / 4);
    let mut acc = 0u32;
    let mut bits = 0u8;
    for chunk in chunks {
        for byte in chunk.bytes() {
            if byte == b'=' { continue; }
            let Some(value) = sextet(byte) else { continue; };
            acc = (acc << 6) | value as u32;
            bits += 6;
            while bits >= 8 {
                bits -= 8;
                out.push((acc >> bits) as u8);
                acc &= if bits == 0 { 0 } else { (1u32 << bits) - 1 };
            }
        }
    }
    out
}

fn u32_at(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(bytes[at..at + 4].try_into().expect("house mesh u32"))
}
fn i16_at(bytes: &[u8], at: usize) -> i16 {
    i16::from_le_bytes(bytes[at..at + 2].try_into().expect("house mesh i16"))
}
fn u16_at(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes(bytes[at..at + 2].try_into().expect("house mesh u16"))
}

fn unpack(chunks: &[&str]) -> Geometry {
    let bytes = decode64(chunks);
    assert!(bytes.len() >= 8, "supplied house mesh is incomplete");
    let vertex_count = u32_at(&bytes, 0) as usize;
    let index_count = u32_at(&bytes, 4) as usize;
    let expected = 8 + vertex_count * 10 + index_count * 2;
    assert!(bytes.len() >= expected, "supplied house mesh data was truncated");

    let mut positions = Vec::with_capacity(vertex_count);
    let mut colors = Vec::with_capacity(vertex_count);
    let mut at = 8usize;
    for _ in 0..vertex_count {
        positions.push([
            i16_at(&bytes, at) as f32 * POSITION_SCALE,
            i16_at(&bytes, at + 2) as f32 * POSITION_SCALE,
            i16_at(&bytes, at + 4) as f32 * POSITION_SCALE,
        ]);
        colors.push([
            bytes[at + 6] as f32 / 255.0,
            bytes[at + 7] as f32 / 255.0,
            bytes[at + 8] as f32 / 255.0,
            bytes[at + 9] as f32 / 255.0,
        ]);
        at += 10;
    }

    let mut indices = Vec::with_capacity(index_count);
    for _ in 0..index_count {
        indices.push(u16_at(&bytes, at) as u32);
        at += 2;
    }

    let mut normals = vec![[0.0f32; 3]; vertex_count];
    for triangle in indices.chunks_exact(3) {
        let a = positions[triangle[0] as usize];
        let b = positions[triangle[1] as usize];
        let c = positions[triangle[2] as usize];
        let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
        let n = [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ];
        for &index in triangle {
            let normal = &mut normals[index as usize];
            normal[0] += n[0];
            normal[1] += n[1];
            normal[2] += n[2];
        }
    }
    for normal in &mut normals {
        let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        if length > 1e-8 {
            normal[0] /= length;
            normal[1] /= length;
            normal[2] /= length;
        } else {
            *normal = [0.0, 1.0, 0.0];
        }
    }

    Geometry::from_raw(positions, normals, Vec::new(), colors, indices)
}

/// House, veranda, exterior ground/patio and vegetation from the supplied FBX.
pub fn lower() -> Geometry { unpack(PROPERTY) }
/// Original upper/roof geometry, separated only so gameplay can reveal the interior.
pub fn upper() -> Geometry { unpack(ROOF) }
