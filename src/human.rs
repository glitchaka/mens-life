//! Stylised articulated people. The silhouette follows the supplied character
//! reference: oversized rounded head, compact torso, narrow shoulders/waist,
//! stronger hips, long tapered limbs and graphic hair/face masses.
use crate::scene::{Avatar, SceneSpec};
use serde_json::json;

fn part(
    s: &mut SceneSpec,
    prefix: &str,
    id: &str,
    kind: &str,
    args: &[f64],
    position: [f64; 3],
    material: &str,
    parent: &str,
) {
    s.person_mesh(
        &format!("{prefix}{id}"),
        kind,
        args,
        position,
        &format!("{prefix}{material}"),
        &format!("{prefix}{parent}"),
    );
}

fn group(s: &mut SceneSpec, prefix: &str, id: &str, p: [f64; 3], parent: &str) {
    s.group(
        &format!("{prefix}{id}"),
        p,
        Some(&format!("{prefix}{parent}")),
    );
}

fn shaped(
    s: &mut SceneSpec,
    prefix: &str,
    id: &str,
    rows: &[[f64; 4]],
    p: [f64; 3],
    material: &str,
    parent: &str,
) {
    let mut args = vec![28.0];
    args.extend(rows.iter().flatten().copied());
    part(s, prefix, id, "loft", &args, p, material, parent);
}

fn ellipsoid(
    s: &mut SceneSpec,
    prefix: &str,
    id: &str,
    p: [f64; 3],
    scale: [f64; 3],
    material: &str,
    parent: &str,
) {
    part(s, prefix, id, "sphere", &[1.0, 28.0, 18.0], p, material, parent);
    s.last()["scale"] = json!(scale);
}

fn material_set(s: &mut SceneSpec, avatar: &Avatar, prefix: &str) {
    for (name, color, roughness) in [
        ("skin", avatar.skin.as_str(), 0.78),
        ("hair", avatar.hair.as_str(), 0.94),
        ("shirt", avatar.shirt.as_str(), 0.88),
        ("pants", avatar.pants.as_str(), 0.94),
        ("shoes", "#343033", 0.92),
        ("white", "#f6f1e8", 0.72),
        ("eyes", "#20272a", 0.64),
        ("iris", "#486b69", 0.58),
        ("lips", "#995e62", 0.80),
    ] {
        s.materials[format!("{prefix}{name}")] =
            json!({"type":"standard","color":color,"roughness":roughness});
    }
}

