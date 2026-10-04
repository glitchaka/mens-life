//! The room, furniture and articulated avatar, expressed as native mesh data.
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
    // This descriptor mirrors geometry, transform, material and shadow options.
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
        self.objects.push(json!({"id":id,"kind":kind,"args":args,"position":p,"name":name,"material":{"type":"standard","color":color,"roughness":roughness},"cast_shadow":shadows[0],"receive_shadow":shadows[1]}));
        id
    }
    fn box_mesh(&mut self, z: [f64; 3], p: [f64; 3], c: u32, n: &str) -> String {
        let radius = 0.1_f64.min(z[0].min(z[1]).min(z[2]) * 0.18);
        self.mesh(
            "rounded-box",
            &[z[0], z[1], z[2], 3.0, radius],
            p,
            c,
            0.68,
            n,
            [true, true],
        )
    }
    fn cyl(&mut self, r: f64, h: f64, p: [f64; 3], c: u32, n: &str) -> String {
        self.mesh("cylinder", &[r, r, h, 24.0], p, c, 0.65, n, [true, false])
    }
    fn last(&mut self) -> &mut Value {
        self.objects.last_mut().unwrap()
    }
    fn group(&mut self, id: &str, p: [f64; 3], parent: Option<&str>) {
        self.objects
            .push(json!({"id":id,"kind":"group","position":p,"parent":parent}));
    }
    fn person_mesh(
        &mut self,
        id: &str,
        kind: &str,
        args: &[f64],
        p: [f64; 3],
        material: &str,
        parent: &str,
    ) {
        self.objects.push(json!({"id":id,"kind":kind,"args":args,"position":p,"material_ref":material,"parent":parent,"cast_shadow":true}));
    }
}

