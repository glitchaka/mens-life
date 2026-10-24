//! Human proportions, shaped anatomical surfaces and a jointed native rig.
use crate::scene::{Avatar, SceneSpec};
use serde_json::json;

fn part(s: &mut SceneSpec, prefix: &str, id: &str, kind: &str, args: &[f64],
        position: [f64; 3], material: &str, parent: &str) {
    s.person_mesh(&format!("{prefix}{id}"), kind, args, position,
        &format!("{prefix}{material}"), &format!("{prefix}{parent}"));
}
fn group(s: &mut SceneSpec, prefix: &str, id: &str, p: [f64; 3], parent: &str) {
    s.group(&format!("{prefix}{id}"), p, Some(&format!("{prefix}{parent}")));
}
fn shaped(s: &mut SceneSpec, prefix: &str, id: &str, rows: &[[f64; 4]],
          p: [f64; 3], material: &str, parent: &str) {
    let mut args = vec![24.0];
    args.extend(rows.iter().flatten().copied());
    part(s, prefix, id, "loft", &args, p, material, parent);
}
fn ellipsoid(s: &mut SceneSpec, prefix: &str, id: &str, p: [f64; 3],
             scale: [f64; 3], material: &str, parent: &str) {
    part(s, prefix, id, "sphere", &[1.0, 24.0, 16.0], p, material, parent);
    s.last()["scale"] = json!(scale);
}

