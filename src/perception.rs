//! Anonymous household perception for NPCs.
//! Residents perceive consequences in the environment; no event identifies the
//! protagonist or a meta-level "player" as its cause.
use crate::{
    game::{Game, Phase, Task},
    psychology::Reaction,
    society::Society,
};

#[derive(Debug, Default)]
pub struct WorldAwareness {
    last_using: Option<Task>,
}

pub fn observe(game: &Game, society: &mut Society, awareness: &mut WorldAwareness) {
    if game.creator || game.paused {
        return;
    }
    let current = (game.phase == Phase::Using).then_some(game.task).flatten();
    if current == awareness.last_using {
        return;
    }
    awareness.last_using = current;
    let Some(task) = current else { return; };

    let origin = task.spec().pose;
    let (radius, words) = match task {
        Task::Bed => (5.5, "Se oyó movimiento en uno de los dormitorios."),
        Task::Eat => (5.8, "Se oye actividad y huele a comida en la cocina."),
        Task::Shower => (6.5, "Se oye correr agua en el baño."),
        Task::Tv => (8.0, "La televisión acaba de encenderse."),
        Task::Toilet => (5.5, "Se oyó movimiento en el baño."),
    };

    for resident in &mut society.residents {
        let p = resident.frame.position;
        let distance = (p.x - origin.x).hypot(p.z - origin.z);
        if distance > radius {
            continue;
        }
        let salience = (1.0 - distance / radius).clamp(0.18, 1.0) as f32;
        resident.psychology.notice(task, salience);
        resident.reaction = Reaction {
            expression: resident.psychology.emotion(),
            words: words.into(),
            remaining: 5.0,
        };
        resident.action = "Atento a un cambio en la casa".into();
        // Orient toward the event without assigning an identity to its source.
        resident.frame.rotation[1] = (origin.x - p.x).atan2(origin.z - p.z);
    }
}
