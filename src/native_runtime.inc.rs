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
    if capture.path.is_none() {
        let dt = time.delta_secs_f64();
        life.game.tick(dt, time.elapsed_secs_f64() * 1000.0);
        let Life { game, society, .. } = &mut *life;
        society.tick(game, dt);
    }
}

/// Propagate household events as anonymous environmental perceptions. Residents
/// observe the event itself; no proposition contains the protagonist/player as
/// its causal agent, so MSPA can react without acquiring meta knowledge.
fn perceive_world(
    mut life: ResMut<Life>,
    mut awareness: Local<perception::WorldAwareness>,
) {
    let Life { game, society, .. } = &mut *life;
    perception::observe(game, society, &mut awareness);
}

fn animate(
    life: Res<Life>,
    shared: Res<Materials>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut nodes: Query<(&NodeId, &mut Transform, &mut Visibility)>,
) {
    for (id, mut t, mut v) in &mut nodes {
        let (actor, part) = if let Some(rest) = id.0.strip_prefix("npc-") {
            let (actor, part) = rest.split_once('/').unwrap();
            (actor.parse::<u64>().unwrap(), part)
        } else {
            (1, id.0.as_str())
        };
        let f = if actor == 1 {
            &life.game.frame
        } else {
            &life
                .society
                .residents
                .iter()
                .find(|r| r.id == actor)
                .unwrap()
                .frame
        };
        match part {
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
            "knee-0" => t.rotation = Quat::from_rotation_x(f.knees[0] as f32),
            "knee-1" => t.rotation = Quat::from_rotation_x(f.knees[1] as f32),
            "arm-0" => {
                t.rotation = Quat::from_euler(EulerRot::XYZ, f.arms[0] as f32, 0.0, -0.06)
            }
            "arm-1" => {
                t.rotation = Quat::from_euler(EulerRot::XYZ, f.arms[1] as f32, 0.0, 0.06)
            }
            "forearm-0" => t.rotation = Quat::from_rotation_x(-f.elbows[0] as f32),
            "forearm-1" => t.rotation = Quat::from_rotation_x(-f.elbows[1] as f32),
            "spine" => {
                t.translation.y = 0.94 + f.breath as f32;
                t.rotation = Quat::from_euler(
                    EulerRot::XYZ,
                    f.spine[0] as f32,
                    f.spine[1] as f32,
                    f.spine[2] as f32,
                );
            }
            "head-bone" => {
                t.rotation = Quat::from_euler(
                    EulerRot::XYZ,
                    f.head[0] as f32,
                    f.head[1] as f32,
                    0.0,
                )
            }
            "eye-white-0" | "eye-white-1" => {
                let clock = (life.game.game_time * 0.8 + actor as f64).rem_euclid(4.8);
                let blink = if clock < 0.14 { 0.12 } else { 1.0 };
                t.scale.y = 0.044 * blink;
            }
            "eye-0" | "eye-1" => {
                let clock = (life.game.game_time * 0.8 + actor as f64).rem_euclid(4.8);
                let blink = if clock < 0.14 { 0.12 } else { 1.0 };
                t.scale.y = 0.033 * blink;
            }
            "pupil-0" | "pupil-1" => {
                let clock = (life.game.game_time * 0.8 + actor as f64).rem_euclid(4.8);
                let blink = if clock < 0.14 { 0.12 } else { 1.0 };
                t.scale.y = 0.016 * blink;
            }
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
    let f = &life.game.frame;
    if let Some(mat) = shared
        .0
        .get("tv-screen")
        .and_then(|h| materials.get_mut(h))
    {
        mat.base_color = rgb(if f.watching { 0x79a7d3 } else { 0x17212b });
        mat.emissive = Color::hsl(
            f.screen_hue as f32 * 360.0,
            0.4,
            if f.watching { 0.4 } else { 0.03 },
        )
        .to_linear();
    }
}
