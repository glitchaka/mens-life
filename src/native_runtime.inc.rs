fn apply_selection(mut life:ResMut<Life>){
    if !life.changed{return;}
    life.changed=false;
    let profile=life.avatar.character;
    life.game.set_profile(profile as u64+1);
    let navigation=life.game.navigation.clone();
    life.society=Society::new(&navigation,profile);
}

fn advance(time:Res<Time>,mut life:ResMut<Life>,capture:Res<Capture>,keys:Res<ButtonInput<KeyCode>>){
    if keys.just_pressed(KeyCode::Space){life.game.paused=!life.game.paused;}
    if capture.path.is_none(){
        let dt=time.delta_secs_f64();
        life.game.tick(dt,time.elapsed_secs_f64()*1000.0);
        let profile=life.avatar.character;
        let Life{game,society,haunting,poltergeist,..}=&mut *life;
        society.tick(game,dt);
        haunting.tick(game,society,profile,dt);
        poltergeist.tick(dt);
    }
}
fn perceive_world(mut life:ResMut<Life>,mut awareness:Local<perception::WorldAwareness>){let Life{game,society,..}=&mut *life;perception::observe(game,society,&mut awareness);}

fn tag_cutaway_parts(mut commands:Commands,names:Query<(Entity,&Name),Added<Name>>){
    for(entity,name)in &names{
        let s=name.as_str();
        let side=if s.starts_with("CUTAWAY_ROOF__")
            ||s=="Backroom_lambert5_0"
            ||s=="Backroom_lambert6_0"
            ||s=="Backroom_lambert7_0"
        {Some(CutawaySide::Roof)}else if s.starts_with("CUTAWAY_EAST__"){Some(CutawaySide::East)}else if s.starts_with("CUTAWAY_WEST__"){Some(CutawaySide::West)}else if s.starts_with("CUTAWAY_NORTH__"){Some(CutawaySide::North)}else if s.starts_with("CUTAWAY_SOUTH__"){Some(CutawaySide::South)}else{None};
        if let Some(side)=side{commands.entity(entity).insert(CutawayPart(side));}
    }
}

fn find_character_root(mut entity:Entity,parents:&Query<&ChildOf>,roots:&Query<(&CharacterRoot,&CharacterSource)>)->Option<(usize,Handle<Gltf>)>{
    loop{
        if let Ok((root,source))=roots.get(entity){return Some((root.profile_index,source.0.clone()));}
        entity=parents.get(entity).ok()?.parent();
    }
}

fn named_animation(gltf:&Gltf,keywords:&[&str])->Option<Handle<AnimationClip>>{
    for keyword in keywords{
        for(name,clip)in &gltf.named_animations{
            let normalized=name.to_ascii_lowercase().replace(['-','_','.'],' ');
            if normalized.split_whitespace().any(|part|part==*keyword)||normalized.contains(keyword){
                return Some(clip.clone());
            }
        }
    }
    None
}

fn bone_side(name:&str)->Option<bool>{
    let n=name.to_ascii_lowercase();
    let left=n.contains(".l")||n.contains("_l")||n.contains("left");
    let right=n.contains(".r")||n.contains("_r")||n.contains("right");
    match(left,right){(true,false)=>Some(true),(false,true)=>Some(false),_=>None}
}

fn procedural_bone_kind(name:&str)->Option<ProceduralBoneKind>{
    let n=name.to_ascii_lowercase();
    let left=bone_side(&n)?;
    let lower_leg=n.contains("shin")||n.contains("lower_leg")||n.contains("lowerleg")||n.contains("calf");
    let upper_leg=(n.contains("thigh")||n.contains("upper_leg")||n.contains("upperleg")||n.contains("upleg")||n.contains("leg."))&&!lower_leg&&!n.contains("foot")&&!n.contains("toe");
    let lower_arm=n.contains("forearm")||n.contains("lower_arm")||n.contains("lowerarm");
    let upper_arm=(n.contains("upper_arm")||n.contains("upperarm")||n.contains("arm."))&&!lower_arm&&!n.contains("hand")&&!n.contains("finger");
    if upper_leg{return Some(if left{ProceduralBoneKind::UpperLegL}else{ProceduralBoneKind::UpperLegR});}
    if lower_leg{return Some(if left{ProceduralBoneKind::LowerLegL}else{ProceduralBoneKind::LowerLegR});}
    if upper_arm{return Some(if left{ProceduralBoneKind::UpperArmL}else{ProceduralBoneKind::UpperArmR});}
    if lower_arm{return Some(if left{ProceduralBoneKind::LowerArmL}else{ProceduralBoneKind::LowerArmR});}
    None
}

fn tag_procedural_bones(
    mut commands:Commands,
    names:Query<(Entity,&Name,&Transform),Added<Name>>,
    parents:Query<&ChildOf>,
    roots:Query<(&CharacterRoot,&CharacterSource)>,
){
    for(entity,name,transform)in &names{
        let Some((profile,_))=find_character_root(entity,&parents,&roots)else{continue;};
        if !character_catalog::character(profile).procedural_walk{continue;}
        if let Some(kind)=procedural_bone_kind(name.as_str()){
            commands.entity(entity).insert(ProceduralBone{profile_index:profile,kind,rest:transform.rotation});
        }
    }
}

