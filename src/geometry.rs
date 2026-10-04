//! Procedural native triangle meshes. Dimensions and vertex rounding follow
//! the original room; no model files or browser renderer are needed.
use crate::{
    game::{Bounds, Vec3 as Point},
    scene::SceneSpec,
};
use bevy::math::{DMat4, DQuat, DVec3};
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use serde_json::Value;
use std::f64::consts::{PI, TAU};

#[derive(Default)]
pub struct Geometry {
    pub positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}
impl Geometry {
    fn vertex(&mut self, p: DVec3, n: DVec3, uv: [f32; 2]) {
        self.positions.push(p.as_vec3().to_array());
        self.normals.push(n.as_vec3().to_array());
        self.uvs.push(uv);
    }
    pub fn mesh(self) -> Mesh {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs)
        .with_inserted_indices(Indices::U32(self.indices))
    }
}
fn box_mesh(size: DVec3, segments: usize, radius: f64) -> Geometry {
    let mut g = Geometry::default();
    let faces = [
        (DVec3::X, -DVec3::Z, DVec3::Y),
        (-DVec3::X, DVec3::Z, DVec3::Y),
        (DVec3::Y, DVec3::X, -DVec3::Z),
        (-DVec3::Y, DVec3::X, DVec3::Z),
        (DVec3::Z, DVec3::X, DVec3::Y),
        (-DVec3::Z, -DVec3::X, DVec3::Y),
    ];
    for (normal, u, v) in faces {
        let base = g.positions.len() as u32;
        for y in 0..=segments {
            for x in 0..=segments {
                let p = (normal * 0.5
                    + u * (x as f64 / segments as f64 - 0.5)
                    + v * (y as f64 / segments as f64 - 0.5))
                    .as_vec3()
                    .as_dvec3();
                let (point, n) = if radius > 0.0 {
                    let sign = p.signum();
                    let n = (p - sign * (0.5 / segments as f64)).normalize();
                    ((size * 0.5 - DVec3::splat(radius)) * sign + n * radius, n)
                } else {
                    (p * size, normal)
                };
                g.vertex(
                    point,
                    n,
                    [x as f32 / segments as f32, y as f32 / segments as f32],
                );
            }
        }
        let stride = (segments + 1) as u32;
        for y in 0..segments as u32 {
            for x in 0..segments as u32 {
                let a = base + y * stride + x;
                g.indices
                    .extend([a, a + 1, a + stride + 1, a, a + stride + 1, a + stride]);
            }
        }
    }
    g
}
fn sphere(
    radius: f64,
    width: usize,
    height: usize,
    theta_length: f64,
    capsule: Option<(f64, usize)>,
) -> Geometry {
    let mut g = Geometry::default();
    let rows: Vec<(f64, f64)> = if let Some((length, cap)) = capsule {
        (0..=cap)
            .map(|i| (i as f64 / cap as f64 * PI / 2.0, length / 2.0))
            .chain((0..=cap).map(|i| (PI / 2.0 + i as f64 / cap as f64 * PI / 2.0, -length / 2.0)))
            .collect()
    } else {
        (0..=height)
            .map(|i| (i as f64 / height as f64 * theta_length, 0.0))
            .collect()
    };
    for (y, &(theta, offset)) in rows.iter().enumerate() {
        for x in 0..=width {
            let phi = x as f64 / width as f64 * TAU;
            let n = DVec3::new(
                -phi.cos() * theta.sin(),
                theta.cos(),
                phi.sin() * theta.sin(),
            );
            g.vertex(
                n * radius + DVec3::Y * offset,
                n,
                [x as f32 / width as f32, y as f32 / (rows.len() - 1) as f32],
            );
        }
    }
    let stride = (width + 1) as u32;
    for y in 0..rows.len() - 1 {
        for x in 0..width {
            let a = y as u32 * stride + x as u32;
            if y != 0 {
                g.indices.extend([a + 1, a, a + stride + 1]);
            }
            if y != rows.len() - 2 || theta_length < PI {
                g.indices.extend([a, a + stride, a + stride + 1]);
            }
        }
    }
    g
}
fn cylinder(top: f64, bottom: f64, height: f64, segments: usize) -> Geometry {
    let mut g = Geometry::default();
    for y in 0..=1 {
        let r = if y == 0 { top } else { bottom };
        for x in 0..=segments {
            let theta = x as f64 / segments as f64 * TAU;
            let n = DVec3::new(theta.sin(), (bottom - top) / height, theta.cos()).normalize();
            g.vertex(
                DVec3::new(r * theta.sin(), height * (0.5 - y as f64), r * theta.cos()),
                n,
                [x as f32 / segments as f32, y as f32],
            );
        }
    }
    let stride = (segments + 1) as u32;
    for x in 0..segments as u32 {
        g.indices
            .extend([x + 1, x, x + stride + 1, x, x + stride, x + stride + 1]);
    }
    for (r, sign) in [(top, 1.0), (bottom, -1.0)] {
        if r == 0.0 {
            continue;
        }
        let base = g.positions.len() as u32;
        g.vertex(DVec3::Y * height * 0.5 * sign, DVec3::Y * sign, [0.5, 0.5]);
        for x in 0..=segments {
            let theta = x as f64 / segments as f64 * TAU;
            g.vertex(
                DVec3::new(r * theta.sin(), height * 0.5 * sign, r * theta.cos()),
                DVec3::Y * sign,
                [0.5, 0.5],
            );
        }
        for x in 0..segments as u32 {
            if sign > 0.0 {
                g.indices.extend([base, base + x + 1, base + x + 2]);
            } else {
                g.indices.extend([base, base + x + 2, base + x + 1]);
            }
        }
    }
    g
}
fn octahedron(radius: f64) -> Geometry {
    let mut g = Geometry::default();
    for sign in [1.0, -1.0] {
        for (a, b) in [
            (DVec3::X, DVec3::Z),
            (DVec3::Z, -DVec3::X),
            (-DVec3::X, -DVec3::Z),
            (-DVec3::Z, DVec3::X),
        ] {
            let (a, b) = if sign > 0.0 { (b, a) } else { (a, b) };
            let apex = DVec3::Y * sign;
            let n = (a - apex).cross(b - apex).normalize();
            let base = g.positions.len() as u32;
            for p in [apex, a, b] {
                g.vertex(p * radius, n, [0.0; 2]);
            }
            g.indices.extend([base, base + 1, base + 2]);
        }
    }
    g
}
pub fn build(node: &Value) -> Geometry {
    let args: Vec<f64> = node["args"]
        .as_array()
        .unwrap()
        .iter()
        .map(|n| n.as_f64().unwrap())
        .collect();
    match node["kind"].as_str().unwrap() {
        "rounded-box" => box_mesh(
            DVec3::new(args[0], args[1], args[2]),
            args[3] as usize * 2 + 1,
            args[4],
        ),
        "box" => box_mesh(DVec3::new(args[0], args[1], args[2]), 1, 0.0),
        "sphere" => sphere(
            args[0],
            args[1] as usize,
            args[2] as usize,
            args.get(6).copied().unwrap_or(PI),
            None,
        ),
        "capsule" => sphere(
            args[0],
            args[3] as usize,
            args[2] as usize * 2,
            PI,
            Some((args[1], args[2] as usize)),
        ),
        "cylinder" => cylinder(args[0], args[1], args[2], args[3] as usize),
        "cone" => cylinder(0.0, args[0], args[1], args[2] as usize),
        "octahedron" => octahedron(args[0]),
        kind => panic!("Unknown native geometry: {kind}"),
    }
}
pub fn vector(value: &Value, fallback: [f64; 3]) -> DVec3 {
    let a = std::array::from_fn(|i| value[i].as_f64().unwrap_or(fallback[i]));
    DVec3::from_array(a)
}
pub fn local_transform(node: &Value) -> DMat4 {
    let r = vector(&node["rotation"], [0.0; 3]);
    DMat4::from_scale_rotation_translation(
        vector(&node["scale"], [1.0; 3]),
        DQuat::from_euler(EulerRot::XYZ, r.x, r.y, r.z),
        vector(&node["position"], [0.0; 3]),
    )
}
/// Build navigation from the actual native vertices, retaining double precision
/// world transforms at cell boundaries as in the reference simulation.
pub fn navigation_bounds(spec: &SceneSpec) -> Vec<Bounds> {
    spec.objects
        .iter()
        .filter(|n| spec.obstacles.iter().any(|id| n["id"] == *id))
        .filter_map(|node| {
            let matrix = local_transform(node);
            let mut local_min = DVec3::splat(f64::INFINITY);
            let mut local_max = DVec3::splat(f64::NEG_INFINITY);
            for p in build(node).positions {
                let p = Vec3::from_array(p).as_dvec3();
                local_min = local_min.min(p);
                local_max = local_max.max(p);
            }
            // The original navigation transforms the local bounding box, which
            // conservatively encloses rotated plant leaves and rounded meshes.
            let mut min = DVec3::splat(f64::INFINITY);
            let mut max = DVec3::splat(f64::NEG_INFINITY);
            for x in [local_min.x, local_max.x] {
                for y in [local_min.y, local_max.y] {
                    for z in [local_min.z, local_max.z] {
                        let p = matrix.transform_point3(DVec3::new(x, y, z));
                        min = min.min(p);
                        max = max.max(p);
                    }
                }
            }
            (min.y < 1.6 && max.y > 0.3).then_some(Bounds {
                min: Point::new(min.x, min.y, min.z),
                max: Point::new(max.x, max.y, max.z),
            })
        })
        .collect()
}
