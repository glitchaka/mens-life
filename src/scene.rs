//! Scene composition. The visible character and architecture come from the supplied assets.
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
            name: "Chica".into(), body: "asset".into(), skin: "#ffffff".into(),
            hair: "#ffffff".into(), shirt: "#ffffff".into(), pants: "#ffffff".into(),
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
    fn new(_avatar: &Avatar) -> Self {
        Self {
            background: 0x9fc8cf,
            fog: [25.0, 44.0],
            camera: json!({"bounds":[-10,10,7,-7],"near":0.1,"far":100}),
            lights: vec![],
            materials: json!({}),
            objects: vec![], obstacles: vec![], drops_y: vec![],
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn mesh(&mut self, kind:&str,args:&[f64],p:[f64;3],color:u32,roughness:f64,name:&str,shadows:[bool;2])->String {
        let id=format!("mesh-{}",self.objects.len());
        self.objects.push(json!({"id":id,"kind":kind,"args":args,"position":p,"name":name,
            "material":{"type":"standard","color":color,"roughness":roughness},
            "cast_shadow":shadows[0],"receive_shadow":shadows[1]}));
        id
    }
    pub(crate) fn box_mesh(&mut self,z:[f64;3],p:[f64;3],c:u32,n:&str)->String {
        let radius=0.1_f64.min(z[0].min(z[1]).min(z[2])*0.18);
        self.mesh("rounded-box",&[z[0],z[1],z[2],3.0,radius],p,c,0.68,n,[true,true])
    }
    pub(crate) fn cyl(&mut self,r:f64,h:f64,p:[f64;3],c:u32,n:&str)->String {
        self.mesh("cylinder",&[r,r,h,24.0],p,c,0.65,n,[true,false])
    }
    pub(crate) fn last(&mut self)->&mut Value { self.objects.last_mut().unwrap() }
    pub(crate) fn group(&mut self,id:&str,p:[f64;3],parent:Option<&str>) {
        self.objects.push(json!({"id":id,"kind":"group","position":p,"parent":parent}));
    }
}

fn append_shower_drops(s:&mut SceneSpec,random:&[f64]) {
    s.group("drops",[0.0;3],None); s.last()["visible"]=json!(false);
    for i in 0..30_usize {
        let sample=|j:usize|random.get(i*3+j).copied().unwrap_or(0.5);
        let y=sample(1)*2.15; s.drops_y.push(y);
        s.mesh("sphere",&[0.025,6.0,4.0],[-6.14+sample(0)*0.48,y,-4.35+sample(2)*0.62],0x98d6f6,1.0,"",[false,false]);
        s.last()["id"]=json!(format!("drop-{i}")); s.last()["parent"]=json!("drops");
        s.last()["scale"]=json!([1,4,1]);
        s.last()["material"]=json!({"type":"basic","color":0x98d6f6,"transparent":true,"opacity":0.65});
    }
}

pub fn build(avatar:&Avatar,random:&[f64])->SceneSpec {
    let mut scene=SceneSpec::new(avatar);
    crate::house::append(&mut scene);
    crate::furnishings::append(&mut scene);
    append_shower_drops(&mut scene,random);
    scene
}

/// No procedural residents: the only visible person is the supplied girl model.
pub fn residents()->Vec<(u64,Avatar,[f64;3])> { Vec::new() }