fn setup_character_animations(
    mut commands:Commands,mut graphs:ResMut<Assets<AnimationGraph>>,gltfs:Res<Assets<Gltf>>,
    parents:Query<&ChildOf>,roots:Query<(&CharacterRoot,&CharacterSource)>,mut players:Query<(Entity,&mut AnimationPlayer),Added<AnimationPlayer>>,
){
    for(entity,mut player)in &mut players{
        let Some((profile,source))=find_character_root(entity,&parents,&roots)else{continue;};
        let Some(gltf)=gltfs.get(&source)else{continue;};
        let spec=character_catalog::character(profile);
        let idle_clip=named_animation(gltf,&["standby","idle","standing","stand","breath","breathing","wait"])
            .or_else(||spec.idle_animation.and_then(|i|gltf.animations.get(i).cloned()))
            .or_else(||gltf.animations.first().cloned());
        let walk_clip=named_animation(gltf,&["step","walk","walking","locomotion","stroll"])
            .or_else(||spec.walk_animation.and_then(|i|gltf.animations.get(i).cloned()));
        if idle_clip.is_none()&&walk_clip.is_none(){continue;}
        let mut graph=AnimationGraph::new();
        let idle=idle_clip.map(|clip|graph.add_clip(clip,1.0,graph.root));
        let walk=walk_clip.map(|clip|graph.add_clip(clip,1.0,graph.root));
        let handle=graphs.add(graph);
        commands.entity(entity).insert((AnimationGraphHandle(handle),CharacterAnimation{profile_index:profile,idle,walk,walking:false}));
        if let Some(node)=idle.or(walk){player.play(node).repeat();}
    }
}

fn rotate_camera(
    time:Res<Time>,keys:Res<ButtonInput<KeyCode>>,buttons:Res<ButtonInput<MouseButton>>,
    mouse:Res<bevy::input::mouse::AccumulatedMouseMotion>,mut orbit:ResMut<CameraOrbit>,
    mut camera:Single<&mut Transform,With<CameraRig>>,life:Res<Life>,mut cutaway:Query<(&CutawayPart,&mut Visibility)>,
){
    let mut delta=0.0;
    if keys.pressed(KeyCode::KeyQ)||keys.pressed(KeyCode::ArrowLeft){delta+=time.delta_secs()*1.25;}
    if keys.pressed(KeyCode::KeyE)||keys.pressed(KeyCode::ArrowRight){delta-=time.delta_secs()*1.25;}
    if buttons.pressed(MouseButton::Right){delta-=mouse.delta.x*0.008;}
    orbit.yaw=(orbit.yaw+delta).rem_euclid(std::f32::consts::TAU);

    let p=life.game.ground_position;
    let focus=Vec3::new(p.x as f32,0.9,p.z as f32);
    let offset=Vec3::new(orbit.yaw.sin()*orbit.radius,orbit.height,orbit.yaw.cos()*orbit.radius);
    **camera=Transform::from_translation(focus+offset).looking_at(focus,Vec3::Y);

    let indoors=(-23.45..=23.45).contains(&p.x)&&(-27.51..=27.51).contains(&p.z);
    let sx=offset.x.signum();let sz=offset.z.signum();
    for(part,mut visibility)in &mut cutaway{
        let hide=indoors&&match part.0{
            CutawaySide::Roof=>true,
            CutawaySide::East=>sx>0.0&&offset.x.abs()>orbit.radius*0.24,
            CutawaySide::West=>sx<0.0&&offset.x.abs()>orbit.radius*0.24,
            CutawaySide::North=>sz>0.0&&offset.z.abs()>orbit.radius*0.24,
            CutawaySide::South=>sz<0.0&&offset.z.abs()>orbit.radius*0.24,
        };
        *visibility=if hide{Visibility::Hidden}else{Visibility::Inherited};
    }
}

fn character_is_moving(life:&Life,profile:usize)->bool{
    if profile==life.avatar.character{life.game.phase==game::Phase::Walking}
    else{life.society.residents.iter().find(|r|r.profile_index==profile).is_some_and(|r|r.action.starts_with("Paseando")&&!r.path.is_empty())}
}

