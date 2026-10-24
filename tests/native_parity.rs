//! The original navigation is preserved; autonomous choices now belong to MSPA.
//! UI controls and the physical navigation reference stay independently covered.
use bevy_egui::egui;
use mens_life::{
    game::{Game, Navigation, Task},
    geometry,
    scene::{self, Avatar},
    ui,
};
use serde_json::Value;

#[test]
fn native_meshes_preserve_reference_navigation() {
    let reference: Value =
        serde_json::from_str(include_str!("reference/native-replay.json")).unwrap();
    let random: Vec<f64> = serde_json::from_value(reference["random"].clone()).unwrap();
    let spec = scene::build(&Avatar::default(), &random);
    let navigation = Navigation::new(geometry::navigation_bounds(&spec));
    for (index, free) in navigation.free.iter().enumerate() {
        assert_eq!(Some(*free), reference["free"][index].as_bool(), "cell {index}");
    }
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
