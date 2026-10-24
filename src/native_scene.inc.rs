fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut shared: ResMut<Materials>,
    mut life: ResMut<Life>,
    capture: Res<Capture>,
) {
    // The supplied autumn-house FBX is converted to glTF and is the visible
    // architectural shell. SceneSpec contributes furniture, people and proxies.
    commands.spawn(SceneRoot(asset_server.load(house::VISUAL_ASSET)));

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
        if let Some(actor) = node["actor"].as_u64() {
            entity.insert(Person(actor));
        }
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
                min_width: 18.0,
                min_height: 13.0,
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
        let Life { game, society, .. } = &mut *life;
        society.tick(game, 0.05);
    }
}

fn interface(
    mut contexts: EguiContexts,
    mut life: ResMut<Life>,
    camera: Single<(&Camera, &GlobalTransform)>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let Life {
        game,
        avatar,
        changed,
        society,
        ..
    } = &mut *life;
    *changed |= ui::draw(ctx, game, avatar).avatar_changed;
    ui::draw_social(ctx, game, society);
    mspa_ui::draw(ctx, game, society);

    let frame = &game.frame;
    let rotation = Quat::from_euler(
        EulerRot::XYZ,
        frame.rotation[0] as f32,
        frame.rotation[1] as f32,
        frame.rotation[2] as f32,
    );
    let head = Vec3::new(
        frame.position.x as f32,
        frame.position.y as f32,
        frame.position.z as f32,
    ) + rotation * Vec3::new(0.0, 1.76, 0.0)
        + Vec3::Y * 0.4;
    let (camera, transform) = *camera;
    if let Ok(position) = camera.world_to_viewport(transform, head) {
        let size = camera.logical_viewport_size().unwrap_or(Vec2::ONE).max(Vec2::ONE);
        let screen = ctx.content_rect();
        ui::draw_reaction(
            ctx,
            game,
            [
                screen.left() + position.x * screen.width() / size.x,
                screen.top() + position.y * screen.height() / size.y,
            ],
        );
    }
    if !game.creator {
        let size = camera.logical_viewport_size().unwrap_or(Vec2::ONE).max(Vec2::ONE);
        let screen = ctx.content_rect();
        for resident in &society.residents {
            let p = resident.frame.position;
            // Larger stylized heads sit slightly higher than the old realistic rig.
            let head = Vec3::new(p.x as f32, p.y as f32 + 2.18, p.z as f32);
            if let Ok(position) = camera.world_to_viewport(transform, head) {
                ui::draw_resident(
                    ctx,
                    resident,
                    [
                        screen.left() + position.x * screen.width() / size.x,
                        screen.top() + position.y * screen.height() / size.y,
                    ],
                );
            }
        }
    }
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
    mut nodes: Query<(&NodeId, Option<&mut Mesh3d>, &mut Transform)>,
) {
    if !life.changed {
        return;
    }
    life.changed = false;
    let spec = scene::build(&life.avatar, &life.random);
    for (id, v) in spec.materials.as_object().unwrap() {
        if let Some(mat) = shared.0.get(id).and_then(|h| materials.get_mut(h)) {
            *mat = material(v);
        }
    }
    for (id, mesh, mut transform) in &mut nodes {
        if let Some(node) = spec.objects.iter().find(|n| n["id"] == id.0 && n["actor"] == 1) {
            if let Some(mut mesh) = mesh {
                mesh.0 = meshes.add(geometry::build(node).mesh());
            }
            transform.translation = geometry::vector(&node["position"], [0.0; 3]).as_vec3();
            transform.scale = geometry::vector(&node["scale"], [1.0; 3]).as_vec3();
        }
    }
    life.game.navigation = Navigation::new(geometry::navigation_bounds(&spec));
}
