//! Navigation envelope for the imported PSX Backroom house interior.
//! Visible architecture comes entirely from `assets/house/house.glb`.
use crate::scene::SceneSpec;
use serde_json::json;

fn boundary(s: &mut SceneSpec, size: [f64; 3], at: [f64; 3], id: &str) {
    s.box_mesh(size, at, 0xffffff, "");
    s.last()["id"] = json!(id);
    s.last()["visible"] = json!(false);
    s.obstacles.push(id.to_owned());
}

pub(crate) fn append(s: &mut SceneSpec) {
    // Measured final GLB footprint after its internal Sketchfab/FBX transforms:
    // 46.8974 x 55.0137 metres. The scene root centres that footprint on X/Z.
    let half_x = 23.44871;
    let half_z = 27.50685;
    let thickness = 0.40;
    let wall_height = 4.95;
    let y = wall_height * 0.5;

    boundary(s, [thickness, wall_height, half_z * 2.0 + thickness], [-half_x - thickness * 0.5, y, 0.0], "house-boundary-west");
    boundary(s, [thickness, wall_height, half_z * 2.0 + thickness], [ half_x + thickness * 0.5, y, 0.0], "house-boundary-east");
    boundary(s, [half_x * 2.0 + thickness, wall_height, thickness], [0.0, y, -half_z - thickness * 0.5], "house-boundary-south");
    boundary(s, [half_x * 2.0 + thickness, wall_height, thickness], [0.0, y,  half_z + thickness * 0.5], "house-boundary-north");
}