fn animate(
    life:Res<Life>,shared:Res<Materials>,mut materials:ResMut<Assets<StandardMaterial>>,
    mut roots:Query<(&CharacterRoot,&mut Transform)>,
    mut players:Query<(&mut AnimationPlayer,&mut CharacterAnimation)>,
    mut nodes:Query<(&NodeId,&mut Transform,&mut Visibility),Without<CharacterRoot>>,
    mut point_lights:Query<&mut PointLight>,
){
    for(root,mut transform)in &mut roots{
        let spec=character_catalog::character(root.profile_index);
        let frame=if root.profile_index==life.avatar.character{Some(&life.game.frame)}else{life.society.residents.iter().find(|r|r.profile_index==root.profile_index).map(|r|&r.frame)};
        let Some(frame)=frame else{continue;};
        transform.translation=Vec3::new(frame.position.x as f32,frame.position.y as f32+spec.vertical_offset,frame.position.z as f32);
        let pose=Quat::from_euler(EulerRot::XYZ,frame.rotation[0]as f32,frame.rotation[1]as f32,frame.rotation[2]as f32);
        let authored_forward=Quat::from_rotation_y(spec.forward_yaw)*Quat::from_rotation_x(spec.rotation_x);
        transform.rotation=pose*authored_forward;
        transform.scale=Vec3::splat(spec.scale);
    }
    for(mut player,mut controller)in &mut players{
        let moving=character_is_moving(&life,controller.profile_index);
        let spec=character_catalog::character(controller.profile_index);
        let authored_walk=moving&&controller.walk.is_some();
        let procedural=moving&&controller.walk.is_none()&&spec.procedural_walk;
        let locomoting=authored_walk||procedural;
        if locomoting!=controller.walking{
            controller.walking=locomoting;
            player.stop_all();
            if authored_walk{
                if let Some(node)=controller.walk{player.play(node).repeat();}
            }else if !moving{
                if let Some(node)=controller.idle.or(controller.walk){player.play(node).repeat();}
            }
        }else if !locomoting&&player.all_finished(){
            if let Some(node)=controller.idle.or(controller.walk){player.play(node).repeat();}
        }
    }
    for(id,mut t,mut v)in &mut nodes{
        match id.0.as_str(){
            "drops"=>*v=if life.game.frame.drops||life.poltergeist.active(Task::Shower){Visibility::Inherited}else{Visibility::Hidden},
            s if s.starts_with("drop-")=>{let i=s[5..].parse::<usize>().unwrap();t.translation.y=life.game.drops_y[i]as f32;},
            s if s.starts_with("specter-root-")=>{
                let index=s[13..].parse::<usize>().unwrap_or(0);
                *v=if life.haunting.specter_visible(index,life.game.game_time){Visibility::Inherited}else{Visibility::Hidden};
                let p=life.haunting.specter_position(index,life.game.game_time);
                t.translation=Vec3::from_array(p);
                t.rotation=Quat::from_rotation_y(((life.game.game_time*0.22)as f32+index as f32*1.7).sin()*0.24);
            },
            _=>{}
        }
    }
    let haunting_light=life.haunting.level>=9&&life.haunting.event_remaining>0.0;
    for mut light in &mut point_lights{
        let wave=((life.game.game_time*31.0).sin()*0.5+0.5)as f32;
        let factor=if haunting_light{0.18+wave*0.82}else{1.0};
        light.intensity=1050.0*factor;
        light.color=if haunting_light&&life.haunting.level>=11{rgb(0xc9e8ee)}else{rgb(0xffe6a8)};
    }
    let tv_active=life.game.frame.watching||life.poltergeist.active(Task::Tv);
    if let Some(mat)=shared.0.get("tv-screen").and_then(|h|materials.get_mut(h)){mat.base_color=rgb(if tv_active{0x79a7d3}else{0x17212b});mat.emissive=Color::hsl(life.game.frame.screen_hue as f32*360.0,0.4,if tv_active{0.4}else{0.03}).to_linear();}
}

fn procedural_gait(life:Res<Life>,mut bones:Query<(&ProceduralBone,&mut Transform)>){
    for(bone,mut transform)in &mut bones{
        if !character_is_moving(&life,bone.profile_index){continue;}
        let frame=if bone.profile_index==life.avatar.character{&life.game.frame}else{
            let Some(resident)=life.society.residents.iter().find(|r|r.profile_index==bone.profile_index)else{continue;};
            &resident.frame
        };
        let(left,index)=match bone.kind{
            ProceduralBoneKind::UpperLegL|ProceduralBoneKind::LowerLegL|ProceduralBoneKind::UpperArmL|ProceduralBoneKind::LowerArmL=>(true,0),
            _=>(false,1),
        };
        let leg=frame.legs[index]as f32;
        let knee=frame.knees[index]as f32;
        let arm=frame.arms[index]as f32;
        let elbow=frame.elbows[index]as f32;
        transform.rotation=match bone.kind{
            ProceduralBoneKind::UpperLegL|ProceduralBoneKind::UpperLegR=>bone.rest*Quat::from_rotation_x(leg),
            ProceduralBoneKind::LowerLegL|ProceduralBoneKind::LowerLegR=>bone.rest*Quat::from_rotation_x(-knee*0.72),
            ProceduralBoneKind::UpperArmL|ProceduralBoneKind::UpperArmR=>{
                let down=Quat::from_rotation_z(if left{-1.15}else{1.15});
                bone.rest*down*Quat::from_rotation_x(arm)
            },
            ProceduralBoneKind::LowerArmL|ProceduralBoneKind::LowerArmR=>bone.rest*Quat::from_rotation_x(-elbow*0.55),
        };
    }
}
