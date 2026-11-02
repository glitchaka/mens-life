fn gltf_asset(asset_server:&AssetServer,path:&'static str)->PendingGltf {
    PendingGltf(asset_server.load(path))
}

fn resolve_gltf(
    mut commands:Commands,
    gltfs:Res<Assets<Gltf>>,
    pending:Query<(Entity,&PendingGltf)>,
){
    for(entity,pending)in &pending{
        let Some(gltf)=gltfs.get(&pending.0)else{continue;};
        if let Some(scene)=gltf.default_scene.clone(){
            commands.entity(entity).insert(SceneRoot(scene)).remove::<PendingGltf>();
        }else if gltf.scenes.len()==1{
            commands.entity(entity).insert(SceneRoot(gltf.scenes[0].clone())).remove::<PendingGltf>();
        }else if !gltf.scenes.is_empty(){
            commands.entity(entity).with_children(|parent|{
                for scene in &gltf.scenes{parent.spawn(SceneRoot(scene.clone()));}
            }).remove::<PendingGltf>();
        }
    }
}

fn setup(
    mut commands:Commands,
    asset_server:Res<AssetServer>,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<StandardMaterial>>,
    mut shared:ResMut<Materials>,
    mut life:ResMut<Life>,
    capture:Res<Capture>,
){
    // The supplied PSX Backroom is authored in large source units. 0.1 gives it a
    // usable ~27 x 32 metre footprint and ~2.3 metre ceiling. Translation centres
    // the room at the simulation origin and places its carpet/floor at Y = 0.
    commands.spawn((
        gltf_asset(&asset_server,"house/house.glb"),
        Name::new("PsxBackroomHouse"),
        Transform{
            translation:Vec3::new(-1.86438,0.196013,11.92789),
            rotation:Quat::IDENTITY,
            scale:Vec3::splat(0.1),
        },
    ));

    // Visible furnishings come from the GLB. Existing gameplay furniture remains
    // as invisible interaction proxies only.
    for(profile_index,spec)in CHARACTERS.iter().enumerate(){
        if !spec.available{continue;}
        let p=if profile_index==life.avatar.character{life.game.frame.position}else{life.society.residents.iter().find(|r|r.profile_index==profile_index).map(|r|r.frame.position).unwrap_or(Point::new(spec.outdoor_spawn[0],0.0,spec.outdoor_spawn[2]))};
        commands.spawn((
            gltf_asset(&asset_server,spec.asset),CharacterRoot{profile_index},Name::new(format!("Character:{}",spec.name)),
            Transform{translation:Vec3::new(p.x as f32,p.y as f32+spec.vertical_offset,p.z as f32),rotation:Quat::from_rotation_x(spec.rotation_x),scale:Vec3::splat(spec.scale)},
        ));
    }

    let spec=scene::build(&life.avatar,&life.random);
    for(id,value)in spec.materials.as_object().unwrap(){shared.0.insert(id.clone(),materials.add(material(value)));}
    let mut entities=HashMap::<String,Entity>::new();
    for node in &spec.objects{
        let id=node["id"].as_str().unwrap();let p=geometry::vector(&node["position"],[0.0;3]).as_vec3();let r=geometry::vector(&node["rotation"],[0.0;3]).as_vec3();
        let transform=Transform{translation:p,rotation:Quat::from_euler(EulerRot::XYZ,r.x,r.y,r.z),scale:geometry::vector(&node["scale"],[1.0;3]).as_vec3()};
        let visibility=if node["visible"]==false{Visibility::Hidden}else{Visibility::Inherited};
        let mut entity=commands.spawn((NodeId(id.into()),transform,visibility));
        if node["kind"]!="group"{
            let mat=if let Some(mid)=node["material_ref"].as_str(){shared.0[mid].clone()}else{let h=materials.add(material(&node["material"]));if id=="tv-screen"{shared.0.insert(id.into(),h.clone());}h};
            entity.insert((Mesh3d(meshes.add(geometry::build(node).mesh())),MeshMaterial3d(mat)));
            if !node["cast_shadow"].as_bool().unwrap_or(false){entity.insert(NotShadowCaster);}if !node["receive_shadow"].as_bool().unwrap_or(false){entity.insert(NotShadowReceiver);}
            let name=node["name"].as_str().unwrap_or("");if let Some(task)=Task::parse(name).or(if name=="sink"{Some(Task::Eat)}else{None}){entity.insert(Furniture(task));}
        }
        if let Some(parent)=node["parent"].as_str(){entity.insert(ChildOf(entities[parent]));}
        let entity=entity.id();entities.insert(id.into(),entity);
    }

    commands.spawn((DirectionalLight{color:rgb(0xfff0cf),illuminance:2.2,shadows_enabled:true,shadow_depth_bias:0.02,shadow_normal_bias:1.8,..default()},CascadeShadowConfigBuilder{num_cascades:1,minimum_distance:0.1,maximum_distance:55.0,..default()}.build(),Transform::from_xyz(-8.0,20.0,11.0).looking_at(Vec3::ZERO,Vec3::Y)));
    for [x,z] in [[0.0,0.0],[-8.0,-8.0],[8.0,-8.0],[-8.0,8.0],[8.0,8.0]]{
        commands.spawn((PointLight{color:rgb(0xffe6a8),intensity:900.0,range:11.0,shadows_enabled:false,..default()},Transform::from_xyz(x,2.15,z)));
    }
    let orbit=CameraOrbit::default();
    let p=life.game.ground_position;
    let focus=Vec3::new(p.x as f32,0.8,p.z as f32);
    let camera_pos=focus+Vec3::new(orbit.yaw.sin()*orbit.radius,orbit.height,orbit.yaw.cos()*orbit.radius);
    commands.spawn((CameraRig,Camera3d::default(),Projection::Orthographic(OrthographicProjection{near:0.1,far:120.0,scaling_mode:ScalingMode::AutoMin{min_width:18.0,min_height:11.25},..OrthographicProjection::default_3d()}),Transform::from_translation(camera_pos).looking_at(focus,Vec3::Y),Exposure{ev100:0.0},Tonemapping::None,Msaa::Sample4,ShadowFilteringMethod::Gaussian,DistanceFog{color:rgb(0x9ec7d0),directional_light_color:Color::NONE,falloff:FogFalloff::Linear{start:35.0,end:60.0},..default()}));

    if let Some(task)=capture.task{life.game.command(task);}
    for i in 0..capture.simulate{
        life.game.tick(0.05,i as f64*50.0);
        let profile=life.avatar.character;
        let Life{game,society,haunting,poltergeist,..}=&mut *life;
        society.tick(game,0.05);
        haunting.tick(game,society,profile,0.05);
        poltergeist.tick(0.05);
    }
}

