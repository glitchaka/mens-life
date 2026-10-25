//! Collision/navigation integration for the house supplied by the user.
//! The visible geometry is spawned from `packed_mesh::house`; this module owns
//! only invisible collision so the patio and doorway remain traversable.
use crate::scene::SceneSpec;
use serde_json::json;

fn wall(s: &mut SceneSpec, size: [f64; 3], at: [f64; 3]) {
    let id = s.box_mesh(size, at, 0xffffff, "");
    s.last()["visible"] = json!(false);
    s.obstacles.push(id);
}

pub(crate) fn append(s: &mut SceneSpec) {
    wall(s, [0.16, 2.8, 8.10], [-6.22, 1.35, -0.40]);
    wall(s, [0.16, 2.8, 8.10], [4.20, 1.35, -0.40]);
    wall(s, [10.50, 2.8, 0.16], [-1.01, 1.35, -4.48]);
    // Split camera/front wall: the gap is the physical interior <-> patio passage.
    wall(s, [3.80, 2.8, 0.16], [-4.25, 1.35, 3.68]);
    wall(s, [3.80, 2.8, 0.16], [2.25, 1.35, 3.68]);
}
