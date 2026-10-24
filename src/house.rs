//! Imported Autumn House integration.
//! Visual architecture is converted from the supplied `House_scene_01.fbx`;
//! this module owns only structural collision proxies.
use crate::scene::SceneSpec;
use serde_json::json;

pub const VISUAL_ASSET: &str = "house/autumn_house.gltf#Scene0";

fn wall(s: &mut SceneSpec, size: [f64; 3], at: [f64; 3]) {
    let id = s.box_mesh(size, at, 0xffffff, "");
    s.last()["visible"] = json!(false);
    s.obstacles.push(id);
}

/// Conservative collision around the imported shell. Furniture collision and
/// click targets belong to `furnishings`, not to the architectural asset layer.
pub(crate) fn append(s: &mut SceneSpec) {
    wall(s, [0.16, 2.8, 8.10], [-6.22, 1.35, -0.40]);
    wall(s, [0.16, 2.8, 8.10], [4.20, 1.35, -0.40]);
    wall(s, [10.50, 2.8, 0.16], [-1.01, 1.35, -4.48]);
    // Camera-side wall is split so the front opening remains traversable.
    wall(s, [3.80, 2.8, 0.16], [-4.25, 1.35, 3.68]);
    wall(s, [3.80, 2.8, 0.16], [2.25, 1.35, 3.68]);
}