pub fn append(s: &mut SceneSpec, avatar: &Avatar, prefix: &str, position: [f64; 3], selected: bool) {
    let start = s.objects.len();
    let wide = match avatar.body.as_str() { "atlético" => 1.13, "delgado" => 0.87, _ => 1.0 };
    let colors = [
        ("skin", avatar.skin.as_str(), 0.82), ("hair", avatar.hair.as_str(), 0.97),
        ("shirt", avatar.shirt.as_str(), 0.95), ("pants", avatar.pants.as_str(), 0.98),
        ("shoes", "#302c2a", 0.9), ("sole", "#b5aaa0", 0.98),
        ("white", "#e1dcd1", 0.9), ("eyes", "#252c2a", 0.75),
        ("lips", "#935c50", 0.88), ("seam", "#64605a", 0.95),
    ];
    for (name, color, roughness) in colors {
        s.materials[format!("{prefix}{name}")] = json!({"type":"standard","color":color,"roughness":roughness});
    }
    s.group(&format!("{prefix}person"), position, None);
    group(s, prefix, "spine", [0.0, 0.94, 0.0], "person");
    shaped(s, prefix, "hips", &[
        [-0.095,0.125*wide,0.09,0.0], [-0.04,0.17*wide,0.112,-0.006],
        [0.04,0.165*wide,0.105,0.0], [0.085,0.145*wide,0.095,0.0],
    ], [0.0,0.94,0.0], "pants", "person");
    shaped(s, prefix, "torso", &[
        [0.04,0.153*wide,0.105,0.0], [0.14,0.15*wide,0.103,0.0],
        [0.25,0.165*wide,0.112,0.008], [0.39,0.20*wide,0.123,0.012],
        [0.51,0.218*wide,0.112,0.0], [0.575,0.165*wide,0.09,-0.005],
        [0.605,0.065,0.058,0.0],
    ], [0.0;3], "shirt", "spine");
    shaped(s, prefix, "neck", &[
        [0.57,0.065,0.06,0.0], [0.63,0.052,0.052,0.0], [0.70,0.055,0.055,0.0],
    ], [0.0;3], "skin", "spine");
    group(s, prefix, "head-bone", [0.0,0.69,0.0], "spine");
    shaped(s, prefix, "head", &[
        [-0.018,0.032,0.042,0.022], [0.015,0.058,0.066,0.014],
        [0.06,0.073,0.079,0.0], [0.11,0.081,0.088,-0.005],
        [0.165,0.082,0.084,-0.008], [0.205,0.068,0.07,-0.01],
        [0.235,0.035,0.04,-0.012], [0.245,0.003,0.004,-0.012],
    ], [0.0;3], "skin", "head-bone");
    for (i, side) in [-1.0,1.0].into_iter().enumerate() {
        ellipsoid(s, prefix, &format!("ear-{i}"), [side*0.083,0.09,-0.003],
            [0.014,0.029,0.017], "skin", "head-bone");
        ellipsoid(s, prefix, &format!("eye-white-{i}"), [side*0.033,0.112,0.076],
            [0.014,0.006,0.007], "white", "head-bone");
        ellipsoid(s, prefix, &format!("eye-{i}"), [side*0.033,0.112,0.082],
            [0.005,0.005,0.003], "eyes", "head-bone");
        ellipsoid(s, prefix, &format!("brow-{i}"), [side*0.034,0.132,0.076],
            [0.018,0.003,0.004], "hair", "head-bone");
    }
    ellipsoid(s, prefix, "nose-bridge", [0.0,0.105,0.084], [0.009,0.027,0.014], "skin", "head-bone");
    ellipsoid(s, prefix, "nose", [0.0,0.083,0.1], [0.014,0.011,0.014], "skin", "head-bone");
    ellipsoid(s, prefix, "mouth", [0.0,0.04,0.078], [0.022,0.0035,0.004], "lips", "head-bone");
    // A shaped crown, sideburns and a swept fringe replace the spherical toy hair cap.
    shaped(s, prefix, "hair", &[
        [0.162,0.082,0.08,-0.017], [0.193,0.082,0.081,-0.014],
        [0.225,0.063,0.065,-0.014], [0.25,0.027,0.033,-0.014],
        [0.255,0.003,0.004,-0.014],
    ], [0.0;3], "hair", "head-bone");
    for (i, side) in [-1.0,1.0].into_iter().enumerate() {
        ellipsoid(s, prefix, &format!("sideburn-{i}"), [side*0.075,0.14,-0.021],
            [0.012,0.035,0.046], "hair", "head-bone");
    }
    ellipsoid(s, prefix, "fringe", [-0.02,0.177,0.068], [0.061,0.018,0.025], "hair", "head-bone");
    s.last()["rotation"] = json!([0.0,0.0,0.2]);
    for (i, side) in [-1.0,1.0].into_iter().enumerate() {
        let leg = format!("leg-{i}");
        let knee = format!("knee-{i}");
        group(s, prefix, &leg, [side*0.092,0.94,0.0], "person");
        shaped(s, prefix, &format!("thigh-{i}"), &[
            [-0.44,0.055,0.056,0.0], [-0.35,0.064,0.068,-0.006],
            [-0.17,0.081*wide,0.085,-0.014], [-0.015,0.084*wide,0.086,0.0],
            [0.02,0.06,0.065,0.0],
        ], [0.0;3], "pants", &leg);
        group(s, prefix, &knee, [0.0,-0.44,0.0], &leg);
        shaped(s, prefix, &format!("calf-{i}"), &[
            [-0.405,0.037,0.04,0.0], [-0.29,0.043,0.045,-0.006],
            [-0.15,0.06,0.06,-0.01], [-0.03,0.057,0.057,0.0], [0.015,0.05,0.05,0.0],
        ], [0.0;3], "pants", &knee);
        part(s, prefix, &format!("shoe-{i}"), "rounded-box", &[0.12,0.09,0.265,3.0,0.025],
            [0.0,-0.448,0.055], "shoes", &knee);
        part(s, prefix, &format!("sole-{i}"), "rounded-box", &[0.125,0.025,0.272,3.0,0.01],
            [0.0,-0.49,0.055], "sole", &knee);
        let arm = format!("arm-{i}");
        let forearm = format!("forearm-{i}");
        group(s, prefix, &arm, [side*0.22*wide,0.53,0.0], "spine");
        shaped(s, prefix, &format!("sleeve-{i}"), &[
            [-0.18,0.053,0.054,0.0], [-0.09,0.065,0.065,0.0],
            [-0.005,0.068,0.067,0.0], [0.035,0.039,0.04,0.0],
        ], [0.0;3], "shirt", &arm);
        shaped(s, prefix, &format!("upper-arm-{i}"), &[
            [-0.30,0.043,0.044,0.0], [-0.22,0.048,0.049,0.0], [-0.16,0.052,0.053,0.0],
        ], [0.0;3], "skin", &arm);
        group(s, prefix, &forearm, [0.0,-0.30,0.0], &arm);
        shaped(s, prefix, &format!("forearm-mesh-{i}"), &[
            [-0.265,0.025,0.027,0.0], [-0.18,0.03,0.033,0.0],
            [-0.07,0.043,0.045,0.0], [0.01,0.04,0.041,0.0],
        ], [0.0;3], "skin", &forearm);
        shaped(s, prefix, &format!("hand-{i}"), &[
            [-0.36,0.025,0.014,0.0], [-0.30,0.034,0.019,0.0], [-0.26,0.025,0.022,0.0],
        ], [0.0;3], "skin", &forearm);
        for finger in 0..4 {
            part(s, prefix, &format!("finger-{i}-{finger}"), "capsule", &[0.0065,0.045,4.0,10.0],
                [-0.021+finger as f64*0.014,-0.38,0.002], "skin", &forearm);
        }
        part(s, prefix, &format!("thumb-{i}"), "capsule", &[0.008,0.035,4.0,10.0],
            [side*0.036,-0.32,0.008], "skin", &forearm);
        s.last()["rotation"] = json!([0.0,0.0,side*0.5]);
    }
    for i in 0..3 {
        ellipsoid(s, prefix, &format!("shirt-button-{i}"), [0.0,0.29+i as f64*0.075,0.126],
            [0.004,0.004,0.003], "white", "spine");
    }
    if selected {
        part(s, prefix, "plumb", "octahedron", &[0.14], [0.0,2.22,0.0], "eyes", "person");
        s.last()["material_ref"] = serde_json::Value::Null;
        s.last()["material"] = json!({"type":"standard","color":0x6bd870,"emissive":0x163b18});
        s.last()["scale"] = json!([1.0,1.6,1.0]);
    }
    let actor = prefix.strip_prefix("npc-").and_then(|p| p.trim_end_matches('/').parse::<u64>().ok()).unwrap_or(1);
    for node in &mut s.objects[start..] { node["actor"] = json!(actor); }
}
