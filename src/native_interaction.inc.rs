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
    people: Query<(&Person, &Mesh3d, &GlobalTransform)>,
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

    let mut person_hit = None;
    let mut person_distance = f32::INFINITY;
    for (person, handle, transform) in &people {
        if person.0 == 1 {
            continue;
        }
        let Some(mesh) = meshes.get(&handle.0) else {
            continue;
        };
        let inverse = transform.affine().inverse();
        if let Some(t) = intersection(
            mesh,
            inverse.transform_point3(ray.origin),
            inverse.transform_vector3(*ray.direction),
        ) && t < person_distance
        {
            person_distance = t;
            person_hit = Some(person.0);
        }
    }

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
    if let Some(actor) = person_hit && person_distance < closest {
        life.society.selected = Some(actor);
        return;
    }
    if let Some(task) = hit {
        life.game.command(task);
        return;
    }
    if ray.direction.y.abs() > 1e-6 {
        let t = (-0.045 - ray.origin.y) / ray.direction.y;
        let p = ray.origin + *ray.direction * t;
        if t > 0.0 && p.x.abs() < 7.5 && p.z.abs() < 5.5 {
            life.game
                .walk_to(Point::new(p.x as f64, 0.0, p.z as f64));
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
