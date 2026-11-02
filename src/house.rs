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
    // The imported GLB is centred and scaled to a roughly 27.4 x 32.1 metre interior.
    // Keep the simulation inside that footprint without drawing a second house shell.
    boundary(s, [0.40, 3.20, 32.40], [-13.88, 1.60, 0.0], "house-boundary-west");
    boundary(s, [0.40, 3.20, 32.40], [ 13.88, 1.60, 0.0], "house-boundary-east");
    boundary(s, [27.40, 3.20, 0.40], [0.0, 1.60, -16.24], "house-boundary-south");
    boundary(s, [27.40, 3.20, 0.40], [0.0, 1.60,  16.24], "house-boundary-north");
}
