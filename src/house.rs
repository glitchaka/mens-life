//! Collision/navigation integration for the real imported Autumn House.
//! Visible architecture comes only from the supplied GLB. These volumes follow
//! the actual shell bounds and keep the real east-side front door open.
use crate::scene::SceneSpec;
use serde_json::json;

fn wall(s: &mut SceneSpec, size: [f64; 3], at: [f64; 3]) {
    let id = s.box_mesh(size, at, 0xffffff, "");
    s.last()["visible"] = json!(false);
    s.obstacles.push(id);
}

pub(crate) fn append(s: &mut SceneSpec) {
    // Actual cutaway shell bounds read from the imported house GLB:
    // west x≈-4.52, east x≈-1.43, north z≈-3.30, south z≈3.37.
    wall(s, [0.14, 2.8, 6.67], [-4.52, 1.40, 0.035]);
    wall(s, [3.63, 2.8, 0.14], [-2.975, 1.40, -3.30]);
    wall(s, [3.63, 2.8, 0.14], [-2.975, 1.40, 3.37]);

    // East wall split around the actual House_door span (z≈0.12..0.86).
    // This gap joins the indoor navigation grid to the veranda, yard and road.
    wall(s, [0.14, 2.8, 3.05], [-1.43, 1.40, -1.535]);
    wall(s, [0.14, 2.8, 2.18], [-1.43, 1.40, 2.04]);
}