fn interface(mut contexts:EguiContexts,mut life:ResMut<Life>,camera:Single<(&Camera,&GlobalTransform),With<CameraRig>>)->Result{
    let ctx=contexts.ctx_mut()?;
    if life.game.creator{
        let result=character_ui::draw(ctx,&mut life.avatar);
        life.changed|=result.changed;
        if result.start{life.game.creator=false;}
        return Ok(());
    }

    let Life{game,avatar,society,haunting,..}=&mut *life;
    let _=ui::draw(ctx,game,avatar);
    if !society.residents.is_empty(){ui::draw_social(ctx,game,society);}
    mspa_ui::draw(ctx,game,society,&avatar.name);
    if let Some(power)=haunting_ui::draw(ctx,haunting){
        haunting.invoke(power,game,society,avatar.character);
    }

    let frame=&game.frame;let rotation=Quat::from_euler(EulerRot::XYZ,frame.rotation[0]as f32,frame.rotation[1]as f32,frame.rotation[2]as f32);let head=Vec3::new(frame.position.x as f32,frame.position.y as f32,frame.position.z as f32)+rotation*Vec3::new(0.0,1.5,0.0);let(camera,transform)=*camera;
    if let Ok(position)=camera.world_to_viewport(transform,head){let size=camera.logical_viewport_size().unwrap_or(Vec2::ONE).max(Vec2::ONE);let screen=ctx.content_rect();ui::draw_reaction(ctx,game,[screen.left()+position.x*screen.width()/size.x,screen.top()+position.y*screen.height()/size.y]);}
    Ok(())
}
fn configure_interface(mut contexts:EguiContexts,mut configured:Local<bool>){if !*configured&&let Ok(ctx)=contexts.ctx_mut(){ui::configure(ctx);*configured=true;}}
