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
    if capture.path.is_none(){let dt=time.delta_secs_f64();life.game.tick(dt,time.elapsed_secs_f64()*1000.0);let Life{game,society,..}=&mut *life;society.tick(game,dt);}
}
fn perceive_world(mut life:ResMut<Life>,mut awareness:Local<perception::WorldAwareness>){let Life{game,society,..}=&mut *life;perception::observe(game,society,&mut awareness);}

fn tag_cutaway_parts(mut commands:Commands,names:Query<(Entity,&Name),Added<Name>>){
    for(entity,name)in &names{
        let s=name.as_str();
        let side=if s.starts_with("CUTAWAY_ROOF__"){Some(CutawaySide::Roof)}else if s.starts_with("CUTAWAY_EAST__"){Some(CutawaySide::East)}else if s.starts_with("CUTAWAY_WEST__"){Some(CutawaySide::West)}else if s.starts_with("CUTAWAY_NORTH__"){Some(CutawaySide::North)}else if s.starts_with("CUTAWAY_SOUTH__"){Some(CutawaySide::South)}else{None};
        if let Some(side)=side{commands.entity(entity).insert(CutawayPart(side));}
    }
}

fn find_character_root(mut entity:Entity,parents:&Query<&ChildOf>,roots:&Query<&CharacterRoot>)->Option<usize>{
    loop{if let Ok(root)=roots.get(entity){return Some(root.profile_index);}entity=parents.get(entity).ok()?.parent();}
}

fn setup_character_animations(
    mut commands:Commands,asset_server:Res<AssetServer>,mut graphs:ResMut<Assets<AnimationGraph>>,
    parents:Query<&ChildOf>,roots:Query<&CharacterRoot>,mut players:Query<(Entity,&mut AnimationPlayer),Added<AnimationPlayer>>,
){
    for(entity,mut player)in &mut players{
        let Some(profile)=find_character_root(entity,&parents,&roots)else{continue;};
        let spec=character_catalog::character(profile);
        if spec.idle_animation.is_none()&&spec.walk_animation.is_none(){continue;}
        let mut graph=AnimationGraph::new();
        let idle=spec.idle_animation.map(|i|graph.add_clip(asset_server.load(GltfAssetLabel::Animation(i).from_asset(spec.asset)),1.0,graph.root));
        let walk=spec.walk_animation.map(|i|graph.add_clip(asset_server.load(GltfAssetLabel::Animation(i).from_asset(spec.asset)),1.0,graph.root));
        let handle=graphs.add(graph);
        commands.entity(entity).insert((AnimationGraphHandle(handle),CharacterAnimation{profile_index:profile,idle,walk,walking:false}));
        if let Some(node)=idle{player.play(node).repeat();}
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
    let focus=Vec3::new(-2.5,0.7,0.0);
    let offset=Vec3::new(orbit.yaw.sin()*orbit.radius,orbit.height,orbit.yaw.cos()*orbit.radius);
    **camera=Transform::from_translation(focus+offset).looking_at(focus,Vec3::Y);

    let p=life.game.ground_position;
    let indoors=(-4.76..=-1.18).contains(&p.x)&&(-3.28..=3.34).contains(&p.z);
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

fn animate(
    life:Res<Life>,shared:Res<Materials>,mut materials:ResMut<Assets<StandardMaterial>>,
    mut roots:Query<(&CharacterRoot,&mut Transform)>,
    mut players:Query<(&mut AnimationPlayer,&mut CharacterAnimation)>,
    mut nodes:Query<(&NodeId,&mut Transform,&mut Visibility),Without<CharacterRoot>>,
){
    for(root,mut transform)in &mut roots{
        let spec=character_catalog::character(root.profile_index);
        let frame=if root.profile_index==life.avatar.character{Some(&life.game.frame)}else{life.society.residents.iter().find(|r|r.profile_index==root.profile_index).map(|r|&r.frame)};
        let Some(frame)=frame else{continue;};
        let moving=if root.profile_index==life.avatar.character{life.game.phase==game::Phase::Walking}else{life.society.residents.iter().find(|r|r.profile_index==root.profile_index).is_some_and(|r|r.action.starts_with("Paseando"))};
        let bob=if spec.walk_animation.is_none()&&moving{(life.game.game_time as f32*8.0).sin().abs()*0.015}else{0.0};
        transform.translation=Vec3::new(frame.position.x as f32,frame.position.y as f32+spec.vertical_offset+bob,frame.position.z as f32);
        let pose=Quat::from_euler(EulerRot::XYZ,frame.rotation[0]as f32,frame.rotation[1]as f32,frame.rotation[2]as f32);
        transform.rotation=pose*Quat::from_rotation_x(spec.rotation_x);
        transform.scale=Vec3::splat(spec.scale);
    }
    for(mut player,mut controller)in &mut players{
        let moving=if controller.profile_index==life.avatar.character{life.game.phase==game::Phase::Walking}else{life.society.residents.iter().find(|r|r.profile_index==controller.profile_index).is_some_and(|r|r.action.starts_with("Paseando"))};
        let wants_walk=moving&&controller.walk.is_some();
        if wants_walk!=controller.walking{
            controller.walking=wants_walk;player.stop_all();
            if let Some(node)=if wants_walk{controller.walk}else{controller.idle.or(controller.walk)}{player.play(node).repeat();}
        }else if player.all_finished(){if let Some(node)=if wants_walk{controller.walk}else{controller.idle.or(controller.walk)}{player.play(node).repeat();}}
    }
    for(id,mut t,mut v)in &mut nodes{match id.0.as_str(){"drops"=>*v=if life.game.frame.drops{Visibility::Inherited}else{Visibility::Hidden},s if s.starts_with("drop-")=>{let i=s[5..].parse::<usize>().unwrap();t.translation.y=life.game.drops_y[i]as f32;},_=>{}}}
    if let Some(mat)=shared.0.get("tv-screen").and_then(|h|materials.get_mut(h)){mat.base_color=rgb(if life.game.frame.watching{0x79a7d3}else{0x17212b});mat.emissive=Color::hsl(life.game.frame.screen_hue as f32*360.0,0.4,if life.game.frame.watching{0.4}else{0.03}).to_linear();}
}
