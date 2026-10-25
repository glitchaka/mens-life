//! Collision/navigation integration for the house supplied by the user.
//! The visible geometry is the baked FBX; this module owns only invisible
//! collision volumes and deliberately leaves the front patio entrance open.
use crate::scene::SceneSpec;
use serde_json::json;

fn wall(s: &mut SceneSpec, size: [f64; 3], at: [f64; 3]) {
    let id = s.box_mesh(size, at, 0xffffff, "");
    s.last()["visible"] = json!(false);
    s.obstacles.push(id);
}

pub(crate) fn append(s: &mut SceneSpec) {
    // Bounds follow the supplied FBX rather than the old procedural room.
    wall(s, [0.16, 2.8, 9.45], [-6.92, 1.35, -0.45]);
    wall(s, [0.16, 2.8, 9.45], [4.02, 1.35, -0.45]);
    wall(s, [10.95, 2.8, 0.16], [-1.45, 1.35, -5.18]);

    // Camera/front side is split around the real access route. The wide gap
    // connects the interior navigation mesh to the patio instead of sealing it.
    wall(s, [3.25, 2.8, 0.16], [-5.28, 1.35, 4.02]);
    wall(s, [3.25, 2.8, 0.16], [2.39, 1.35, 4.02]);
}
