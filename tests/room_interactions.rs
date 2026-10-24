use mens_life::{
    game::{Game,Navigation,Phase,Task}, geometry,
    psychology::SocialKind, scene::{self,Avatar}, society::Society,
};

fn furnished_game() -> Game {
    let scene=scene::build(&Avatar::default(),&[0.5;90]);
    let mut game=Game::new(Navigation::new(geometry::navigation_bounds(&scene)),scene.drops_y);
    game.creator=false;
    game
}
fn reach_using(game: &mut Game, task: Task) {
    game.command(task);
    for i in 0..900 {
        game.tick(0.05,i as f64*50.0);
        assert!(!game.navigation.blocked(game.ground_position.x,game.ground_position.z));
        if game.phase==Phase::Using && game.task==Some(task) { return; }
    }
    panic!("{task:?} never reached: {} at {:?}",game.action,game.ground_position);
}

#[test]
fn bathroom_interruption_uses_the_walkable_exit_instead_of_the_seated_pose() {
    let mut game=furnished_game();
    for fixture in [Task::Toilet,Task::Shower] {
        reach_using(&mut game,fixture);
        for i in 0..40 { game.tick(0.05,i as f64*50.0); }
        assert_ne!(game.frame.position,game.ground_position);
        reach_using(&mut game,Task::Tv);
        assert_eq!(game.task,Some(Task::Tv));
    }
}

#[test]
fn sleeping_is_face_up_on_the_mattress_and_leaves_a_free_exit() {
    let mut game=furnished_game();
    reach_using(&mut game,Task::Bed);
    for i in 0..30 { game.tick(0.05,i as f64*50.0); }
    let rotation=bevy::math::DQuat::from_euler(bevy::math::EulerRot::XYZ,
        game.frame.rotation[0],game.frame.rotation[1],game.frame.rotation[2]);
    let face=rotation*bevy::math::DVec3::Z;
    assert!(face.y>0.99,"must face up instead of into the bed");
    let head=bevy::math::DVec3::from_array(game.frame.position.array())
        +rotation*bevy::math::DVec3::new(0.0,1.75,0.0);
    assert!((head.z-3.8).abs()<0.25,"head must be over the pillow");
    reach_using(&mut game,Task::Eat);
}

#[test]
fn a_real_conversation_updates_both_characters_and_freezes_with_pause() {
    let mut game=furnished_game();
    let mut society=Society::new(&game.navigation);
    society.request(2,SocialKind::Chat,&mut game);
    for i in 0..900 {
        game.tick(0.05,i as f64*50.0);
        society.tick(&mut game,0.05);
        if society.residents[0].relationship.familiarity>0.0 { break; }
    }
    assert!(society.residents[0].relationship.familiarity>0.0,"Mara remembers the encounter");
    assert!(game.psychology.relationship(2).familiarity>0.0,"player also remembers Mara");
    game.paused=true;
    let positions: Vec<_>=society.residents.iter().map(|r| r.frame.position).collect();
    let memories=society.residents[0].psychology.state.memories;
    for _ in 0..100 { society.tick(&mut game,0.05); }
    assert_eq!(positions,society.residents.iter().map(|r| r.frame.position).collect::<Vec<_>>());
    assert_eq!(memories,society.residents[0].psychology.state.memories);
}