pub fn build(avatar: &Avatar, random: &[f64]) -> SceneSpec {
    let mut s = SceneSpec {
        background: 0x9ec7d0,
        fog: [25.0, 44.0],
        camera: json!({"bounds":[-10,10,7,-7],"near":0.1,"far":100,"position":[15,17,18],"look_at":[0,0,0],"resize":{"min_vertical":8.5,"min_horizontal":11.5}}),
        lights: vec![
            json!({"kind":"hemisphere","sky":0xfff6df,"ground":0x4d6972,"intensity":2.5}),
            json!({"kind":"directional","color":0xfff0cf,"intensity":3.3,"position":[-8,16,10],"cast_shadow":true,"shadow_size":[2048,2048]}),
        ],
        materials: json!({
            "skin":{"type":"standard","color":avatar.skin,"roughness":0.62},
            "shirt":{"type":"standard","color":avatar.shirt,"roughness":0.72},
            "pants":{"type":"standard","color":avatar.pants,"roughness":0.8},
            "hair":{"type":"standard","color":avatar.hair,"roughness":0.9},
            "eyes":{"type":"standard","color":0x1b2529},
            "shoes":{"type":"standard","color":0x3a332e},
        }),
        objects: vec![],
        obstacles: vec![],
        drops_y: vec![],
    };
    let floor = s.box_mesh([15.0, 0.35, 11.0], [0.0, -0.22, 0.0], 0xd6bd92, "");
    s.last()["id"] = json!("floor");
    for i in -7..=7 {
        s.box_mesh([0.035, 0.02, 10.5], [i as f64, -0.035, 0.0], 0xb59b76, "");
    }
    s.box_mesh([0.25, 4.5, 11.0], [-7.45, 2.1, 0.0], 0xf1dfc4, "");
    s.box_mesh([15.0, 4.5, 0.25], [0.0, 2.1, -5.45], 0xead7bc, "");
    s.box_mesh([5.2, 0.06, 3.4], [2.0, 0.04, 1.1], 0x557d7b, "");
    s.box_mesh([4.4, 0.75, 1.35], [3.5, 0.55, 3.2], 0xc89066, "");
    s.box_mesh([4.4, 0.8, 0.32], [3.5, 1.15, 3.72], 0xb67854, "");
    s.box_mesh([2.5, 0.22, 1.25], [2.0, 0.35, 0.55], 0x9b6c43, "");
    for x in [1.05, 2.95] {
        s.box_mesh([0.18, 0.6, 0.18], [x, 0.12, 0.15], 0x604430, "");
    }
    s.box_mesh([3.7, 0.45, 1.05], [2.0, 0.65, -3.92], 0x73563e, "");
    s.box_mesh([3.25, 2.05, 0.28], [2.0, 1.9, -4.18], 0x24272d, "tv");
    s.box_mesh([2.75, 1.5, 0.03], [2.0, 1.9, -4.02], 0x315d71, "tv");
    for (z, p, c) in [
        ([5.1, 0.55, 3.25], [-4.25, 0.4, 2.7], 0x815942),
        ([4.75, 0.65, 2.85], [-4.25, 0.82, 2.7], 0xdbc8a5),
        ([4.75, 0.22, 1.65], [-4.25, 1.15, 3.3], 0xb86554),
        ([1.6, 0.3, 0.78], [-4.25, 1.36, 3.72], 0xf2e7d3),
    ] {
        s.box_mesh(z, p, c, "bed");
    }
    s.box_mesh([1.3, 1.7, 1.2], [-6.25, 0.85, 0.6], 0x73513c, "");
    s.box_mesh([1.3, 0.08, 1.2], [-6.25, 1.74, 0.6], 0xd6b873, "");
    s.box_mesh([5.2, 0.08, 3.35], [-4.65, 0.02, -3.45], 0xbad0cf, "");
    s.box_mesh([0.18, 3.5, 3.6], [-2.12, 1.7, -3.62], 0xe4ece8, "");
    s.box_mesh([1.5, 1.05, 0.18], [-6.5, 0.5, -1.7], 0xe4ece8, "");
    s.box_mesh([2.2, 1.05, 0.18], [-3.22, 0.5, -1.7], 0xe4ece8, "");
    s.box_mesh([2.15, 0.2, 1.7], [-5.75, 0.12, -4.25], 0xf4f6f2, "shower");
    s.mesh(
        "box",
        &[0.06, 2.7, 1.8],
        [-4.68, 1.4, -4.2],
        0xbfe0e5,
        0.1,
        "shower",
        [false, false],
    );
    s.last()["id"] = json!("glass");
    s.last()["material"] = json!({"type":"physical","color":0xbfe0e5,"transparent":true,"opacity":0.35,"roughness":0.1});
    s.cyl(0.09, 2.3, [-6.65, 1.25, -4.9], 0xb4b8b5, "shower");
    s.cyl(0.38, 0.08, [-6.3, 2.35, -4.9], 0xb4b8b5, "shower");
    s.last()["id"] = json!("shower-head");
    s.last()["rotation"] = json!([0, 0, PI / 2.0]);
    s.box_mesh([1.05, 0.48, 1.15], [-3.25, 0.35, -4.35], 0xf4f2e9, "toilet");
    s.cyl(0.53, 0.48, [-3.25, 0.65, -4.12], 0xf4f2e9, "toilet");
    s.box_mesh([1.2, 1.05, 0.35], [-3.25, 1.05, -5.02], 0xf4f2e9, "toilet");
    s.box_mesh([1.35, 0.25, 0.85], [-3.15, 1.05, -2.35], 0xf5f2e9, "sink");
    s.cyl(0.12, 0.95, [-3.15, 0.52, -2.35], 0xb9b5aa, "");
    s.box_mesh([1.5, 1.3, 0.08], [-3.15, 2.05, -1.82], 0x91b8bd, "");
    s.box_mesh([2.3, 1.9, 0.18], [-2.7, 2.5, -5.27], 0x6f5241, "");
    s.box_mesh([1.9, 1.5, 0.03], [-2.7, 2.5, -5.38], 0xe4b663, "");
    s.cyl(0.48, 0.55, [6.3, 0.3, -4.25], 0x9d6c4e, "");
    for i in 0..7 {
        let i = i as f64;
        s.mesh(
            "sphere",
            &[0.22, 12.0, 8.0],
            [
                6.3 + i.sin() * 0.45,
                1.0 + (i * 0.8).cos() * 0.35,
                -4.25 + i.cos() * 0.4,
            ],
            0x4f8056,
            1.0,
            "",
            [false, false],
        );
        // MeshStandardMaterial defaults to roughness=1, as in the reference.
        s.last()["scale"] = json!([0.7, 2.0, 0.45]);
        s.last()["rotation"] = json!([0, 0, i.sin() * 0.7]);
    }
    s.box_mesh([3.8, 0.9, 0.72], [4.9, 0.46, -2.05], 0xb6c5b7, "");
    s.box_mesh([1.45, 2.85, 0.82], [6.35, 1.45, -2.08], 0xe8e7dc, "");
    s.box_mesh([1.15, 1.65, 0.05], [6.35, 1.75, -1.65], 0xc4d5d7, "");
    s.box_mesh([1.8, 0.92, 0.75], [3.05, 0.47, -2.05], 0x8fa78f, "");
    s.cyl(0.32, 0.08, [3.05, 0.97, -2.15], 0x31383b, "");
    s.box_mesh([0.7, 0.12, 0.5], [5.05, 0.98, -2.1], 0x30383a, "");
    for x in [3.8, 4.9, 5.8] {
        s.box_mesh([0.025, 0.5, 0.03], [x, 0.48, -1.66], 0x6f8075, "");
    }
    s.cyl(1.05, 0.18, [5.15, 0.72, 0.25], 0xa77a54, "");
    s.cyl(0.16, 1.25, [5.15, 0.16, 0.25], 0x604a37, "");
    for (x, z) in [(4.1, 0.25), (6.2, 0.25), (5.15, -0.82), (5.15, 1.32)] {
        s.box_mesh([0.72, 0.15, 0.72], [x, 0.48, z], 0xc38c62, "");
        s.cyl(0.1, 0.88, [x, 0.04, z], 0x6e4c36, "");
    }
    s.box_mesh([3.1, 1.65, 0.1], [3.6, 2.8, -5.27], 0xf4ead8, "");
    s.box_mesh([2.55, 1.18, 0.035], [3.6, 2.8, -5.20], 0x86b7c2, "");
    s.box_mesh([0.08, 1.18, 0.05], [3.6, 2.8, -5.16], 0xe9f1ef, "");
    s.box_mesh([2.55, 0.07, 0.05], [3.6, 2.8, -5.16], 0xe9f1ef, "");
    s.cyl(0.5, 0.35, [0.2, 2.9, 0.1], 0xe5c981, "");
    s.cyl(0.04, 2.6, [0.2, 1.45, 0.1], 0x544d45, "");
    s.lights.push(json!({"kind":"point","color":0xffd89b,"intensity":18,"distance":8,"position":[0.2,2.7,0.1],"after_objects":s.objects.len()}));

    // Select the same candidates at the same construction stage as the original.
    for node in &s.objects {
        let id = node["id"].as_str().unwrap();
        let y = node["position"][1].as_f64().unwrap();
        let name = node["name"].as_str().unwrap_or("");
        if id != floor
            && id != "floor"
            && id != "glass"
            && id != "shower-head"
            && y > 0.1
            && y < 2.0
            && name != "bed"
            && name != "toilet"
        {
            s.obstacles.push(id.into());
        }
    }

    s.group("person", [0.0, 0.0, 2.0], None);
    s.person_mesh(
        "torso",
        "capsule",
        &[
            if avatar.body == "atlético" {
                0.44
            } else {
                0.4
            },
            0.72,
            8.0,
            18.0,
        ],
        [0.0, 1.38, 0.0],
        "shirt",
        "person",
    );
    s.last()["scale"] = json!([1, 1, if avatar.body == "delgado" { 0.78 } else { 0.92 }]);
    s.person_mesh(
        "neck",
        "cylinder",
        &[0.16, 0.18, 0.22, 18.0],
        [0.0, 1.92, 0.0],
        "skin",
        "person",
    );
    s.person_mesh(
        "head",
        "sphere",
        &[0.36, 32.0, 24.0],
        [0.0, 2.25, 0.0],
        "skin",
        "person",
    );
    s.last()["scale"] = json!([0.92, 1.12, 0.94]);
    s.person_mesh(
        "nose",
        "cone",
        &[0.065, 0.16, 16.0],
        [0.0, 2.25, 0.36],
        "skin",
        "person",
    );
    s.last()["rotation"] = json!([PI / 2.0, 0, 0]);
    for (i, x) in [-0.13, 0.13].iter().enumerate() {
        s.person_mesh(
            &format!("eye-{i}"),
            "sphere",
            &[0.032, 12.0, 8.0],
            [*x, 2.32, 0.337],
            "eyes",
            "person",
        );
    }
    s.person_mesh(
        "hair",
        "sphere",
        &[0.375, 28.0, 16.0, 0.0, PI * 2.0, 0.0, PI * 0.52],
        [0.0, 2.32, 0.0],
        "hair",
        "person",
    );
    s.last()["scale"] = json!([0.94, 1.1, 0.96]);
    s.person_mesh(
        "hips",
        "box",
        &[0.62, 0.32, 0.36],
        [0.0, 0.92, 0.0],
        "pants",
        "person",
    );
    for (i, x) in [-0.2, 0.2].iter().enumerate() {
        let id = format!("leg-{i}");
        s.group(&id, [*x, 0.82, 0.0], Some("person"));
        s.person_mesh(
            &format!("upper-{i}"),
            "capsule",
            &[0.105, 0.48, 6.0, 12.0],
            [0.0, -0.28, 0.0],
            "pants",
            &id,
        );
        s.person_mesh(
            &format!("shoe-{i}"),
            "box",
            &[0.24, 0.17, 0.42],
            [0.0, -0.68, 0.1],
            "shoes",
            &id,
        );
    }
    for (i, x) in [-0.55, 0.55].iter().enumerate() {
        let id = format!("arm-{i}");
        s.group(&id, [*x, 1.66, 0.0], Some("person"));
        s.person_mesh(
            &format!("limb-{i}"),
            "capsule",
            &[0.09, 0.56, 6.0, 12.0],
            [0.0, -0.3, 0.0],
            "shirt",
            &id,
        );
        s.person_mesh(
            &format!("hand-{i}"),
            "sphere",
            &[0.105, 16.0, 12.0],
            [0.0, -0.68, 0.0],
            "skin",
            &id,
        );
    }
    s.person_mesh(
        "plumb",
        "octahedron",
        &[0.28],
        [0.0, 3.25, 0.0],
        "eyes",
        "person",
    );
    s.last()["material_ref"] = Value::Null;
    s.last()["material"] = json!({"type":"standard","color":0x6bd870,"emissive":0x163b18});
    s.last()["scale"] = json!([1, 1.75, 1]);
    s.box_mesh([2.85, 1.65, 0.035], [2.0, 1.9, -4.01], 0x17212b, "tv");
    s.last()["id"] = json!("tv-screen");
    s.group("drops", [0.0; 3], None);
    s.last()["visible"] = json!(false);
    for i in 0..30_usize {
        let sample = |j: usize| random.get(i * 3 + j).copied().unwrap_or(0.5);
        let y = sample(1) * 2.4;
        s.drops_y.push(y);
        s.mesh(
            "sphere",
            &[0.025, 6.0, 4.0],
            [-6.3 + sample(0) * 0.7, y, -4.7 + sample(2) * 0.7],
            0x98d6f6,
            1.0,
            "",
            [false, false],
        );
        s.last()["id"] = json!(format!("drop-{i}"));
        s.last()["parent"] = json!("drops");
        s.last()["scale"] = json!([1, 4, 1]);
        s.last()["material"] =
            json!({"type":"basic","color":0x98d6f6,"transparent":true,"opacity":0.65});
    }
    s
}
