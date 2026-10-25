fn intersection(mesh:&Mesh,origin:Vec3,direction:Vec3)->Option<f32>{
    let Some(VertexAttributeValues::Float32x3(p))=mesh.attribute(Mesh::ATTRIBUTE_POSITION)else{return None;};
    let indices:Vec<usize>=mesh.indices()?.iter().collect();let mut closest=f32::INFINITY;
    for tri in indices.chunks_exact(3){let a=Vec3::from_array(p[tri[0]]);let b=Vec3::from_array(p[tri[1]]);let c=Vec3::from_array(p[tri[2]]);let edge=b-a;let e2=c-a;let cross=direction.cross(e2);let det=edge.dot(cross);if det<1e-7{continue;}let inv=det.recip();let offset=origin-a;let u=offset.dot(cross)*inv;if !(0.0..=1.0).contains(&u){continue;}let q=offset.cross(edge);let v=direction.dot(q)*inv;if v<0.0||u+v>1.0{continue;}let t=e2.dot(q)*inv;if t>0.0{closest=closest.min(t);}}
    closest.is_finite().then_some(closest)
}

fn pick(
    buttons:Res<ButtonInput<MouseButton>>,mut contexts:EguiContexts,
    window:Single<&Window,With<PrimaryWindow>>,camera:Single<(&Camera,&GlobalTransform),With<CameraRig>>,
    furniture:Query<(&Furniture,&Mesh3d,&GlobalTransform)>,characters:Query<(&CharacterRoot,&GlobalTransform)>,
    meshes:Res<Assets<Mesh>>,mut life:ResMut<Life>,
){
    if !buttons.just_pressed(MouseButton::Left)||life.game.creator||life.game.paused{return;}
    if contexts.ctx_mut().is_ok_and(|ctx|ctx.is_pointer_over_area()){return;}
    let Some(cursor)=window.cursor_position()else{return;};let Ok(ray)=camera.0.viewport_to_world(camera.1,cursor)else{return;};

    // Character roots are selected with a small world-space capsule proxy, so complex
    // imported GLBs remain untouched while still being clickable.
    let mut character_hit=None;let mut character_t=f32::INFINITY;
    for(root,transform)in &characters{
        if root.profile_index==life.avatar.character{continue;}
        let center=transform.translation()+Vec3::Y*0.85;let delta=center-ray.origin;let t=delta.dot(*ray.direction);
        if t<=0.0{continue;}let closest=ray.origin+*ray.direction*t;let radius=0.58;
        if center.distance_squared(closest)<=radius*radius&&t<character_t{character_t=t;character_hit=Some(root.profile_index);}
    }

    let mut hit=None;let mut closest=f32::INFINITY;
    for(f,handle,transform)in &furniture{let Some(mesh)=meshes.get(&handle.0)else{continue;};let inverse=transform.affine().inverse();if let Some(t)=intersection(mesh,inverse.transform_point3(ray.origin),inverse.transform_vector3(*ray.direction))&&t<closest{closest=t;hit=Some(f.0);}}
    if let Some(profile)=character_hit&&character_t<closest{
        if let Some(resident)=life.society.residents.iter().find(|r|r.profile_index==profile){life.society.selected=Some(resident.id);life.society.status=format!("{} está afuera",resident.avatar.name);}
        return;
    }
    if let Some(task)=hit{
        let profile=life.avatar.character;
        let Life{game,society,haunting,poltergeist,..}=&mut *life;
        poltergeist.disturb(task,haunting,game,society,profile);
        return;
    }

    // Ground clicks intentionally do not move a resident. The user is an unseen
    // environmental agency; people decide where to go through their own MSPA state.
}

fn capture_frame(mut commands:Commands,mut capture:ResMut<Capture>){if capture.path.is_none()||capture.requested{return;}capture.frame+=1;if capture.frame>=90{capture.requested=true;let path=capture.path.clone().unwrap();commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path)).observe(|_:On<ScreenshotCaptured>,mut exit:MessageWriter<AppExit>|{exit.write(AppExit::Success);});}}
