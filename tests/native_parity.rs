//! Regression data captured from the original project, independent of the Rust
//! implementation. Includes all five activities and interruption/pause cases.
use bevy_egui::egui;
use mens_life::{
    game::{Game, Navigation, Task, Vec3},
    geometry,
    scene::{self, Avatar},
    ui,
};
use serde_json::{Value, json};

fn snapshot(g: &Game) -> Value {
    json!({"values":g.values,"path":g.path,"task":g.task.map(Task::key).unwrap_or(""),"phase":g.phase,"elapsed":g.elapsed,"decisionIn":g.decision_in,"uiTime":g.ui_time,"gameTime":g.game_time,"walkTime":g.walk_time,"action":g.action,"position":g.frame.position,"rotation":g.frame.rotation,"legs":g.frame.legs,"arms":g.frame.arms,"plumb":g.frame.plumb,"drops":g.frame.drops,"dropsY":g.drops_y,"watching":g.frame.watching,"screenHue":g.frame.screen_hue,"displayedValues":g.displayed_values,"displayedClock":g.displayed_clock,"needsVisible":g.needs_visible})
}
fn close(a: &Value, b: &Value, path: &str) {
    match b {
        Value::Number(n) => {
            let expected = n.as_f64().unwrap();
            let actual = a.as_f64().unwrap();
            assert!(
                (actual - expected).abs() <= 1e-9,
                "{path}: native {actual}, original {expected}"
            );
        }
        Value::Array(values) => {
            assert_eq!(a.as_array().unwrap().len(), values.len(), "{path}");
            for (i, b) in values.iter().enumerate() {
                close(&a[i], b, &format!("{path}[{i}]"));
            }
        }
        Value::Object(values) => {
            assert_eq!(a.as_object().unwrap().len(), values.len(), "{path}");
            for (k, b) in values {
                close(&a[k], b, &format!("{path}.{k}"));
            }
        }
        _ => assert_eq!(a, b, "{path}"),
    }
}
#[test]
fn native_meshes_preserve_navigation_and_reference_gameplay() {
    let reference: Value =
        serde_json::from_str(include_str!("reference/native-replay.json")).unwrap();
    let random: Vec<f64> = serde_json::from_value(reference["random"].clone()).unwrap();
    let spec = scene::build(&Avatar::default(), &random);
    let bounds = geometry::navigation_bounds(&spec);
    let mut game = Game::new(Navigation::new(bounds), spec.drops_y);
    let differences: Vec<_> = game
        .navigation
        .free
        .iter()
        .enumerate()
        .filter(|(i, free)| Some(**free) != reference["free"][*i].as_bool())
        .map(|(i, _)| (i, Navigation::point(i)))
        .collect();
    assert!(
        differences.is_empty(),
        "Native navigation differs at {} cells: {:?}",
        differences.len(),
        differences.iter().take(12).collect::<Vec<_>>()
    );
    let checkpoints = reference["checkpoints"].as_array().unwrap();
    let mut cp = 0;
    let mut index = 0;
    for op in reference["ops"].as_array().unwrap() {
        let count = op["count"].as_u64().unwrap_or(1);
        for frame in 0..count {
            match op["kind"].as_str().unwrap() {
                "command" => game.command(Task::parse(op["task"].as_str().unwrap()).unwrap()),
                "walk" => {
                    game.walk_to(serde_json::from_value::<Vec3>(op["point"].clone()).unwrap())
                }
                "paused" => game.paused = op["value"].as_bool().unwrap(),
                "creator" => game.creator = op["value"].as_bool().unwrap(),
                "values" => game.values = serde_json::from_value(op["value"].clone()).unwrap(),
                "frames" => {
                    let dt = op["dt"].as_f64().unwrap();
                    game.tick(dt, op["now"].as_f64().unwrap() + frame as f64 * dt * 1000.0);
                }
                _ => panic!("Unknown reference event"),
            }
            if cp < checkpoints.len() && checkpoints[cp]["index"].as_u64() == Some(index) {
                close(
                    &snapshot(&game),
                    &checkpoints[cp]["state"],
                    &format!("operation {index}"),
                );
                cp += 1;
            }
            index += 1;
        }
    }
    assert_eq!(index, 12065);
    assert_eq!(cp, checkpoints.len());
}
fn frame(ctx: &egui::Context, game: &mut Game, avatar: &mut Avatar, events: Vec<egui::Event>) {
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1280.0, 800.0),
        )),
        events,
        ..Default::default()
    };
    let _ = ctx.run(input, |ctx| {
        ui::draw(ctx, game, avatar);
    });
}
fn click(ctx: &egui::Context, game: &mut Game, avatar: &mut Avatar, x: f32, y: f32) {
    let pos = egui::pos2(x, y);
    frame(ctx, game, avatar, vec![egui::Event::PointerMoved(pos)]);
    frame(
        ctx,
        game,
        avatar,
        vec![
            egui::Event::PointerMoved(pos),
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: Default::default(),
            },
        ],
    );
    frame(
        ctx,
        game,
        avatar,
        vec![egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: Default::default(),
        }],
    );
    frame(ctx, game, avatar, vec![]);
}
#[test]
fn native_controls_enter_pause_command_and_edit_character() {
    let spec = scene::build(&Avatar::default(), &[0.5; 90]);
    let mut game = Game::new(
        Navigation::new(geometry::navigation_bounds(&spec)),
        spec.drops_y,
    );
    let mut avatar = Avatar::default();
    let ctx = egui::Context::default();
    ui::configure(&ctx);
    for _ in 0..3 {
        frame(&ctx, &mut game, &mut avatar, vec![]);
    }
    click(&ctx, &mut game, &mut avatar, 640.0, 545.0);
    assert!(!game.creator, "Enter must dismiss native creator");
    for (task, x) in [
        (Task::Eat, 860.0),
        (Task::Tv, 952.0),
        (Task::Bed, 1046.0),
        (Task::Shower, 1130.0),
        (Task::Toilet, 1214.0),
    ] {
        click(&ctx, &mut game, &mut avatar, x, 735.0);
        assert_eq!(
            game.task,
            Some(task),
            "Native action button {task:?}; action={}, creator={}, paused={}",
            game.action,
            game.creator,
            game.paused
        );
    }
    click(&ctx, &mut game, &mut avatar, 580.0, 50.0);
    assert!(game.paused);
    let before = game.frame.position;
    game.tick(1.0, 1000.0);
    assert_eq!(game.frame.position, before);
    click(&ctx, &mut game, &mut avatar, 580.0, 50.0);
    assert!(!game.paused);
    click(&ctx, &mut game, &mut avatar, 985.0, 50.0);
    assert!(game.creator);
    click(&ctx, &mut game, &mut avatar, 710.0, 330.0);
    frame(
        &ctx,
        &mut game,
        &mut avatar,
        vec![egui::Event::Text(" Ñ".into())],
    );
    assert!(avatar.name.contains('Ñ'));
}
