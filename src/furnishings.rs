//! Invisible interaction/navigation proxies for the imported GLB furniture.
//! Visible furniture is spawned from the user-provided assets in native_scene.
use crate::scene::SceneSpec;
use serde_json::json;

fn proxy(s: &mut SceneSpec, size: [f64; 3], at: [f64; 3], name: &str, obstacle: bool) {
    let id = s.box_mesh(size, at, 0xffffff, name);
    s.last()["visible"] = json!(false);
    if obstacle { s.obstacles.push(id); }
}

pub(crate) fn append(s: &mut SceneSpec) {
    // Bedroom: bed.glb scaled to ~1.41 x 1.77 m.
    proxy(s, [1.42, 0.85, 1.78], [-4.03, 0.43, 2.08], "bed", true);

    // Living room: sofa ~1.82 m wide and supplied small table ~0.57 m wide.
    proxy(s, [1.84, 0.80, 0.74], [-3.67, 0.40, 0.35], "", true);
    proxy(s, [0.57, 0.60, 0.41], [-2.55, 0.30, 0.38], "", true);
    let tv = s.box_mesh([1.15, 0.68, 0.08], [-2.15, 1.18, -0.05], 0x17212b, "tv");
    s.last()["id"] = json!("tv-screen");
    s.last()["material"] = json!({"type":"standard","color":0x17212b,"roughness":0.30,"emissive":0x101619});
    s.obstacles.push(tv);

    // Dining: classic table ~0.73 x 1.18 m, height ~0.68 m; four real chair copies.
    proxy(s, [0.74, 0.69, 1.19], [-2.20, 0.345, -1.12], "sink", true);
    for (x, z) in [(-2.90,-1.12),(-1.50,-1.12),(-2.20,-1.88),(-2.20,-0.38)] {
        proxy(s, [0.46, 0.98, 0.52], [x, 0.49, z], "", true);
    }

    // Bathroom set stays visually intact. Do not treat the whole 1.8 m set as one
    // solid obstacle: its shower/toilet live inside that footprint and must remain reachable.
    proxy(s, [1.80, 1.74, 1.80], [-4.02, 0.87, -2.27], "", false);
    proxy(s, [0.50, 1.00, 0.54], [-4.18, 0.50, -2.58], "shower", false);
    proxy(s, [0.46, 0.68, 0.46], [-3.56, 0.34, -2.32], "toilet", false);
}
