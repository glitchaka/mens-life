fn advance(time:Res<Time>,mut life:ResMut<Life>,capture:Res<Capture>,keys:Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space){life.game.paused=!life.game.paused;}
    if capture.path.is_none(){let dt=time.delta_secs_f64();life.game.tick(dt,time.elapsed_secs_f64()*1000.0);let Life{game,society,..}=&mut *life;society.tick(game,dt);}
}
fn perceive_world(mut life:ResMut<Life>,mut awareness:Local<perception::WorldAwareness>){let Life{game,society,..}=&mut *life;perception::observe(game,society,&mut awareness);}

fn animate(
    life:Res<Life>, shared:Res<Materials>, mut materials:ResMut<Assets<StandardMaterial>>,
    mut girls:Query<&mut Transform,With<Girl>>, mut nodes:Query<(&NodeId,&mut Transform,&mut Visibility),Without<Girl>>,
) {
    let f=&life.game.frame;
    for mut transform in &mut girls {
        transform.translation=Vec3::new(f.position.x as f32,f.position.y as f32,f.position.z as f32);
        transform.rotation=Quat::from_euler(EulerRot::XYZ,f.rotation[0] as f32,f.rotation[1] as f32,f.rotation[2] as f32);
    }
    for (id,mut t,mut v) in &mut nodes {
        match id.0.as_str() {
            "drops"=>*v=if f.drops{Visibility::Inherited}else{Visibility::Hidden},
            s if s.starts_with("drop-")=>{let i=s[5..].parse::<usize>().unwrap();t.translation.y=life.game.drops_y[i] as f32;},
            _=>{}
        }
    }
    if let Some(mat)=shared.0.get("tv-screen").and_then(|h|materials.get_mut(h)){
        mat.base_color=rgb(if f.watching{0x79a7d3}else{0x17212b});
        mat.emissive=Color::hsl(f.screen_hue as f32*360.0,0.4,if f.watching{0.4}else{0.03}).to_linear();
    }
}
