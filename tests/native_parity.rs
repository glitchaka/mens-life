//! Native controls and collision-safe access to the furnished room.
use bevy_egui::egui;
use mens_life::{
    game::{Game, Navigation, Task},
    geometry,
    scene::{self, Avatar},
    ui,
};

#[test]
fn all_furniture_has_a_reachable_collision_safe_approach() {
    let spec = scene::build(&Avatar::default(), &[0.5;90]);
    let navigation = Navigation::new(geometry::navigation_bounds(&spec));
    let origin = mens_life::game::Vec3::new(0.0,0.0,2.0);
    for task in [Task::Bed,Task::Tv,Task::Eat,Task::Shower,Task::Toilet] {
        let mut previous=origin;
        let path=navigation.route(origin,task.spec().approach).expect("furniture is reachable");
        for next in path {
            assert!(navigation.segment_free(previous,next),"{task:?} cuts a corner");
            previous=next;
        }
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
