//! Compact runtime meshes baked directly from the user-supplied FBX/OBJ assets.
//! No procedural replacement geometry is generated here.
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

const HOUSE: &[&str] = &[
    include_str!("model_data/house_00.txt"), include_str!("model_data/house_01.txt"),
    include_str!("model_data/house_02.txt"), include_str!("model_data/house_03.txt"),
    include_str!("model_data/house_04.txt"),
];
const GIRL: &[&str] = &[
    include_str!("model_data/girl_00.txt"), include_str!("model_data/girl_01.txt"),
    include_str!("model_data/girl_02.txt"),
];

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

fn decode(chunks: &[&str]) -> Vec<u8> {
    let mut out = Vec::with_capacity(chunks.iter().map(|s| s.len()).sum::<usize>() * 3 / 4);
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
    u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap())
}
fn i16_at(bytes: &[u8], at: usize) -> i16 {
    i16::from_le_bytes(bytes[at..at + 2].try_into().unwrap())
}
fn u16_at(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes(bytes[at..at + 2].try_into().unwrap())
}

fn unpack(chunks: &[&str]) -> Mesh {
    let bytes = decode(chunks);
    let vertices = u32_at(&bytes, 0) as usize;
    let index_count = u32_at(&bytes, 4) as usize;
    let mut positions = Vec::with_capacity(vertices);
    let mut colors = Vec::with_capacity(vertices);
    let mut at = 8usize;
    for _ in 0..vertices {
        positions.push([
            i16_at(&bytes, at) as f32 / 4096.0,
            i16_at(&bytes, at + 2) as f32 / 4096.0,
            i16_at(&bytes, at + 4) as f32 / 4096.0,
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

    let mut normals = vec![Vec3::ZERO; vertices];
    for triangle in indices.chunks_exact(3) {
        let a = Vec3::from_array(positions[triangle[0] as usize]);
        let b = Vec3::from_array(positions[triangle[1] as usize]);
        let c = Vec3::from_array(positions[triangle[2] as usize]);
        let normal = (b - a).cross(c - a);
        if normal.length_squared() > 1e-10 {
            for index in triangle { normals[*index as usize] += normal; }
        }
    }
    let normals: Vec<[f32; 3]> = normals.into_iter()
        .map(|normal| normal.try_normalize().unwrap_or(Vec3::Y).to_array())
        .collect();

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
        .with_inserted_indices(Indices::U32(indices))
}

pub fn house() -> Mesh { unpack(HOUSE) }
pub fn girl() -> Mesh { unpack(GIRL) }
