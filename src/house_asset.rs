//! Baked from the user-provided `House_scene_01.fbx`.
//! The FBX geometry is flattened into low-poly triangles; colors are sampled from its original texture maps.
use crate::geometry::Geometry;

const POSITION_SCALE: f32 = 1.0 / 4096.0;
const LOWER: &[&str] = &[
    include_str!("house_data/lower_0.txt"),
    include_str!("house_data/lower_1.txt"),
    include_str!("house_data/lower_2.txt"),
    include_str!("house_data/lower_3.txt"),
];
const UPPER: &[&str] = &[
    include_str!("house_data/upper_0.txt"),
    include_str!("house_data/upper_1.txt"),
];

fn sextet(b: u8) -> Option<u8> {
    match b {
        b'A'..=b'Z' => Some(b-b'A'), b'a'..=b'z' => Some(b-b'a'+26),
        b'0'..=b'9' => Some(b-b'0'+52), b'+' => Some(62), b'/' => Some(63), _ => None,
    }
}

fn decode64(chunks: &[&str]) -> Vec<u8> {
    let capacity=chunks.iter().map(|s| s.len()).sum::<usize>()*3/4;
    let mut out=Vec::with_capacity(capacity); let mut acc=0u32; let mut bits=0u8;
    for input in chunks {
        for b in input.bytes() {
            if b==b'=' { break; }
            let Some(v)=sextet(b) else { continue; };
            acc=(acc<<6)|v as u32; bits+=6;
            if bits>=8 { bits-=8; out.push((acc>>bits) as u8); acc &= (1u32<<bits)-1; }
        }
    }
    out
}

fn unpack(encoded: &[&str]) -> Geometry {
    let bytes=decode64(encoded); const RECORD:usize=25; let triangles=bytes.len()/RECORD;
    let mut positions=Vec::with_capacity(triangles*3); let mut normals=Vec::with_capacity(triangles*3);
    let mut colors=Vec::with_capacity(triangles*3); let mut indices=Vec::with_capacity(triangles*3);
    for chunk in bytes.chunks_exact(RECORD) {
        let mut at=0usize; let mut tri=[[0.0f32;3];3];
        for p in &mut tri { for axis in 0..3 {
            let q=i16::from_le_bytes([chunk[at],chunk[at+1]]); at+=2; p[axis]=q as f32*POSITION_SCALE;
        } }
        let n=[chunk[18] as i8 as f32/127.0,chunk[19] as i8 as f32/127.0,chunk[20] as i8 as f32/127.0];
        let c=[chunk[21] as f32/255.0,chunk[22] as f32/255.0,chunk[23] as f32/255.0,chunk[24] as f32/255.0];
        for p in tri { positions.push(p); normals.push(n); colors.push(c); indices.push(indices.len() as u32); }
    }
    Geometry::from_raw(positions,normals,Vec::new(),colors,indices)
}

pub(crate) fn lower() -> Geometry { unpack(LOWER) }
pub(crate) fn upper() -> Geometry { unpack(UPPER) }
