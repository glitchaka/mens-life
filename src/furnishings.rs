//! Interior furnishing and interaction anchors for the imported Autumn House.
//! Architecture comes from `house`; this module owns only furniture/decor and
//! the physical obstacles associated with those objects.
use crate::scene::SceneSpec;
use serde_json::json;

fn obstacle(s: &mut SceneSpec, id: String) -> String {
    s.obstacles.push(id.clone());
    id
}

fn last_obstacle(s: &mut SceneSpec) {
    let id = s.objects.last()["id"].as_str().unwrap().to_owned();
    s.obstacles.push(id);
}

fn box_obstacle(
    s: &mut SceneSpec,
    size: [f64; 3],
    at: [f64; 3],
    color: u32,
    name: &str,
) -> String {
    let id = s.box_mesh(size, at, color, name);
    obstacle(s, id)
}

fn glass(s: &mut SceneSpec, size: [f64; 3], at: [f64; 3]) {
    s.box_mesh(size, at, 0xc7dde0, "");
    s.last()["material"] = json!({
        "type":"standard","color":0xc7dde0,"roughness":0.18,
        "transparent":true,"opacity":0.34
    });
    s.last()["cast_shadow"] = json!(false);
}

fn rug(s: &mut SceneSpec, size: [f64; 3], at: [f64; 3], color: u32) {
    s.box_mesh(size, at, color, "");
    s.last()["cast_shadow"] = json!(false);
}

/// Furnishes the real imported shell while preserving the task coordinates used
/// by the simulation. Named meshes are clickable interaction anchors.
pub(crate) fn append(s: &mut SceneSpec) {
    // Interior floor under the supplied shell. The shell remains the visible
    // architectural authority; this simply gives the characters a readable floor.
    s.box_mesh([10.35, 0.12, 8.10], [-1.02, -0.08, -0.40], 0xb78f62, "");
    s.last()["material"] = json!({"type":"standard","color":0xb78f62,"roughness":0.93});

    // Bedroom — task pose (-4.6, 2.1), approach (-2.8, 2.2).
    rug(s, [2.85, 0.035, 2.25], [-4.42, 0.015, 2.18], 0x7f7566);
    box_obstacle(s, [2.35, 0.42, 1.45], [-4.58, 0.38, 2.12], 0x6f5545, "bed");
    s.last()["material"] = json!({"type":"standard","color":0x6f5545,"roughness":0.88});
    s.box_mesh([2.18, 0.28, 1.30], [-4.58, 0.68, 2.12], 0xd5c5ad, "");
    s.last()["cast_shadow"] = json!(true);
    s.box_mesh([2.25, 0.92, 0.16], [-4.58, 0.85, 2.78], 0x55453c, "");
    // Kept behind the bed approach lane so the pathfinder can actually reach it.
    s.box_mesh([0.72, 0.46, 0.58], [-3.04, 0.28, 3.12], 0x5e5045, "");
    last_obstacle(s);
    s.cyl(0.19, 0.52, [-3.04, 0.82, 3.12], 0xc8aa72, "");

    // Living area — the existing TV animation faces -Z, so the screen is south
    // of the sofa and the sitting pose remains physically coherent.
    rug(s, [4.20, 0.035, 2.65], [1.60, 0.018, 2.10], 0x768277);
    box_obstacle(s, [2.55, 0.72, 0.90], [2.55, 0.42, 3.02], 0x667363, "");
    s.box_mesh([2.25, 0.72, 0.32], [2.55, 0.82, 3.36], 0x596759, "");
    s.box_mesh([0.30, 0.86, 0.86], [1.38, 0.55, 3.02], 0x596759, "");
    s.box_mesh([0.30, 0.86, 0.86], [3.72, 0.55, 3.02], 0x596759, "");
    // Visual table, deliberately not navigation-blocking: Alex initially stands nearby.
    s.box_mesh([1.50, 0.34, 0.72], [0.85, 0.26, 2.12], 0x765d4a, "");
    s.box_mesh([1.82, 1.05, 0.12], [2.60, 1.18, 1.10], 0x202a2c, "tv");
    s.last()["material"] = json!({"type":"standard","color":0x17212b,"roughness":0.30,"emissive":0x101619});
    s.last()["id"] = json!("tv-screen");
    s.box_mesh([2.25, 0.52, 0.42], [2.60, 0.30, 1.12], 0x5b4d42, "");
    last_obstacle(s);

    // Kitchen — sink is the Eat interaction anchor consumed by native.rs.
    box_obstacle(s, [2.10, 0.94, 0.72], [3.02, 0.48, -1.77], 0x596b67, "sink");
    s.box_mesh([2.18, 0.12, 0.80], [3.02, 1.00, -1.77], 0xe0d8c4, "");
    s.box_mesh([0.72, 1.92, 0.82], [3.65, 0.96, -3.33], 0xd8d5c8, "");
    last_obstacle(s);
    s.box_mesh([2.15, 0.88, 0.65], [0.66, 0.45, -3.48], 0x64736d, "");
    last_obstacle(s);
    s.box_mesh([2.22, 0.12, 0.73], [0.66, 0.93, -3.48], 0xd8cfb7, "");
    // Dining surface and chairs remain clear of the Eat approach at (3.05,-1.05).
    box_obstacle(s, [1.65, 0.16, 1.05], [0.30, 0.78, -1.35], 0x74553e, "");
    for (x, z) in [(-0.65, -1.35), (1.25, -1.35), (0.30, -2.12)] {
        s.box_mesh([0.52, 0.62, 0.52], [x, 0.33, z], 0x5f5246, "");
        last_obstacle(s);
    }

    // Bathroom — click targets align with the original task approach/pose data.
    rug(s, [3.45, 0.03, 1.75], [-4.32, 0.016, -3.63], 0x6f807d);
    s.box_mesh([0.96, 0.10, 0.92], [-5.90, 0.05, -4.02], 0x9eb7b5, "shower");
    glass(s, [0.06, 2.05, 1.05], [-6.15, 1.03, -3.98]);
    glass(s, [0.82, 2.05, 0.06], [-5.78, 1.03, -4.43]);
    let toilet = s.cyl(0.32, 0.48, [-3.65, 0.25, -4.08], 0xe5e0d4, "toilet");
    obstacle(s, toilet);
    s.box_mesh([0.62, 0.72, 0.22], [-3.65, 0.60, -4.42], 0xe0dcd2, "");
    box_obstacle(s, [1.05, 0.82, 0.48], [-2.60, 0.43, -3.78], 0x6a726c, "");
    s.box_mesh([1.10, 0.10, 0.54], [-2.60, 0.89, -3.78], 0xdad2be, "");
    s.box_mesh([0.82, 0.72, 0.04], [-2.60, 1.45, -4.02], 0x9fb4b3, "");
    s.last()["material"] = json!({"type":"standard","color":0x9fb4b3,"roughness":0.12,"metallic":0.15});

    // Restrained accents so the imported shell reads as a lived-in home.
    s.cyl(0.24, 0.46, [-1.30, 0.23, 2.65], 0x6a5140, "");
    s.mesh("cone", &[0.42, 1.15, 12.0], [-1.30, 1.02, 2.65], 0x607252, 0.92, "", [true, false]);
    s.cyl(0.18, 0.42, [2.10, 0.21, -3.85], 0x6a5140, "");
    s.mesh("cone", &[0.34, 0.88, 12.0], [2.10, 0.84, -3.85], 0x617551, 0.92, "", [true, false]);
}
