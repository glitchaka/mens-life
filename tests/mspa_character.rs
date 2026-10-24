use mens_life::{
    game::{Game, Navigation, Phase, Task, Vec3},
    psychology::{ActivityEvent, Expression, Psychology},
};

fn game() -> Game {
    let mut game = Game::new(Navigation::new(vec![]), vec![]);
    game.creator = false;
    game
}

#[test]
fn mspa_selects_only_available_actions_and_learns_real_outcomes() {
    let mut brain = Psychology::new(42).unwrap();
    brain.advance(0.05, [95.0, 2.0, 95.0, 95.0, 95.0]);
    assert_eq!(brain.decide(&[(Task::Eat, 0.02, 0.0)], None), Some(Task::Eat));
    // A hungry character cannot invent food if the host offers no reachable food.
    assert_eq!(brain.decide(&[], None), None);
    let before = brain.state;
    brain.event(Task::Eat, ActivityEvent::Started);
    brain.event(Task::Eat, ActivityEvent::Completed);
    assert!(brain.error.is_none(), "{:?}", brain.error);
    assert!(brain.state.memories > before.memories);
    assert!(brain.state.learned_patterns > before.learned_patterns);
    assert!(brain.state.emotions[0] > before.emotions[0]);
}

#[test]
fn physical_hunger_drives_autonomy_words_and_emoticons_and_pause_freezes_mspa() {
    let mut game = game();
    game.values = [95.0, 2.0, 95.0, 95.0, 95.0];
    for i in 0..30 { game.tick(0.05, i as f64 * 50.0); }
    assert_eq!(game.task, Some(Task::Eat));
    assert_eq!(game.expression(), Expression::Hungry);
    assert_eq!(game.reaction.expression.emoticon(), ":P");
    assert!(!game.reaction.words.is_empty());
    assert!(game.psychology.state.memories > 0);
    game.paused = true;
    let state = game.psychology.state;
    let speech = game.reaction.words.clone();
    let remaining = game.reaction.remaining;
    for _ in 0..200 { game.tick(0.05, 10000.0); }
    assert_eq!(game.psychology.state, state);
    assert_eq!(game.reaction.words, speech);
    assert_eq!(game.reaction.remaining, remaining);
}

#[test]
fn completing_a_physical_activity_produces_relief_and_interruption_is_not_failure() {
    let mut game = game();
    game.values = [90.0, 40.0, 90.0, 90.0, 90.0];
    game.frame.position = Task::Eat.spec().approach;
    game.command(Task::Eat);
    for i in 0..205 { game.tick(0.05, i as f64 * 50.0); }
    assert_eq!(game.phase, Phase::Idle);
    assert!(game.values[1] > 75.0);
    assert!(game.reaction.words.contains("hambre"));
    assert!(game.psychology.state.learned_patterns > 0);
    assert!(game.psychology.state.emotions[0] > 0.0);
    let patterns = game.psychology.state.learned_patterns;
    game.command(Task::Bed);
    game.walk_to(Vec3::new(0.0, 0.0, 0.0));
    assert_eq!(game.task, None);
    assert_eq!(game.psychology.state.learned_patterns, patterns);
    assert_eq!(game.action, "Caminando");
}
