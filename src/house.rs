//! Gameplay integration for the imported dwelling.
//! The FBX-derived GLB is the sole visual authority. This module only defines
//! invisible interaction volumes and conservative navigation collision.
use crate::scene::SceneSpec;
use serde_json::json;

/// Converted directly from `House_scene_01.fbx` supplied in `autumn-house.zip`.
pub const VISUAL_ASSET: &str = "house/autumn_house.glb#Scene0";

fn proxy(s: &mut SceneSpec, size: [f64; 3], at: [f64; 3], name: &str, obstacle: bool) {
    let id = s.box_mesh(size, at, 0xffffff, name);
    s.last()["visible"] = json!(false);
    if obstacle {
        s.obstacles.push(id);
    }
}

/// Invisible geometry aligned to the existing task poses. Rendering comes only
/// from [`VISUAL_ASSET`], so none of these boxes can visually replace or cover
/// the house supplied by the user.
pub(crate) fn append(s: &mut SceneSpec) {
    // Outer transformed footprint. The front is deliberately split to keep an
    // entrance and free circulation for both the protagonist and residents.
    proxy(s, [0.18, 2.8, 7.75], [-6.18, 1.35, -0.42], "", true);
    proxy(s, [0.18, 2.8, 7.75], [3.72, 1.35, -0.42], "", true);
    proxy(s, [9.95, 2.8, 0.18], [-1.23, 1.35, -4.31], "", true);
    proxy(s, [3.45, 2.8, 0.18], [-4.45, 1.35, 3.49], "", true);
    proxy(s, [3.45, 2.8, 0.18], [2.02, 1.35, 3.49], "", true);

    // Interactive furniture volumes. Their names are the host-facing task IDs
    // consumed by native.rs. They are hidden but still ray-castable.
    proxy(s, [2.75, 1.05, 3.25], [-4.62, 0.52, 2.78], "bed", true);
    proxy(s, [2.72, 2.10, 0.52], [2.60, 1.05, 0.66], "tv", true);
    proxy(s, [1.55, 2.35, 1.42], [-5.92, 1.17, -4.08], "shower", true);
    proxy(s, [0.78, 1.10, 0.82], [-3.65, 0.55, -4.25], "toilet", true);
    proxy(s, [3.12, 1.10, 0.92], [3.02, 0.55, -2.12], "sink", true);

    // A couple of large fixed masses prevent pathfinding from cutting through
    // the imported kitchen/cabinetry while leaving the task approach points free.
    proxy(s, [2.70, 1.05, 0.72], [2.90, 0.52, -4.12], "", true);
}
