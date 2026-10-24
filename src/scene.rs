//! Scene composition. Architecture, furnishings, people and transient effects live in dedicated modules.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

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
    fn new(avatar: &Avatar) -> Self {
        Self {
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
            objects: vec![],
            obstacles: vec![],
            drops_y: vec![],
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn mesh(
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
    pub(crate) fn box_mesh(&mut self, z: [f64; 3], p: [f64; 3], c: u32, n: &str) -> String {
        let radius = 0.1_f64.min(z[0].min(z[1]).min(z[2]) * 0.18);
        self.mesh("rounded-box", &[z[0], z[1], z[2], 3.0, radius], p, c, 0.68, n, [true, true])
    }
    pub(crate) fn cyl(&mut self, r: f64, h: f64, p: [f64; 3], c: u32, n: &str) -> String {
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

fn append_shower_drops(s: &mut SceneSpec, random: &[f64]) {
    s.group("drops", [0.0; 3], None);
    s.last()["visible"] = json!(false);
    for i in 0..30_usize {
        let sample = |j: usize| random.get(i * 3 + j).copied().unwrap_or(0.5);
        let y = sample(1) * 2.15;
        s.drops_y.push(y);
        s.mesh(
            "sphere",
            &[0.025, 6.0, 4.0],
            [-6.14 + sample(0) * 0.48, y, -4.35 + sample(2) * 0.62],
            0x98d6f6,
            1.0,
            "",
            [false, false],
        );
        s.last()["id"] = json!(format!("drop-{i}"));
        s.last()["parent"] = json!("drops");
        s.last()["scale"] = json!([1, 4, 1]);
        s.last()["material"] = json!({
            "type":"basic","color":0x98d6f6,"transparent":true,"opacity":0.65
        });
    }
}

/// Composition root: the imported house, interior, people and effects are each
/// delegated to their own module rather than being authored here.
pub fn build(avatar: &Avatar, random: &[f64]) -> SceneSpec {
    let mut scene = SceneSpec::new(avatar);
    crate::house::append(&mut scene);
    crate::furnishings::append(&mut scene);

    crate::human::append(&mut scene, avatar, "", [0.0, 0.0, 2.0], true);
    for (id, resident, position) in residents() {
        crate::human::append(
            &mut scene,
            &resident,
            &format!("npc-{id}/"),
            position,
            false,
        );
    }

    append_shower_drops(&mut scene, random);
    scene
}

/// Stable IDs used by rendering, physical interaction and MSPA relationship targets.
pub fn residents() -> Vec<(u64, Avatar, [f64;3])> {
    vec![
        (2, Avatar {
            name: "Mara".into(), body: "delgado".into(), skin: "#b78467".into(),
            hair: "#382b24".into(), shirt: "#8a5360".into(), pants: "#484c4b".into()
        }, [-0.8,0.0,0.0]),
        (3, Avatar {
            name: "Nicolás".into(), body: "adulto".into(), skin: "#d4a181".into(),
            hair: "#342b23".into(), shirt: "#64785d".into(), pants: "#494d5b".into()
        }, [3.55,0.0,0.25]),
    ]
}
