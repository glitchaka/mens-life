use bevy::{
    animation::{AnimationPlayer, graph::{AnimationGraph, AnimationGraphHandle, AnimationNodeIndex}},
    asset::AssetPlugin,
    camera::{Exposure, ScalingMode},
    core_pipeline::tonemapping::Tonemapping,
    gltf::{Gltf, GltfAssetLabel},
    light::{CascadeShadowConfigBuilder, DirectionalLightShadowMap, NotShadowCaster, NotShadowReceiver, ShadowFilteringMethod},
    mesh::VertexAttributeValues,
    prelude::*,
    render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
    scene::SceneRoot,
    window::{PrimaryWindow, WindowResolution},
};
use bevy_egui::{EguiContexts, EguiPlugin, EguiPreUpdateSet, EguiPrimaryContextPass};
use mens_life::{
    character_catalog::{self, CHARACTERS}, character_ui,
    game::{self, Game, Navigation, Task, Vec3 as Point},
    geometry,
    haunting::Haunting,
    haunting_ui,
    mspa_ui, perception,
    poltergeist::Poltergeist,
    scene::{self, Avatar}, society::Society, ui,
};
use serde_json::Value;
use std::{collections::HashMap, path::PathBuf};

#[derive(Resource)]
struct Life { game: Game, avatar: Avatar, society: Society, haunting: Haunting, poltergeist: Poltergeist, changed: bool, random: Vec<f64> }
#[derive(Component)] struct NodeId(String);
#[derive(Component)] struct Furniture(Task);
#[derive(Component)] struct PendingGltf(Handle<Gltf>);
#[derive(Component)] struct CharacterRoot { profile_index: usize }
#[derive(Component)] struct CameraRig;
#[derive(Clone, Copy, Component, PartialEq, Eq)] enum CutawaySide { Roof, East, West, North, South }
#[derive(Component)] struct CutawayPart(CutawaySide);
#[derive(Component)] struct CharacterAnimation { profile_index: usize, idle: Option<AnimationNodeIndex>, walk: Option<AnimationNodeIndex>, walking: bool }
#[derive(Resource)] struct CameraOrbit { yaw: f32, radius: f32, height: f32 }
impl Default for CameraOrbit { fn default() -> Self { Self { yaw: 0.72, radius: 15.5, height: 11.5 } } }
#[derive(Resource, Default)] struct Materials(HashMap<String, Handle<StandardMaterial>>);
#[derive(Resource, Default)] struct Capture { path: Option<String>, frame: u32, requested: bool, skip_creator: bool, task: Option<Task>, simulate: u32 }

fn color(v:&Value)->Color { let n=v.as_u64().map(|n|n as u32).or_else(||v.as_str().and_then(|s|u32::from_str_radix(s.trim_start_matches('#'),16).ok())).unwrap_or(0xffffff);rgb(n) }
fn rgb(n:u32)->Color { Color::srgb_u8((n>>16)as u8,(n>>8)as u8,n as u8) }
fn material(v:&Value)->StandardMaterial { let c=color(&v["color"]);let alpha=v["opacity"].as_f64().unwrap_or(1.0)as f32;StandardMaterial{base_color:c.with_alpha(alpha),perceptual_roughness:v["roughness"].as_f64().unwrap_or(1.0)as f32,emissive:if v["emissive"].is_null(){LinearRgba::BLACK}else{color(&v["emissive"]).to_linear()},alpha_mode:if v["transparent"].as_bool().unwrap_or(false){AlphaMode::Blend}else{AlphaMode::Opaque},unlit:v["type"]=="basic",..default()} }

fn asset_root()->String {
    let mut candidates=Vec::<PathBuf>::new();
    if let Ok(cwd)=std::env::current_dir(){candidates.push(cwd.join("assets"));}
    if let Ok(exe)=std::env::current_exe(){
        if let Some(dir)=exe.parent(){
            candidates.push(dir.join("assets"));
            if let Some(repo)=dir.parent().and_then(|p|p.parent()){candidates.push(repo.join("assets"));}
        }
    }
    candidates.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets"));
    candidates.into_iter().find(|p|p.is_dir()).unwrap_or_else(||PathBuf::from("assets")).to_string_lossy().into_owned()
}

pub fn run(){
    let mut capture=Capture::default();let mut args=std::env::args().skip(1);
    while let Some(arg)=args.next(){match arg.as_str(){"--capture"=>capture.path=args.next(),"--start"=>capture.skip_creator=true,"--task"=>capture.task=args.next().and_then(|s|Task::parse(&s)),"--simulate"=>capture.simulate=args.next().and_then(|s|s.parse().ok()).unwrap_or(0),"--help"=>{println!("Vida Isométrica 3D\n  cargo run --release\n  --capture archivo.png [--start] [--task bed|tv|shower|toilet|eat] [--simulate frames]");return;},_=>{eprintln!("Argumento desconocido: {arg}");std::process::exit(2);}}}
    let mut seed=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos()as u64;
    let random=(0..90).map(|_|{seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1);(seed>>11)as f64/(1u64<<53)as f64}).collect::<Vec<_>>();
    let avatar=Avatar::default();let spec=scene::build(&avatar,&random);let mut game=Game::new(Navigation::new(geometry::navigation_bounds(&spec)),spec.drops_y);game.set_profile(avatar.character as u64+1);game.creator=!capture.skip_creator;let society=Society::new(&game.navigation,avatar.character);
    let assets=asset_root();
    App::new()
        .register_type::<Transform>()
        .register_type::<GlobalTransform>()
        .insert_resource(Life{game,avatar,society,haunting:Haunting::default(),poltergeist:Poltergeist::default(),changed:false,random})
        .insert_resource(capture).insert_resource(CameraOrbit::default())
        .insert_resource(ClearColor(rgb(0x9ec7d0))).insert_resource(AmbientLight{color:rgb(0xfff6df),brightness:1.6,..default()}).insert_resource(DirectionalLightShadowMap{size:2048}).init_resource::<Materials>()
        .add_plugins(DefaultPlugins
            .set(AssetPlugin{file_path:assets,..default()})
            .set(WindowPlugin{primary_window:Some(Window{title:"Vida Isométrica 3D".into(),resolution:WindowResolution::new(1280,800),..default()}),..default()}))
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup,setup)
        .add_systems(PreUpdate,configure_interface.after(EguiPreUpdateSet::InitContexts).before(EguiPreUpdateSet::BeginPass))
        .add_systems(EguiPrimaryContextPass,interface)
        .add_systems(Update,(apply_selection,advance,perceive_world,resolve_gltf,tag_cutaway_parts,setup_character_animations,rotate_camera,animate,pick,capture_frame).chain())
        .run();
}
include!("native_scene.inc.rs");
include!("native_runtime.inc.rs");
include!("native_interaction.inc.rs");
