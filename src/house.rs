//! Supplied Autumn House integration.
//! Visual geometry is baked directly from `House_scene_01.fbx`; this module owns
//! only the architectural shell and conservative structural collision proxies.
use crate::scene::SceneSpec;
use serde_json::json;

fn wall(s: &mut SceneSpec, size: [f64; 3], at: [f64; 3]) {
    let id = s.box_mesh(size, at, 0xffffff, "");
    s.last()["visible"] = json!(false);
    s.obstacles.push(id);
}

pub(crate) fn append(s: &mut SceneSpec) {
    s.objects.push(json!({
        "id":"house-lower",
        "kind":"house-lower",
        "position":[0.0,0.0,0.0],
        "material":{"type":"standard","color":0xffffff,"roughness":0.82},
        "cast_shadow":true,"receive_shadow":true
    }));
    // Roof/upper storey stay visible as context but transparent enough for the
    // isometric camera to read the interactive interior beneath them.
    s.objects.push(json!({
        "id":"house-upper",
        "kind":"house-upper",
        "position":[0.0,0.0,0.0],
        "material":{
            "type":"standard","color":0xffffff,"roughness":0.82,
            "transparent":true,"opacity":0.22
        },
        "cast_shadow":false,"receive_shadow":false
    }));

    // Navigation remains independent from render geometry. These invisible
    // proxies follow the usable footprint and keep the front opening traversable.
    wall(s, [0.16, 2.8, 8.10], [-6.22, 1.35, -0.40]);
    wall(s, [0.16, 2.8, 8.10], [4.20, 1.35, -0.40]);
    wall(s, [10.50, 2.8, 0.16], [-1.01, 1.35, -4.48]);
    wall(s, [3.80, 2.8, 0.16], [-4.25, 1.35, 3.68]);
    wall(s, [3.80, 2.8, 0.16], [2.25, 1.35, 3.68]);
}
