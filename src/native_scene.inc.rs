fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut shared: ResMut<Materials>,
    mut life: ResMut<Life>,
    capture: Res<Capture>,
) {
    // Visible architecture: the cutaway/lower shell baked directly from the supplied FBX.
    // This includes the exterior ground/patio; nothing procedural replaces the house.
    let house_material=materials.add(StandardMaterial{base_color:Color::WHITE,perceptual_roughness:0.9,..default()});
    commands.spawn((
        Mesh3d(meshes.add(house_asset::lower().mesh())),
        MeshMaterial3d(house_material),
        Transform::IDENTITY,
    ));

    // Only visible person: compact derivative of the supplied girl OBJ.
    let girl_material=materials.add(StandardMaterial{base_color:Color::WHITE,perceptual_roughness:0.88,..default()});
    let p=life.game.frame.position;
    commands.spawn((Girl,Person(1),Mesh3d(meshes.add(packed_mesh::girl())),MeshMaterial3d(girl_material),Transform::from_xyz(p.x as f32,p.y as f32,p.z as f32)));

    // SceneSpec contributes only allowed interior additions, interaction volumes and effects.
    let spec=scene::build(&life.avatar,&life.random);
    for (id,value) in spec.materials.as_object().unwrap(){shared.0.insert(id.clone(),materials.add(material(value)));}
    let mut entities=HashMap::<String,Entity>::new();
    for node in &spec.objects {
        let id=node["id"].as_str().unwrap();
        let p=geometry::vector(&node["position"],[0.0;3]).as_vec3();
        let r=geometry::vector(&node["rotation"],[0.0;3]).as_vec3();
        let transform=Transform{translation:p,rotation:Quat::from_euler(EulerRot::XYZ,r.x,r.y,r.z),scale:geometry::vector(&node["scale"],[1.0;3]).as_vec3()};
        let visibility=if node["visible"]==false{Visibility::Hidden}else{Visibility::Inherited};
        let mut entity=commands.spawn((NodeId(id.into()),transform,visibility));
        if node["kind"]!="group" {
            let mat=if let Some(mid)=node["material_ref"].as_str(){shared.0[mid].clone()}else{
                let h=materials.add(material(&node["material"])); if id=="tv-screen"{shared.0.insert(id.into(),h.clone());} h
            };
            entity.insert((Mesh3d(meshes.add(geometry::build(node).mesh())),MeshMaterial3d(mat)));
            if !node["cast_shadow"].as_bool().unwrap_or(false){entity.insert(NotShadowCaster);}
            if !node["receive_shadow"].as_bool().unwrap_or(false){entity.insert(NotShadowReceiver);}
            let name=node["name"].as_str().unwrap_or("");
            if let Some(task)=Task::parse(name).or(if name=="sink"{Some(Task::Eat)}else{None}){entity.insert(Furniture(task));}
        }
        if let Some(parent)=node["parent"].as_str(){entity.insert(ChildOf(entities[parent]));}
        let entity=entity.id(); entities.insert(id.into(),entity);
    }

    commands.spawn((DirectionalLight{color:rgb(0xfff0cf),illuminance:3.3,shadows_enabled:true,shadow_depth_bias:0.02,shadow_normal_bias:1.8,..default()},
        CascadeShadowConfigBuilder{num_cascades:1,minimum_distance:0.1,maximum_distance:55.0,..default()}.build(),
        Transform::from_xyz(-8.0,18.0,11.0).looking_at(Vec3::new(-1.0,0.0,-0.5),Vec3::Y)));
    commands.spawn((PointLight{color:rgb(0xffd89b),intensity:18.0*std::f32::consts::TAU*2.0,range:9.0,..default()},Transform::from_xyz(0.2,2.7,0.1)));
    commands.spawn((Camera3d::default(),Projection::Orthographic(OrthographicProjection{near:0.1,far:120.0,scaling_mode:ScalingMode::AutoMin{min_width:20.0,min_height:14.0},..OrthographicProjection::default_3d()}),
        Transform::from_xyz(16.0,18.0,19.0).looking_at(Vec3::new(-1.0,0.0,-0.5),Vec3::Y),Exposure{ev100:0.0},Tonemapping::None,Msaa::Sample4,ShadowFilteringMethod::Gaussian,
        DistanceFog{color:rgb(0x9ec7d0),directional_light_color:Color::NONE,falloff:FogFalloff::Linear{start:30.0,end:55.0},..default()}));

    if let Some(task)=capture.task { life.game.command(task); }
    for i in 0..capture.simulate { life.game.tick(0.05,i as f64*50.0); let Life{game,society,..}=&mut *life; society.tick(game,0.05); }
}

fn interface(mut contexts:EguiContexts,mut life:ResMut<Life>,camera:Single<(&Camera,&GlobalTransform)>)->Result {
    let ctx=contexts.ctx_mut()?; let Life{game,avatar,changed,society,..}=&mut *life;
    *changed|=ui::draw(ctx,game,avatar).avatar_changed;
    if !society.residents.is_empty(){ui::draw_social(ctx,game,society);}
    mspa_ui::draw(ctx,game,society,&avatar.name);
    let frame=&game.frame; let rotation=Quat::from_euler(EulerRot::XYZ,frame.rotation[0] as f32,frame.rotation[1] as f32,frame.rotation[2] as f32);
    let head=Vec3::new(frame.position.x as f32,frame.position.y as f32,frame.position.z as f32)+rotation*Vec3::new(0.0,1.48,0.0);
    let (camera,transform)=*camera;
    if let Ok(position)=camera.world_to_viewport(transform,head){let size=camera.logical_viewport_size().unwrap_or(Vec2::ONE).max(Vec2::ONE);let screen=ctx.content_rect();
        ui::draw_reaction(ctx,game,[screen.left()+position.x*screen.width()/size.x,screen.top()+position.y*screen.height()/size.y]);}
    Ok(())
}
fn configure_interface(mut contexts:EguiContexts,mut configured:Local<bool>){if !*configured&&let Ok(ctx)=contexts.ctx_mut(){ui::configure(ctx);*configured=true;}}
fn avatar_changed(mut life:ResMut<Life>){if !life.changed{return;}life.changed=false;let spec=scene::build(&life.avatar,&life.random);life.game.navigation=Navigation::new(geometry::navigation_bounds(&spec));}
