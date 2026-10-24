//! The house, furniture and articulated avatars, expressed as native mesh data.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::f64::consts::PI;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Avatar {
    pub name: String,
    pub body: String,
    pub skin: String,
    pub hair: String,
    pub shirt: String,
    pub pants: String,
}
impl Default for Avatar {
    fn default() -> Self {
        Self {
            name: "Alex".into(),
            body: "adulto".into(),
            skin: "#b97855".into(),
            hair: "#37271f".into(),
            shirt: "#315f79".into(),
            pants: "#27384b".into(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SceneSpec {
    pub background: u32,
    pub fog: [f64; 2],
    pub camera: Value,
    pub lights: Vec<Value>,
    pub materials: Value,
    pub objects: Vec<Value>,
    pub obstacles: Vec<String>,
    pub drops_y: Vec<f64>,
}
impl SceneSpec {
    #[allow(clippy::too_many_arguments)]
    fn mesh(
        &mut self,
        kind: &str,
        args: &[f64],
        p: [f64; 3],
        color: u32,
        roughness: f64,
        name: &str,
        shadows: [bool; 2],
    ) -> String {
        let id = format!("mesh-{}", self.objects.len());
        self.objects.push(json!({
            "id":id,"kind":kind,"args":args,"position":p,"name":name,
            "material":{"type":"standard","color":color,"roughness":roughness},
            "cast_shadow":shadows[0],"receive_shadow":shadows[1]
        }));
        id
    }
    fn box_mesh(&mut self, z: [f64; 3], p: [f64; 3], c: u32, n: &str) -> String {
        let radius = 0.1_f64.min(z[0].min(z[1]).min(z[2]) * 0.18);
        self.mesh("rounded-box", &[z[0], z[1], z[2], 3.0, radius], p, c, 0.68, n, [true, true])
    }
    fn cyl(&mut self, r: f64, h: f64, p: [f64; 3], c: u32, n: &str) -> String {
        self.mesh("cylinder", &[r, r, h, 24.0], p, c, 0.65, n, [true, false])
    }
    pub(crate) fn last(&mut self) -> &mut Value { self.objects.last_mut().unwrap() }
    pub(crate) fn group(&mut self, id: &str, p: [f64; 3], parent: Option<&str>) {
        self.objects.push(json!({"id":id,"kind":"group","position":p,"parent":parent}));
    }
    pub(crate) fn person_mesh(
        &mut self,
        id: &str,
        kind: &str,
        args: &[f64],
        p: [f64; 3],
        material: &str,
        parent: &str,
    ) {
        self.objects.push(json!({
            "id":id,"kind":kind,"args":args,"position":p,
            "material_ref":material,"parent":parent,"cast_shadow":true
        }));
    }
}

fn wall_x(s: &mut SceneSpec, width: f64, x: f64, z: f64) {
    s.box_mesh([width, 3.05, 0.16], [x, 1.47, z], 0xeee4d4, "");
}
fn wall_z(s: &mut SceneSpec, depth: f64, x: f64, z: f64) {
    s.box_mesh([0.16, 3.05, depth], [x, 1.47, z], 0xeee4d4, "");
}
fn trim_x(s: &mut SceneSpec, width: f64, x: f64, z: f64) {
    s.box_mesh([width, 0.13, 0.08], [x, 0.08, z + 0.09], 0x765b48, "");
}
fn trim_z(s: &mut SceneSpec, depth: f64, x: f64, z: f64) {
    s.box_mesh([0.08, 0.13, depth], [x + 0.09, 0.08, z], 0x765b48, "");
}
fn flat(s: &mut SceneSpec, size: [f64; 2], at: [f64; 2], color: u32) {
    s.box_mesh([size[0], 0.055, size[1]], [at[0], 0.015, at[1]], color, "");
}
fn plant(s: &mut SceneSpec, x: f64, z: f64, scale: f64) {
    s.cyl(0.34 * scale, 0.46 * scale, [x, 0.24 * scale, z], 0x8b674f, "");
    for (i, (dx, dz)) in [(-0.18,0.03),(0.13,-0.08),(0.02,0.16),(-0.04,-0.18),(0.18,0.13)].into_iter().enumerate() {
        s.mesh("sphere", &[0.24 * scale, 12.0, 8.0],
            [x + dx * scale, 0.82 * scale + (i as f64 * 0.07), z + dz * scale],
            0x56785d, 0.95, "", [false, false]);
        s.last()["scale"] = json!([0.78, 1.65, 0.55]);
        s.last()["rotation"] = json!([0.0, 0.0, (i as f64 - 2.0) * 0.22]);
    }
}
fn lamp(s: &mut SceneSpec, x: f64, z: f64) {
    s.cyl(0.08, 1.5, [x, 0.76, z], 0x554d47, "");
    s.cyl(0.34, 0.36, [x, 1.62, z], 0xe8d2a6, "");
}
fn picture(s: &mut SceneSpec, x: f64, y: f64, z: f64, width: f64, color: u32) {
    s.box_mesh([width + 0.14, 1.10, 0.08], [x, y, z], 0x654d3e, "");
    s.box_mesh([width, 0.92, 0.035], [x, y, z + 0.06], color, "");
}
fn simple_bed(s: &mut SceneSpec, x: f64, z: f64, interactive: bool) {
    let name = if interactive { "bed" } else { "" };
    s.box_mesh([2.70, 0.34, 3.35], [x, 0.34, z], 0x725b4c, name);
    s.box_mesh([2.52, 0.42, 3.12], [x, 0.68, z], 0xd9d0c3, name);
    s.box_mesh([2.54, 0.11, 2.05], [x, 0.92, z - 0.28], 0x9f776c, name);
    s.box_mesh([1.02, 0.18, 0.62], [x - 0.58, 0.98, z + 1.18], 0xf4eee3, name);
    s.box_mesh([1.02, 0.18, 0.62], [x + 0.58, 0.98, z + 1.18], 0xf4eee3, name);
    s.box_mesh([2.78, 1.05, 0.16], [x, 0.90, z + 1.72], 0x60483c, name);
}

pub fn build(avatar: &Avatar, random: &[f64]) -> SceneSpec {
    let mut s = SceneSpec {
        background: 0x9fc8cf,
        fog: [25.0, 44.0],
        camera: json!({
            "bounds":[-10,10,7,-7],"near":0.1,"far":100,
            "position":[15,17,18],"look_at":[0,0,0],
            "resize":{"min_vertical":8.5,"min_horizontal":11.5}
        }),
        lights: vec![
            json!({"kind":"hemisphere","sky":0xfff7e7,"ground":0x536a72,"intensity":2.5}),
            json!({"kind":"directional","color":0xffefd0,"intensity":3.3,"position":[-8,16,10],"cast_shadow":true,"shadow_size":[2048,2048]}),
        ],
        materials: json!({
            "skin":{"type":"standard","color":avatar.skin,"roughness":0.62},
            "shirt":{"type":"standard","color":avatar.shirt,"roughness":0.72},
            "pants":{"type":"standard","color":avatar.pants,"roughness":0.8},
            "hair":{"type":"standard","color":avatar.hair,"roughness":0.9},
            "eyes":{"type":"standard","color":0x1b2529},
            "shoes":{"type":"standard","color":0x3a332e},
        }),
        objects: vec![], obstacles: vec![], drops_y: vec![],
    };

    // Shell: same physical footprint, but now a complete furnished house instead of a compressed room.
    let floor = s.box_mesh([15.0, 0.34, 11.0], [0.0, -0.22, 0.0], 0xc9ad82, "");
    s.last()["id"] = json!("floor");
    for i in -7..=7 {
        s.box_mesh([0.028, 0.018, 10.6], [i as f64, -0.035, 0.0], 0xa88b68, "");
    }
    wall_z(&mut s, 11.0, -7.45, 0.0);
    wall_x(&mut s, 15.0, 0.0, -5.45);
    trim_z(&mut s, 10.8, -7.34, 0.0);
    trim_x(&mut s, 14.7, 0.0, -5.34);

    for x in [-4.9, 0.0, 4.9] {
        s.box_mesh([2.35, 1.45, 0.05], [x, 2.10, -5.30], 0x90b8c1, "");
        s.last()["material"] = json!({"type":"physical","color":0x9bc8d1,"transparent":true,"opacity":0.42,"roughness":0.08});
        s.box_mesh([2.55, 0.08, 0.08], [x, 1.36, -5.24], 0xf0e8db, "");
        s.box_mesh([0.08, 1.60, 0.08], [x, 2.10, -5.24], 0xf0e8db, "");
    }
    s.box_mesh([0.05, 1.45, 2.35], [-7.30, 2.10, 3.65], 0x90b8c1, "");
    s.last()["material"] = json!({"type":"physical","color":0x9bc8d1,"transparent":true,"opacity":0.42,"roughness":0.08});

    // Master bedroom.
    wall_z(&mut s, 4.10, -2.05, 3.30);
    wall_x(&mut s, 3.80, -5.40, 1.25);
    wall_x(&mut s, 0.48, -2.29, 1.25);
    trim_z(&mut s, 3.92, -1.95, 3.30);
    trim_x(&mut s, 3.62, -5.40, 1.36);
    flat(&mut s, [4.45, 3.55], [-4.75, 3.30], 0xb4a68d);
    simple_bed(&mut s, -4.60, 2.82, true);
    for x in [-6.35, -2.92] {
        s.box_mesh([0.72, 0.70, 0.72], [x, 0.36, 3.92], 0x765948, "");
        s.cyl(0.18, 0.42, [x, 1.02, 3.92], 0xd3b879, "");
    }
    s.box_mesh([1.55, 2.35, 0.72], [-6.35, 1.18, 5.00], 0x806653, "");
    s.box_mesh([1.18, 2.08, 0.05], [-6.35, 1.20, 4.60], 0x9d8065, "");
    s.box_mesh([1.15, 0.45, 0.58], [-2.78, 0.33, 4.62], 0xb78d70, "");

    // Second bedroom / study. Its doorway and lower wall leave the initial player spawn clear.
    wall_z(&mut s, 2.80, 1.28, 4.00);
    wall_x(&mut s, 2.02, -0.65, 2.55);
    wall_x(&mut s, 0.52, 1.02, 2.55);
    flat(&mut s, [2.85, 2.55], [-0.30, 4.02], 0xb8b1a2);
    simple_bed(&mut s, -0.18, 4.10, false);
    s.box_mesh([1.42, 0.13, 0.62], [-1.15, 0.76, 2.96], 0x8f674b, "");
    for x in [-1.72, -0.58] { s.cyl(0.07, 0.76, [x, 0.39, 2.96], 0x5d4a3c, ""); }
    s.box_mesh([0.65, 0.72, 0.65], [-1.12, 0.37, 3.62], 0x71847a, "");

    // Full bathroom.
    flat(&mut s, [5.10, 3.08], [-4.70, -3.72], 0xbecfd0);
    wall_z(&mut s, 3.12, -2.10, -3.80);
    wall_x(&mut s, 1.00, -6.82, -2.22);
    wall_x(&mut s, 3.20, -3.70, -2.22);
    s.box_mesh([2.25, 0.18, 1.72], [-5.78, 0.10, -4.30], 0xf4f6f2, "shower");
    s.mesh("box", &[0.06, 2.48, 1.86], [-5.00, 1.29, -4.28], 0xbfe0e5, 0.1, "shower", [false, false]);
    s.last()["id"] = json!("glass");
    s.last()["material"] = json!({"type":"physical","color":0xbfe0e5,"transparent":true,"opacity":0.34,"roughness":0.08});
    s.cyl(0.07, 2.10, [-6.52, 1.14, -4.82], 0xb4b8b5, "shower");
    s.cyl(0.32, 0.08, [-6.22, 2.17, -4.82], 0xb4b8b5, "shower");
    s.last()["rotation"] = json!([0.0, 0.0, PI / 2.0]);
    s.box_mesh([0.62, 0.38, 0.72], [-3.65, 0.28, -4.28], 0xf4f2e9, "toilet");
    s.cyl(0.36, 0.16, [-3.65, 0.45, -4.17], 0xf4f2e9, "toilet");
    s.box_mesh([0.72, 0.65, 0.25], [-3.65, 0.75, -4.82], 0xf4f2e9, "toilet");
    s.box_mesh([1.10, 0.72, 0.62], [-2.75, 0.38, -3.05], 0x8c755f, "");
    s.box_mesh([1.14, 0.14, 0.66], [-2.75, 0.78, -3.05], 0xf2eee5, "");
    s.cyl(0.25, 0.12, [-2.75, 0.88, -3.05], 0xf8f7f1, "");
    s.box_mesh([0.92, 1.18, 0.06], [-2.15, 1.67, -3.12], 0x8fb5ba, "");

    // Living room.
    flat(&mut s, [5.25, 3.75], [3.55, 2.87], 0x718987);
    s.box_mesh([4.25, 0.38, 1.38], [3.35, 0.38, 3.42], 0x9cae9f, "");
    s.box_mesh([4.00, 0.72, 0.30], [3.35, 0.80, 4.00], 0x87998b, "");
    s.box_mesh([1.18, 0.34, 1.36], [5.58, 0.42, 3.42], 0x9cae9f, "");
    s.box_mesh([1.65, 0.22, 0.85], [5.40, 0.28, 2.25], 0x8a674e, "");
    s.box_mesh([1.35, 0.06, 0.62], [5.40, 0.41, 2.25], 0xb89672, "");
    s.box_mesh([3.72, 0.50, 0.48], [2.60, 0.35, 0.66], 0x6c5548, "tv");
    s.box_mesh([3.35, 2.08, 0.22], [2.60, 1.65, 0.54], 0x292b2e, "tv");
    s.box_mesh([2.92, 1.62, 0.035], [2.60, 1.65, 0.40], 0x17212b, "tv");
    s.last()["id"] = json!("tv-screen");
    s.last()["name"] = json!("tv");
    lamp(&mut s, 6.35, 4.45);
    plant(&mut s, 6.48, 1.08, 0.95);

    // Kitchen and island. The physical Eat approach remains free in front of it.
    flat(&mut s, [7.05, 3.18], [3.72, -3.55], 0xc9c5b9);
    s.box_mesh([5.90, 0.88, 0.76], [3.80, 0.45, -4.68], 0x839287, "");
    s.box_mesh([5.96, 0.11, 0.84], [3.80, 0.95, -4.68], 0xeee9df, "");
    for x in [1.35, 2.62, 3.89, 5.16] {
        s.box_mesh([0.035, 0.66, 0.04], [x, 0.47, -4.28], 0x65736b, "");
    }
    s.box_mesh([1.48, 2.45, 0.92], [6.38, 1.22, -4.35], 0xe5e6df, "");
    s.box_mesh([1.10, 1.62, 0.05], [6.38, 1.56, -3.86], 0xbccfd0, "");
    s.box_mesh([2.90, 0.82, 0.92], [3.05, 0.43, -2.16], 0x73857b, "sink");
    s.box_mesh([2.98, 0.10, 1.00], [3.05, 0.89, -2.16], 0xf0ede5, "sink");
    s.cyl(0.29, 0.10, [2.46, 0.96, -2.16], 0xaeb7b5, "sink");
    s.cyl(0.045, 0.42, [2.62, 1.20, -2.34], 0x9ca5a3, "sink");
    s.box_mesh([0.54, 0.10, 0.36], [3.76, 0.99, -2.16], 0x292e2f, "");
    for x in [2.05, 4.05] {
        s.box_mesh([0.52, 0.12, 0.52], [x, 0.52, -1.18], 0xb78665, "");
        s.cyl(0.075, 0.82, [x, 0.09, -1.18], 0x624b3c, "");
    }

    // Dining and entry.
    s.box_mesh([3.28, 0.18, 1.48], [5.00, 0.78, -0.18], 0xa27856, "");
    for (x, z) in [(3.72,-0.18),(6.28,-0.18),(5.00,-1.16),(5.00,0.80)] {
        s.box_mesh([0.72, 0.14, 0.72], [x, 0.52, z], 0xc08b68, "");
        s.cyl(0.08, 0.88, [x, 0.07, z], 0x684d3a, "");
    }
    s.box_mesh([1.42, 0.48, 0.44], [-0.82, 0.32, -0.10], 0x806653, "");
    s.box_mesh([1.22, 1.10, 0.06], [-0.82, 1.22, -0.34], 0x9eb7b5, "");
    plant(&mut s, -1.35, -1.20, 0.75);

    // Decorative lighting and artwork.
    s.cyl(0.50, 0.28, [0.15, 2.92, 0.15], 0xe4c680, "");
    s.cyl(0.04, 2.55, [0.15, 1.47, 0.15], 0x554d45, "");
    picture(&mut s, 4.90, 2.25, -5.27, 1.42, 0x8da29b);

    s.lights.push(json!({
        "kind":"point","color":0xffd89b,"intensity":18,"distance":8,
        "position":[0.15,2.72,0.15],"after_objects":s.objects.len()
    }));

    for node in &s.objects {
        let id = node["id"].as_str().unwrap();
        let y = node["position"][1].as_f64().unwrap();
        if id != floor && id != "floor" && id != "shower-head" && y > 0.1 && y < 2.0 {
            s.obstacles.push(id.into());
        }
    }

    crate::human::append(&mut s, avatar, "", [0.0, 0.0, 2.0], true);
    for (id, resident, position) in residents() {
        crate::human::append(&mut s, &resident, &format!("npc-{id}/"), position, false);
    }

    s.group("drops", [0.0; 3], None);
    s.last()["visible"] = json!(false);
    for i in 0..30_usize {
        let sample = |j: usize| random.get(i * 3 + j).copied().unwrap_or(0.5);
        let y = sample(1) * 2.4;
        s.drops_y.push(y);
        s.mesh("sphere", &[0.025, 6.0, 4.0],
            [-6.3 + sample(0) * 0.7, y, -4.7 + sample(2) * 0.7],
            0x98d6f6, 1.0, "", [false, false]);
        s.last()["id"] = json!(format!("drop-{i}"));
        s.last()["parent"] = json!("drops");
        s.last()["scale"] = json!([1, 4, 1]);
        s.last()["material"] = json!({"type":"basic","color":0x98d6f6,"transparent":true,"opacity":0.65});
    }
    s
}

/// Stable IDs used by rendering, physical interaction and MSPA relationship targets.
pub fn residents() -> Vec<(u64, Avatar, [f64;3])> {
    vec![
        (2, Avatar { name: "Mara".into(), body: "delgado".into(), skin: "#b78467".into(),
            hair: "#382b24".into(), shirt: "#8a5360".into(), pants: "#484c4b".into() }, [-0.8,0.0,0.0]),
        (3, Avatar { name: "Nicolás".into(), body: "adulto".into(), skin: "#d4a181".into(),
            hair: "#342b23".into(), shirt: "#64785d".into(), pants: "#494d5b".into() }, [4.0,0.0,1.9]),
    ]
}
