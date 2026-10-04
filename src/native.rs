use bevy::{
    camera::{Exposure, ScalingMode},
    core_pipeline::tonemapping::Tonemapping,
    light::{
        CascadeShadowConfigBuilder, DirectionalLightShadowMap, NotShadowCaster, NotShadowReceiver,
        ShadowFilteringMethod,
    },
    mesh::VertexAttributeValues,
    prelude::*,
    render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
    window::{PrimaryWindow, WindowResolution},
};
use bevy_egui::{EguiContexts, EguiPlugin, EguiPreUpdateSet, EguiPrimaryContextPass};
use mens_life::{
    game::{Game, Navigation, Task, Vec3 as Point},
    geometry,
    scene::{self, Avatar},
    ui,
};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Resource)]
struct Life {
    game: Game,
    avatar: Avatar,
    changed: bool,
    random: Vec<f64>,
}
#[derive(Component)]
struct NodeId(String);
#[derive(Component)]
struct Furniture(Task);
#[derive(Resource, Default)]
struct Materials(HashMap<String, Handle<StandardMaterial>>);
#[derive(Resource, Default)]
struct Capture {
    path: Option<String>,
    frame: u32,
    requested: bool,
    skip_creator: bool,
    task: Option<Task>,
    simulate: u32,
}

