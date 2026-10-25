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
    // Bedroom: imported bed.glb.
    proxy(s, [1.42, 0.85, 1.78], [-4.03, 0.84, 2.08], "bed", true);

    // Living room: imported sofa + small table. TV remains a lightweight host object
    // because no TV asset was supplied; its screen is the interaction anchor.
    proxy(s, [1.86, 0.90, 0.78], [-3.67, 0.85, 0.35], "", true);
    proxy(s, [0.58, 0.62, 0.42], [-2.55, 0.72, 0.38], "", true);
    let tv = s.box_mesh([1.15, 0.68, 0.08], [-2.15, 1.18, -0.05], 0x17212b, "tv");
    s.last()["id"] = json!("tv-screen");
    s.last()["material"] = json!({"type":"standard","color":0x17212b,"roughness":0.30,"emissive":0x101619});
    s.obstacles.push(tv);

    // Dining: classic table and four copies of the supplied chair.
    proxy(s, [0.82, 0.78, 1.22], [-2.20, 0.82, -1.12], "sink", true);
    for (x, z) in [(-2.90,-1.12),(-1.50,-1.12),(-2.20,-1.88),(-2.20,-0.38)] {
        proxy(s, [0.42, 0.82, 0.46], [x, 0.82, z], "", true);
    }

    // Bathroom furniture set. Separate invisible task anchors sit inside the same volume.
    proxy(s, [1.38, 1.30, 1.38], [-4.02, 1.05, -2.27], "", true);
    proxy(s, [0.50, 1.00, 0.54], [-4.18, 0.92, -2.58], "shower", false);
    proxy(s, [0.46, 0.68, 0.46], [-3.56, 0.76, -2.32], "toilet", false);
}
