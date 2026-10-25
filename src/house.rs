//! Invisible navigation/collision proxies for the original Autumn House asset.
//! The visible architecture is never reconstructed here.
use crate::scene::SceneSpec;
use serde_json::json;

fn wall(s: &mut SceneSpec, size: [f64; 3], at: [f64; 3]) {
    let id = s.box_mesh(size, at, 0xffffff, "");
    s.last()["visible"] = json!(false);
    s.obstacles.push(id);
}

pub(crate) fn append(s: &mut SceneSpec) {
    // House_base in the supplied model spans approximately x -4.79..-1.16,
    // z -3.30..3.37. The real front door sits on the east wall around z 0.49.
    wall(s, [0.14, 2.8, 6.72], [-4.82, 1.45, 0.04]);
    wall(s, [3.72, 2.8, 0.14], [-2.98, 1.45, -3.34]);
    wall(s, [3.72, 2.8, 0.14], [-2.98, 1.45, 3.41]);

    // East wall split around the actual door: interior and patio remain connected.
    wall(s, [0.14, 2.8, 3.10], [-1.13, 1.45, -1.70]);
    wall(s, [0.14, 2.8, 2.12], [-1.13, 1.45, 2.31]);
}