pub fn append(
    s: &mut SceneSpec,
    avatar: &Avatar,
    prefix: &str,
    position: [f64; 3],
    selected: bool,
) {
    let start = s.objects.len();
    material_set(s, avatar, prefix);

    let width = match avatar.body.as_str() {
        "atlético" => 1.10,
        "delgado" => 0.91,
        _ => 1.0,
    };
    let hip = match avatar.body.as_str() {
        "atlético" => 1.04,
        "delgado" => 0.94,
        _ => 1.0,
    };
    let is_mara = avatar.name.eq_ignore_ascii_case("Mara");
    let is_nicolas = avatar.name.eq_ignore_ascii_case("Nicolás")
        || avatar.name.eq_ignore_ascii_case("Nicolas");

    s.group(&format!("{prefix}person"), position, None);
    group(s, prefix, "spine", [0.0, 0.94, 0.0], "person");

    shaped(
        s,
        prefix,
        "hips",
        &[
            [-0.12, 0.19 * hip, 0.135, 0.0],
            [-0.05, 0.245 * hip, 0.165, -0.006],
            [0.05, 0.25 * hip, 0.17, -0.004],
            [0.13, 0.205 * hip, 0.145, 0.0],
        ],
        [0.0, 0.94, 0.0],
        "pants",
        "person",
    );

    shaped(
        s,
        prefix,
        "torso",
        &[
            [0.00, 0.175 * width, 0.12, 0.0],
            [0.11, 0.165 * width, 0.115, 0.0],
            [0.26, 0.185 * width, 0.13, 0.008],
            [0.42, 0.225 * width, 0.145, 0.010],
            [0.54, 0.235 * width, 0.135, 0.0],
            [0.60, 0.15 * width, 0.095, -0.004],
        ],
        [0.0; 3],
        "shirt",
        "spine",
    );
    shaped(
        s,
        prefix,
        "neck",
        &[
            [0.56, 0.066, 0.060, 0.0],
            [0.66, 0.062, 0.058, 0.0],
        ],
        [0.0; 3],
        "skin",
        "spine",
    );

    group(s, prefix, "head-bone", [0.0, 0.65, 0.0], "spine");
    ellipsoid(
        s,
        prefix,
        "head",
        [0.0, 0.235, 0.0],
        [0.285, 0.315, 0.255],
        "skin",
        "head-bone",
    );

    for (i, side) in [-1.0, 1.0].into_iter().enumerate() {
        ellipsoid(s, prefix, &format!("eye-white-{i}"), [side * 0.097, 0.275, 0.238], [0.067, 0.044, 0.016], "white", "head-bone");
        ellipsoid(s, prefix, &format!("eye-{i}"), [side * 0.097, 0.274, 0.253], [0.029, 0.033, 0.012], "iris", "head-bone");
        ellipsoid(s, prefix, &format!("pupil-{i}"), [side * 0.097, 0.274, 0.264], [0.012, 0.016, 0.006], "eyes", "head-bone");
        ellipsoid(s, prefix, &format!("brow-{i}"), [side * 0.10, 0.335, 0.238], [0.072, 0.011, 0.011], "hair", "head-bone");
    }
    ellipsoid(s, prefix, "nose", [0.0, 0.205, 0.255], [0.026, 0.030, 0.025], "skin", "head-bone");
    ellipsoid(s, prefix, "mouth", [0.0, 0.132, 0.244], [0.064, 0.010, 0.009], "lips", "head-bone");

    shaped(
        s,
        prefix,
        "hair",
        &[
            [0.28, 0.30, 0.255, -0.035],
            [0.40, 0.29, 0.25, -0.045],
            [0.50, 0.25, 0.22, -0.055],
            [0.56, 0.13, 0.13, -0.055],
            [0.58, 0.02, 0.02, -0.055],
        ],
        [0.0; 3],
        "hair",
        "head-bone",
    );
    ellipsoid(
        s,
        prefix,
        "fringe",
        [-0.045, 0.405, 0.205],
        [if is_nicolas { 0.20 } else { 0.25 }, 0.055, 0.055],
        "hair",
        "head-bone",
    );
    s.last()["rotation"] = json!([0.0, 0.0, 0.16]);
    if !is_nicolas {
        for (i, side) in [-1.0, 1.0].into_iter().enumerate() {
            ellipsoid(
                s,
                prefix,
                &format!("sideburn-{i}"),
                [side * 0.245, if is_mara { 0.18 } else { 0.25 }, -0.035],
                [0.055, if is_mara { 0.25 } else { 0.15 }, 0.075],
                "hair",
                "head-bone",
            );
        }
    }

    for (i, side) in [-1.0, 1.0].into_iter().enumerate() {
        let leg = format!("leg-{i}");
        let knee = format!("knee-{i}");
        group(s, prefix, &leg, [side * 0.14 * hip, 0.94, 0.0], "person");
        shaped(
            s,
            prefix,
            &format!("thigh-{i}"),
            &[
                [-0.43, 0.058, 0.065, 0.0],
                [-0.30, 0.082, 0.09, -0.006],
                [-0.12, 0.105 * hip, 0.112, -0.010],
                [0.02, 0.11 * hip, 0.115, 0.0],
            ],
            [0.0; 3],
            "pants",
            &leg,
        );
        group(s, prefix, &knee, [0.0, -0.43, 0.0], &leg);
        shaped(
            s,
            prefix,
            &format!("calf-{i}"),
            &[
                [-0.39, 0.040, 0.046, 0.0],
                [-0.27, 0.052, 0.058, -0.004],
                [-0.12, 0.070, 0.074, -0.008],
                [0.02, 0.062, 0.066, 0.0],
            ],
            [0.0; 3],
            "skin",
            &knee,
        );
        part(
            s,
            prefix,
            &format!("shoe-{i}"),
            "rounded-box",
            &[0.14, 0.085, 0.29, 3.0, 0.028],
            [0.0, -0.435, 0.065],
            "shoes",
            &knee,
        );

        let arm = format!("arm-{i}");
        let forearm = format!("forearm-{i}");
        group(s, prefix, &arm, [side * 0.235 * width, 0.50, 0.0], "spine");
        shaped(
            s,
            prefix,
            &format!("upper-arm-{i}"),
            &[
                [-0.31, 0.048, 0.052, 0.0],
                [-0.18, 0.058, 0.062, 0.0],
                [0.02, 0.070, 0.072, 0.0],
            ],
            [0.0; 3],
            "shirt",
            &arm,
        );
        group(s, prefix, &forearm, [0.0, -0.31, 0.0], &arm);
        shaped(
            s,
            prefix,
            &format!("forearm-mesh-{i}"),
            &[
                [-0.29, 0.032, 0.035, 0.0],
                [-0.16, 0.043, 0.047, 0.0],
                [0.02, 0.052, 0.056, 0.0],
            ],
            [0.0; 3],
            "skin",
            &forearm,
        );
        ellipsoid(
            s,
            prefix,
            &format!("hand-{i}"),
            [0.0, -0.33, 0.0],
            [0.045, 0.070, 0.030],
            "skin",
            &forearm,
        );
    }

    if selected {
        part(
            s,
            prefix,
            "plumb",
            "octahedron",
            &[0.14],
            [0.0, 2.38, 0.0],
            "eyes",
            "person",
        );
        s.last()["material_ref"] = serde_json::Value::Null;
        s.last()["material"] =
            json!({"type":"standard","color":0x6bd870,"emissive":0x163b18});
        s.last()["scale"] = json!([1.0, 1.6, 1.0]);
    }

    let actor = prefix
        .strip_prefix("npc-")
        .and_then(|p| p.trim_end_matches('/').parse::<u64>().ok())
        .unwrap_or(1);
    for node in &mut s.objects[start..] {
        node["actor"] = json!(actor);
    }
}
