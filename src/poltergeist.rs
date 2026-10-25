//! Anonymous player/environment interactions. These effects never identify a player actor.
use crate::{
    character_catalog::{self, Voice},
    game::{Game, Task},
    haunting::Haunting,
    psychology::{Expression, Reaction},
    society::Society,
};

#[derive(Clone, Debug, Default)]
pub struct Poltergeist {
    active: Option<Task>,
    remaining: f64,
}

impl Poltergeist {
    pub fn tick(&mut self, dt: f64) {
        self.remaining = (self.remaining - dt.max(0.0)).max(0.0);
        if self.remaining <= 0.0 { self.active = None; }
    }

    pub fn active(&self, task: Task) -> bool {
        self.remaining > 0.0 && self.active == Some(task)
    }

    pub fn disturb(
        &mut self,
        task: Task,
        haunting: &mut Haunting,
        game: &mut Game,
        society: &mut Society,
        selected_profile: usize,
    ) {
        self.active = Some(task);
        self.remaining = match task {
            Task::Tv | Task::Shower => 5.0,
            _ => 2.8,
        };
        haunting.essence += 4.5 + haunting.level as f64 * 0.30;
        haunting.event_remaining = 6.0;
        haunting.last_event = event(task).into();

        // Existing MSPA environmental observation records an anonymous cue. No causal
        // character or meta-level player identity is supplied to the psychology engine.
        game.psychology.notice(task, 0.95);
        let (expression, words) = response(selected_profile, game.psychology.state.derived_traits[3], game.psychology.state.derived_traits[5], task);
        game.reaction = Reaction { expression, words: words.into(), remaining: 6.0 };

        for resident in &mut society.residents {
            resident.psychology.notice(task, 0.80);
            let (expression, words) = response(
                resident.profile_index,
                resident.psychology.state.derived_traits[3],
                resident.psychology.state.derived_traits[5],
                task,
            );
            resident.reaction = Reaction { expression, words: words.into(), remaining: 6.0 };
            resident.action = "Intentando entender qué pasó".into();
        }
        society.status = "Algo acaba de ocurrir dentro de la casa".into();
    }
}

fn event(task: Task) -> &'static str {
    match task {
        Task::Tv => "La televisión se enciende sola.",
        Task::Shower => "La ducha se abre sola aunque nadie entró al baño.",
        Task::Bed => "El colchón se hunde como si alguien acabara de sentarse.",
        Task::Toilet => "La cisterna suena aunque el baño está vacío.",
        Task::Eat => "La vajilla golpea la encimera sin que nadie la toque.",
    }
}

fn response(profile: usize, threat: f32, exploration: f32, task: Task) -> (Expression, &'static str) {
    use Voice::*;
    let expression = if threat > 0.66 { Expression::Afraid }
        else if exploration > 0.72 { Expression::Calm }
        else { Expression::Anxious };
    let voice = character_catalog::character(profile).voice;
    let line = match (voice, task) {
        (SharpSchoolgirl, Task::Tv) => "¿Quién mierda encendió la tele? Aquí no hay nadie.",
        (SharpSchoolgirl, Task::Shower) => "No me jodas. La ducha no se abre sola.",
        (SharpSchoolgirl, _) => "Vale, eso se movió solo. Que alguien me explique cómo.",
        (SweetSchoolgirl, Task::Tv) => "La tele se encendió sola... ¿también lo viste?",
        (SweetSchoolgirl, Task::Shower) => "Se abrió el agua. Por favor dime que no fui la única que lo oyó.",
        (SweetSchoolgirl, _) => "Eso acaba de moverse sin que nadie lo tocara...",
        (Reserved, Task::Tv) => "La televisión recibió una orden sin que nadie estuviera cerca.",
        (Reserved, Task::Shower) => "La llave de la ducha acaba de abrirse. Eso merece atención.",
        (Reserved, _) => "No veo una causa física inmediata para lo que acaba de pasar.",
        (Excitable, Task::Tv) => "¡La tele se encendió sola! Espera, quiero ver si vuelve a hacerlo.",
        (Excitable, Task::Shower) => "¡El agua! Nadie estaba ahí. Esto se está poniendo raro.",
        (Excitable, _) => "¡Eso se movió! Lo vi perfectamente.",
        (Playful, Task::Tv) => "Bueno, fantasma, al menos podrías elegir un canal bueno.",
        (Playful, Task::Shower) => "Si la casa quería ducharse, podría haber avisado.",
        (Playful, _) => "Muy gracioso. ¿Quién está jugando con las cosas?",
        (Gentle, Task::Tv) => "La televisión se encendió sola. Voy a mantener la calma.",
        (Gentle, Task::Shower) => "Se abrió la ducha sin que hubiera nadie. Eso no es normal.",
        (Gentle, _) => "Algo cambió aquí sin que nadie lo tocara.",
    };
    (expression, line)
}