fn color(v: &Value) -> Color {
    let n = v
        .as_u64()
        .map(|n| n as u32)
        .or_else(|| {
            v.as_str()
                .and_then(|s| u32::from_str_radix(s.trim_start_matches('#'), 16).ok())
        })
        .unwrap_or(0xffffff);
    rgb(n)
}
fn rgb(n: u32) -> Color {
    Color::srgb_u8((n >> 16) as u8, (n >> 8) as u8, n as u8)
}
fn material(v: &Value) -> StandardMaterial {
    let c = color(&v["color"]);
    let alpha = v["opacity"].as_f64().unwrap_or(1.0) as f32;
    StandardMaterial {
        base_color: c.with_alpha(alpha),
        perceptual_roughness: v["roughness"].as_f64().unwrap_or(1.0) as f32,
        emissive: if v["emissive"].is_null() {
            LinearRgba::BLACK
        } else {
            color(&v["emissive"]).to_linear()
        },
        alpha_mode: if v["transparent"].as_bool().unwrap_or(false) {
            AlphaMode::Blend
        } else {
            AlphaMode::Opaque
        },
        unlit: v["type"] == "basic",
        ..default()
    }
}
pub fn run() {
    let mut capture = Capture::default();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--capture" => capture.path = args.next(),
            "--start" => capture.skip_creator = true,
            "--task" => capture.task = args.next().and_then(|s| Task::parse(&s)),
            "--simulate" => {
                capture.simulate = args.next().and_then(|s| s.parse().ok()).unwrap_or(0)
            }
            "--help" => {
                println!(
                    "Vida Isométrica 3D\n  cargo run --release\n  --capture archivo.png [--start] [--task bed|tv|shower|toilet|eat] [--simulate frames]"
                );
                return;
            }
            _ => {
                eprintln!("Argumento desconocido: {arg}");
                std::process::exit(2);
            }
        }
    }
    let mut seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let random = (0..90)
        .map(|_| {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            (seed >> 11) as f64 / (1u64 << 53) as f64
        })
        .collect::<Vec<_>>();
    let avatar = Avatar::default();
    let spec = scene::build(&avatar, &random);
    let game = Game::new(
        Navigation::new(geometry::navigation_bounds(&spec)),
        spec.drops_y,
    );
    App::new()
        .insert_resource(Life {
            game,
            avatar,
            changed: false,
            random,
        })
        .insert_resource(capture)
        .insert_resource(ClearColor(rgb(0x9ec7d0)))
        .insert_resource(AmbientLight {
            color: rgb(0xfff6df),
            brightness: 1.6,
            ..default()
        })
        .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .init_resource::<Materials>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Vida Isométrica 3D".into(),
                resolution: WindowResolution::new(1280, 800),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            PreUpdate,
            configure_interface
                .after(EguiPreUpdateSet::InitContexts)
                .before(EguiPreUpdateSet::BeginPass),
        )
        .add_systems(EguiPrimaryContextPass, interface)
        .add_systems(
            Update,
            (avatar_changed, advance, animate, pick, capture_frame).chain(),
        )
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut shared: ResMut<Materials>,
    mut life: ResMut<Life>,
    capture: Res<Capture>,
) {
    let spec = scene::build(&life.avatar, &life.random);
    for (id, value) in spec.materials.as_object().unwrap() {
        shared.0.insert(id.clone(), materials.add(material(value)));
    }
    let mut entities = HashMap::<String, Entity>::new();
    for node in &spec.objects {
        let id = node["id"].as_str().unwrap();
        let p = geometry::vector(&node["position"], [0.0; 3]).as_vec3();
        let r = geometry::vector(&node["rotation"], [0.0; 3]).as_vec3();
        let transform = Transform {
            translation: p,
            rotation: Quat::from_euler(EulerRot::XYZ, r.x, r.y, r.z),
            scale: geometry::vector(&node["scale"], [1.0; 3]).as_vec3(),
        };
        let visibility = if node["visible"] == false {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
        let mut entity = commands.spawn((NodeId(id.into()), transform, visibility));
        if node["kind"] != "group" {
            let mat = if let Some(id) = node["material_ref"].as_str() {
                shared.0[id].clone()
            } else {
                let h = materials.add(material(&node["material"]));
                if id == "tv-screen" {
                    shared.0.insert(id.into(), h.clone());
                }
                h
            };
            entity.insert((
                Mesh3d(meshes.add(geometry::build(node).mesh())),
                MeshMaterial3d(mat),
            ));
            if !node["cast_shadow"].as_bool().unwrap_or(false) {
                entity.insert(NotShadowCaster);
            }
            if !node["receive_shadow"].as_bool().unwrap_or(false) {
                entity.insert(NotShadowReceiver);
            }
            let name = node["name"].as_str().unwrap_or("");
            if let Some(task) = Task::parse(name).or(if name == "sink" {
                Some(Task::Eat)
            } else {
                None
            }) {
                entity.insert(Furniture(task));
            }
        }
        if let Some(parent) = node["parent"].as_str() {
            entity.insert(ChildOf(entities[parent]));
        }
        let entity = entity.id();
        entities.insert(id.into(), entity);
    }
    commands.spawn((
        DirectionalLight {
            color: rgb(0xfff0cf),
            illuminance: 3.3,
            shadows_enabled: true,
            shadow_depth_bias: 0.02,
            shadow_normal_bias: 1.8,
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 1,
            minimum_distance: 0.1,
            maximum_distance: 50.0,
            ..default()
        }
        .build(),
        Transform::from_xyz(-8.0, 16.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // Convert the point light's candela to lumens for Bevy's physical units.
    commands.spawn((
        PointLight {
            color: rgb(0xffd89b),
            intensity: 18.0 * std::f32::consts::TAU * 2.0,
            range: 8.0,
            ..default()
        },
        Transform::from_xyz(0.2, 2.7, 0.1),
    ));
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            near: 0.1,
            far: 100.0,
            scaling_mode: ScalingMode::AutoMin {
                min_width: 23.0,
                min_height: 17.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(15.0, 17.0, 18.0).looking_at(Vec3::ZERO, Vec3::Y),
        Exposure { ev100: 0.0 },
        Tonemapping::None,
        Msaa::Sample4,
        ShadowFilteringMethod::Gaussian,
        DistanceFog {
            color: rgb(0x9ec7d0),
            directional_light_color: Color::NONE,
            falloff: FogFalloff::Linear {
                start: 25.0,
                end: 44.0,
            },
            ..default()
        },
    ));
    if capture.skip_creator {
        life.game.creator = false;
    }
    if let Some(task) = capture.task {
        life.game.creator = false;
        life.game.command(task);
    }
    for i in 0..capture.simulate {
        life.game.tick(0.05, i as f64 * 50.0);
    }
}
fn interface(mut contexts: EguiContexts, mut life: ResMut<Life>) -> Result {
    let ctx = contexts.ctx_mut()?;
    let Life {
        game,
        avatar,
        changed,
        ..
    } = &mut *life;
    *changed |= ui::draw(ctx, game, avatar).avatar_changed;
    Ok(())
}
fn configure_interface(mut contexts: EguiContexts, mut configured: Local<bool>) {
    if !*configured && let Ok(ctx) = contexts.ctx_mut() {
        ui::configure(ctx);
        *configured = true;
    }
}
fn avatar_changed(
    mut life: ResMut<Life>,
    shared: Res<Materials>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut nodes: Query<(&NodeId, &mut Mesh3d, &mut Transform)>,
) {
    if !life.changed {
        return;
    }
    life.changed = false;
    let spec = scene::build(&life.avatar, &life.random);
    for (id, v) in spec.materials.as_object().unwrap() {
        if let Some(mat) = materials.get_mut(&shared.0[id]) {
            *mat = material(v);
        }
    }
    for (id, mut mesh, mut transform) in &mut nodes {
        if id.0 == "torso" {
            let node = spec.objects.iter().find(|n| n["id"] == "torso").unwrap();
            mesh.0 = meshes.add(geometry::build(node).mesh());
            transform.scale = geometry::vector(&node["scale"], [1.0; 3]).as_vec3();
        }
    }
    let creator = life.game.creator;
    let paused = life.game.paused;
    life.game = Game::new(
        Navigation::new(geometry::navigation_bounds(&spec)),
        spec.drops_y,
    );
    life.game.creator = creator;
    life.game.paused = paused;
}
fn advance(
    time: Res<Time>,
    mut life: ResMut<Life>,
    capture: Res<Capture>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) && !life.game.creator {
        life.game.paused = !life.game.paused;
    }
    if keys.just_pressed(KeyCode::Escape) {
        life.game.creator = !life.game.creator;
    }
    // Captures preserve the requested state while the GPU warms its pipelines.
    if capture.path.is_none() {
        life.game
            .tick(time.delta_secs_f64(), time.elapsed_secs_f64() * 1000.0);
    }
}
fn animate(
    life: Res<Life>,
    shared: Res<Materials>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut nodes: Query<(&NodeId, &mut Transform, &mut Visibility)>,
) {
    let f = &life.game.frame;
    for (id, mut t, mut v) in &mut nodes {
        match id.0.as_str() {
            "person" => {
                t.translation = Vec3::new(
                    f.position.x as f32,
                    f.position.y as f32,
                    f.position.z as f32,
                );
                t.rotation = Quat::from_euler(
                    EulerRot::XYZ,
                    f.rotation[0] as f32,
                    f.rotation[1] as f32,
                    f.rotation[2] as f32,
                );
            }
            "leg-0" => t.rotation = Quat::from_rotation_x(f.legs[0] as f32),
            "leg-1" => t.rotation = Quat::from_rotation_x(f.legs[1] as f32),
            "arm-0" => t.rotation = Quat::from_rotation_x(f.arms[0] as f32),
            "arm-1" => t.rotation = Quat::from_rotation_x(f.arms[1] as f32),
            "plumb" => {
                *v = if f.plumb {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                }
            }
            "drops" => {
                *v = if f.drops {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                }
            }
            s if s.starts_with("drop-") => {
                let i = s[5..].parse::<usize>().unwrap();
                t.translation.y = life.game.drops_y[i] as f32;
            }
            _ => {}
        }
    }
    if let Some(mat) = shared.0.get("tv-screen").and_then(|h| materials.get_mut(h)) {
        mat.base_color = rgb(if f.watching { 0x79a7d3 } else { 0x17212b });
        mat.emissive = Color::hsl(
            f.screen_hue as f32 * 360.0,
            0.4,
            if f.watching { 0.4 } else { 0.03 },
        )
        .to_linear();
    }
}
// Möller–Trumbore intersections use the same native triangles as the GPU.
fn intersection(mesh: &Mesh, origin: Vec3, direction: Vec3) -> Option<f32> {
    let Some(VertexAttributeValues::Float32x3(p)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
        return None;
    };
    let indices: Vec<usize> = mesh.indices()?.iter().collect();
    let mut closest = f32::INFINITY;
    for tri in indices.chunks_exact(3) {
        let a = Vec3::from_array(p[tri[0]]);
        let b = Vec3::from_array(p[tri[1]]);
        let c = Vec3::from_array(p[tri[2]]);
        let edge = b - a;
        let e2 = c - a;
        let cross = direction.cross(e2);
        let det = edge.dot(cross);
        if det < 1e-7 {
            continue;
        }
        let inv = det.recip();
        let offset = origin - a;
        let u = offset.dot(cross) * inv;
        if !(0.0..=1.0).contains(&u) {
            continue;
        }
        let q = offset.cross(edge);
        let v = direction.dot(q) * inv;
        if v < 0.0 || u + v > 1.0 {
            continue;
        }
        let t = e2.dot(q) * inv;
        if t > 0.0 {
            closest = closest.min(t);
        }
    }
    closest.is_finite().then_some(closest)
}
fn pick(
    buttons: Res<ButtonInput<MouseButton>>,
    mut contexts: EguiContexts,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    furniture: Query<(&Furniture, &Mesh3d, &GlobalTransform)>,
    meshes: Res<Assets<Mesh>>,
    mut life: ResMut<Life>,
) {
    if !buttons.just_pressed(MouseButton::Left) || life.game.creator || life.game.paused {
        return;
    }
    if contexts
        .ctx_mut()
        .is_ok_and(|ctx| ctx.is_pointer_over_area())
    {
        return;
    }
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Ok(ray) = camera.0.viewport_to_world(camera.1, cursor) else {
        return;
    };
    let mut hit = None;
    let mut closest = f32::INFINITY;
    for (f, handle, transform) in &furniture {
        let Some(mesh) = meshes.get(&handle.0) else {
            continue;
        };
        let inverse = transform.affine().inverse();
        if let Some(t) = intersection(
            mesh,
            inverse.transform_point3(ray.origin),
            inverse.transform_vector3(*ray.direction),
        ) && t < closest
        {
            closest = t;
            hit = Some(f.0);
        }
    }
    if let Some(task) = hit {
        life.game.command(task);
        return;
    }
    if ray.direction.y.abs() > 1e-6 {
        let t = (-0.045 - ray.origin.y) / ray.direction.y;
        let p = ray.origin + *ray.direction * t;
        if t > 0.0 && p.x.abs() < 7.5 && p.z.abs() < 5.5 {
            life.game.walk_to(Point::new(p.x as f64, 0.0, p.z as f64));
        }
    }
}
fn capture_frame(mut commands: Commands, mut capture: ResMut<Capture>) {
    if capture.path.is_none() || capture.requested {
        return;
    }
    capture.frame += 1;
    if capture.frame >= 90 {
        capture.requested = true;
        let path = capture.path.clone().unwrap();
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path))
            .observe(
                |_: On<ScreenshotCaptured>, mut exit: MessageWriter<AppExit>| {
                    exit.write(AppExit::Success);
                },
            );
    }
}
