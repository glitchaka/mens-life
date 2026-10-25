//! Lightweight interaction proxies retained while the new furnished house is calibrated.
//! They are deliberately non-blocking so stale Autumn House coordinates cannot create
//! invisible navigation walls inside the replacement model.
use crate::scene::SceneSpec;
use serde_json::json;

fn proxy(s: &mut SceneSpec, size: [f64; 3], at: [f64; 3], name: &str) {
    s.box_mesh(size, at, 0xffffff, name);
    s.last()["visible"] = json!(false);
}

pub(crate) fn append(s: &mut SceneSpec) {
    proxy(s, [1.42, 0.85, 1.78], [-4.03, 0.43, 2.08], "bed");
    proxy(s, [1.84, 0.80, 0.74], [-3.67, 0.40, 0.35], "");
    proxy(s, [0.57, 0.60, 0.41], [-2.55, 0.30, 0.38], "");

    s.box_mesh([1.15, 0.68, 0.08], [-2.15, 1.18, -0.05], 0x17212b, "tv");
    s.last()["id"] = json!("tv-screen");
    s.last()["visible"] = json!(false);
    s.last()["material"] = json!({"type":"standard","color":0x17212b,"roughness":0.30,"emissive":0x101619});

    proxy(s, [0.74, 0.69, 1.19], [-2.20, 0.345, -1.12], "sink");
    for (x, z) in [(-2.90,-1.12),(-1.50,-1.12),(-2.20,-1.88),(-2.20,-0.38)] {
        proxy(s, [0.46, 0.98, 0.52], [x, 0.49, z], "");
    }

    proxy(s, [1.80, 1.74, 1.80], [-4.02, 0.87, -2.27], "");
    proxy(s, [0.50, 1.00, 0.54], [-4.18, 0.50, -2.58], "shower");
    proxy(s, [0.46, 0.68, 0.46], [-3.56, 0.34, -2.32], "toilet");
}
