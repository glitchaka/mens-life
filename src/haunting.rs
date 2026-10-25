//! Idle supernatural progression. It escalates from ambiguous household noises to visible specters.
//! Character reactions are shaped by the personality state produced by MSPA.
use crate::{
    character_catalog::{self, Voice},
    game::{Game, Phase},
    psychology::{Expression, Psychology, Reaction},
    scene::SceneSpec,
    society::Society,
};
use serde_json::json;

#[derive(Clone, Copy, Debug)]
pub struct HauntTier {
    pub level: u8,
    pub name: &'static str,
    pub description: &'static str,
    pub cost: f64,
}

pub const TIERS: [HauntTier; 20] = [
    HauntTier { level: 1, name: "Crujidos", description: "La casa cruje cuando nadie se mueve.", cost: 22.0 },
    HauntTier { level: 2, name: "Golpes lejanos", description: "Golpes secos aparecen dentro de los muros.", cost: 34.0 },
    HauntTier { level: 3, name: "Pasos sin dueño", description: "Se oyen pasos en habitaciones vacías.", cost: 48.0 },
    HauntTier { level: 4, name: "Objetos desplazados", description: "Pequeños objetos dejan de estar donde estaban.", cost: 64.0 },
    HauntTier { level: 5, name: "Susurros", description: "Surgen susurros demasiado bajos para entenderlos.", cost: 82.0 },
    HauntTier { level: 6, name: "Voces lejanas", description: "Las voces parecen venir de otra habitación.", cost: 102.0 },
    HauntTier { level: 7, name: "Voces que responden", description: "A veces una voz parece contestar a lo que se dijo.", cost: 125.0 },
    HauntTier { level: 8, name: "Sombras periféricas", description: "Algo cruza el borde de la visión y desaparece al mirar.", cost: 151.0 },
    HauntTier { level: 9, name: "Luces inestables", description: "Las luces titilan sin una causa eléctrica clara.", cost: 180.0 },
    HauntTier { level: 10, name: "Puertas inquietas", description: "Puertas y cajones se mueven solos unos centímetros.", cost: 212.0 },
    HauntTier { level: 11, name: "Frío localizado", description: "Aparecen zonas heladas que duran apenas unos segundos.", cost: 248.0 },
    HauntTier { level: 12, name: "Reflejos imposibles", description: "Los reflejos muestran movimiento que no está en la habitación.", cost: 288.0 },
    HauntTier { level: 13, name: "Presencia", description: "La sensación de que alguien está detrás deja de ser ocasional.", cost: 332.0 },
    HauntTier { level: 14, name: "Siluetas fugaces", description: "Figuras humanas aparecen durante una fracción de segundo.", cost: 381.0 },
    HauntTier { level: 15, name: "Contacto invisible", description: "Algo roza ropa, pelo y hombros sin mostrar un cuerpo.", cost: 435.0 },
    HauntTier { level: 16, name: "Apariciones parciales", description: "Rostros, manos o torsos llegan a definirse por instantes.", cost: 495.0 },
    HauntTier { level: 17, name: "Espectro tenue", description: "Un espectro translúcido logra manifestarse brevemente.", cost: 562.0 },
    HauntTier { level: 18, name: "Espectros móviles", description: "Las apariciones pueden desplazarse por la propiedad.", cost: 636.0 },
    HauntTier { level: 19, name: "Interacción espectral", description: "Los espectros responden a personas y objetos cercanos.", cost: 718.0 },
    HauntTier { level: 20, name: "Manifestación completa", description: "Los espectros pueden permanecer visibles y actuar en el mundo.", cost: 808.0 },
];

#[derive(Clone, Debug)]
pub struct Haunting {
    pub level: u8,
    pub essence: f64,
    pub last_event: String,
    pub event_remaining: f64,
    event_in: f64,
    serial: u64,
}

impl Default for Haunting {
    fn default() -> Self {
        Self {
            level: 0,
            essence: 0.0,
            last_event: "La casa parece completamente normal.".into(),
            event_remaining: 0.0,
            event_in: 8.0,
            serial: 0,
        }
    }
}

impl Haunting {
    pub fn current(&self) -> Option<&'static HauntTier> {
        self.level.checked_sub(1).and_then(|index| TIERS.get(index as usize))
    }

    pub fn next(&self) -> Option<&'static HauntTier> {
        TIERS.get(self.level as usize)
    }

    pub fn progress(&self) -> f32 {
        self.next().map_or(1.0, |tier| (self.essence / tier.cost).clamp(0.0, 1.0) as f32)
    }

    pub fn tick(&mut self, game: &mut Game, society: &mut Society, selected_profile: usize, dt: f64) {
        if game.creator || game.paused { return; }
        let dt = if dt.is_finite() { dt.clamp(0.0, 0.05) } else { 0.0 };
        self.event_remaining = (self.event_remaining - dt).max(0.0);

        // Passive idle income. Household activity and conversations increase the rate,
        // so the system progresses even if the player simply watches the simulation.
        let activity = if game.phase == Phase::Using { 0.42 } else if game.phase == Phase::Walking { 0.12 } else { 0.0 };
        let social = if game.talking { 0.15 } else { 0.0 };
        let rate = 0.72 + self.level as f64 * 0.055 + activity + social;
        self.essence += dt * rate;

        while let Some(next) = self.next() {
            if self.essence < next.cost { break; }
            self.essence -= next.cost;
            self.level = next.level;
            self.last_event = format!("Desbloqueado: {} — {}", next.name, next.description);
            self.event_remaining = 8.0;
            self.event_in = 1.2;
        }

        if self.level == 0 { return; }
        self.event_in -= dt;
        if self.event_in > 0.0 { return; }
        self.trigger(game, society, selected_profile);
        let base = (17.0 - self.level as f64 * 0.48).max(5.2);
        self.event_in = base + (self.serial % 5) as f64 * 0.85;
    }

    fn trigger(&mut self, game: &mut Game, society: &mut Society, selected_profile: usize) {
        self.serial = self.serial.wrapping_add(1);
        let tier = self.current().expect("haunting tier");
        self.last_event = event_text(self.level, self.serial).into();
        self.event_remaining = 7.0;

        let (expression, line) = reaction(selected_profile, &game.psychology, self.level, &self.last_event);
        game.reaction = Reaction { expression, words: line, remaining: 6.5 };

        for resident in &mut society.residents {
            let (expression, line) = reaction(resident.profile_index, &resident.psychology, self.level, &self.last_event);
            resident.reaction = Reaction { expression, words: line, remaining: 6.5 };
            resident.action = if self.level < 8 { "Atenta a un ruido extraño" } else if self.level < 17 { "Buscando qué acaba de ver" } else { "Mirando una aparición" }.into();
        }

        if self.level >= 17 {
            society.status = format!("Actividad espectral: {}", tier.name);
        }
    }

    pub fn specter_visible(&self, index: usize, time: f64) -> bool {
        if self.level < 17 { return false; }
        let count = ((self.level - 16) as usize).min(3);
        if index >= count { return false; }
        if self.level >= 20 { return true; }
        let phase = time * (1.25 + index as f64 * 0.31) + index as f64 * 1.7;
        phase.sin() > if self.level == 17 { 0.55 } else { -0.15 }
    }

    pub fn specter_position(&self, index: usize, time: f64) -> [f32; 3] {
        let anchors = [[-1.9, 0.0, 1.5], [2.9, 0.0, -1.9], [4.7, 0.0, 3.7]];
        let a = anchors[index.min(anchors.len() - 1)];
        let drift = (time * (0.31 + index as f64 * 0.07) + index as f64).sin() as f32;
        [a[0] + drift * 0.32, a[1] + (time * 1.7 + index as f64).sin() as f32 * 0.035, a[2] + drift * 0.18]
    }
}

fn event_text(level: u8, serial: u64) -> &'static str {
    let alternate = serial % 3;
    match level {
        1 => ["La madera cruje en una habitación vacía.", "Un crujido recorre el pasillo.", "El suelo cruje sin que nadie pase."][alternate as usize],
        2 => ["Tres golpes secos suenan dentro de un muro.", "Algo golpea una pared desde el otro lado.", "Un golpe aislado hace vibrar la casa."][alternate as usize],
        3 => ["Se oyen pasos donde no hay nadie.", "Unos pasos recorren el pasillo y se detienen.", "Alguien parece caminar en la habitación contigua."][alternate as usize],
        4 => "Un objeto aparece unos centímetros lejos de donde estaba.",
        5 => "Un susurro apenas audible atraviesa la habitación.",
        6 => "Una voz lejana pronuncia algo imposible de entender.",
        7 => "Una voz contesta desde una habitación vacía.",
        8 => "Una sombra cruza el borde de la visión.",
        9 => "Las luces titilan dos veces y vuelven a la normalidad.",
        10 => "Una puerta se mueve sola y queda entreabierta.",
        11 => "El aire se vuelve helado en un punto de la habitación.",
        12 => "Un reflejo se mueve un instante después que la persona frente a él.",
        13 => "La sensación de que alguien está detrás se vuelve insoportablemente precisa.",
        14 => "Una silueta humana aparece y desaparece al girar la cabeza.",
        15 => "Algo invisible roza un hombro al pasar.",
        16 => "Un rostro pálido toma forma durante un segundo y se disuelve.",
        17 => "Una figura translúcida se manifiesta dentro de la propiedad.",
        18 => "Un espectro atraviesa lentamente una habitación.",
        19 => "La aparición se detiene y parece observar a quienes están cerca.",
        _ => "Un espectro se manifiesta por completo y permanece en el mundo.",
    }
}

fn reaction(profile: usize, psychology: &Psychology, level: u8, event: &str) -> (Expression, String) {
    let state = psychology.state;
    let threat = state.derived_traits[3];
    let exploration = state.derived_traits[5];
    let stability = state.personality[1];
    let expression = if level >= 17 || threat > 0.68 { Expression::Afraid }
        else if stability < 0.48 { Expression::Anxious }
        else if exploration > 0.72 && level < 10 { Expression::Calm }
        else { Expression::Anxious };
    let voice = character_catalog::character(profile).voice;
    let stage = ((level.saturating_sub(1)) / 4).min(4);
    let line = match (voice, stage) {
        (Voice::SharpSchoolgirl, 0) => "¿Quién mierda está haciendo ese ruido? Si es una broma, no tiene gracia.",
        (Voice::SharpSchoolgirl, 1) => "Vale, eso no fue la casa asentándose. ¿Quién está hablando?",
        (Voice::SharpSchoolgirl, 2) => "No me jodas... yo también vi eso.",
        (Voice::SharpSchoolgirl, 3) => "Perfecto. Ahora resulta que la casa también toca a la gente.",
        (Voice::SharpSchoolgirl, _) => "Eso es un espectro. Y está ahí de verdad. Genial.",
        (Voice::SweetSchoolgirl, 0) => "¿Escuchaste eso? Dime que sí, por favor.",
        (Voice::SweetSchoolgirl, 1) => "Esa voz sonó demasiado cerca... no quiero estar sola ahora.",
        (Voice::SweetSchoolgirl, 2) => "Acabo de ver algo. Fue una sombra, ¿verdad?",
        (Voice::SweetSchoolgirl, 3) => "Sentí que alguien me tocó... pero aquí no había nadie.",
        (Voice::SweetSchoolgirl, _) => "Puedo verlo. De verdad puedo verlo...",
        (Voice::Reserved, 0) => "Eso sonó dentro de la casa. Voy a recordar de dónde vino.",
        (Voice::Reserved, 1) => "No encuentro una explicación normal para esa voz.",
        (Voice::Reserved, 2) => "Lo vi también. No fue un reflejo.",
        (Voice::Reserved, 3) => "Esto ya no parece una serie de coincidencias.",
        (Voice::Reserved, _) => "La aparición es estable. Está ocupando un lugar real frente a nosotros.",
        (Voice::Excitable, 0) => "¡Eso sonó aquí mismo! Espera... quiero saber qué fue.",
        (Voice::Excitable, 1) => "¡La voz respondió! Eso es imposible... y un poco increíble.",
        (Voice::Excitable, 2) => "¡Lo vi! No me digas que fui la única.",
        (Voice::Excitable, 3) => "Vale, curiosidad satisfecha. Esto ya da bastante miedo.",
        (Voice::Excitable, _) => "¡Hay alguien ahí! Bueno... alguien que probablemente está muerto.",
        (Voice::Playful, 0) => "Muy gracioso, casa. ¿Cuál es el siguiente truco?",
        (Voice::Playful, 1) => "Si esa voz quiere conversar, podría presentarse primero.",
        (Voice::Playful, 2) => "Eso definitivamente tenía forma de persona.",
        (Voice::Playful, 3) => "Ya no estoy segura de querer seguir provocándolo.",
        (Voice::Playful, _) => "Bien. Oficialmente tenemos fantasmas.",
        (Voice::Gentle, 0) => "Hay algo raro aquí. No parece peligroso todavía.",
        (Voice::Gentle, 1) => "Escuché una voz. Intentemos mantener la calma.",
        (Voice::Gentle, 2) => "Vi una figura por un instante. No creo haberlo imaginado.",
        (Voice::Gentle, 3) => "Lo que sea que ocurre, está consiguiendo acercarse más.",
        (Voice::Gentle, _) => "Puedo ver la aparición con claridad. Ya no podemos explicarlo como otra cosa.",
    };
    let mut words = line.to_string();
    if state.confidence < 0.35 && level < 13 {
        words.push_str(" Tal vez estoy interpretando demasiado lo que pasó.");
    } else if level >= 13 && event.len() < 90 {
        words.push(' ');
        words.push_str(event);
    }
    (expression, words)
}

pub fn append_visuals(scene: &mut SceneSpec) {
    scene.materials["specter"] = json!({
        "type":"standard","color":0xb8e7df,"roughness":0.18,
        "transparent":true,"opacity":0.28,"emissive":0x6ba99f
    });
    let anchors = [[-1.9, 0.0, 1.5], [2.9, 0.0, -1.9], [4.7, 0.0, 3.7]];
    for (index, anchor) in anchors.into_iter().enumerate() {
        let root = format!("specter-root-{index}");
        scene.group(&root, anchor, None);
        scene.last()["visible"] = json!(false);

        scene.mesh("capsule", &[0.23, 0.92, 6.0, 12.0], [0.0, 0.88, 0.0], 0xb8e7df, 0.18, "", [false, false]);
        scene.last()["id"] = json!(format!("specter-body-{index}"));
        scene.last()["parent"] = json!(root.clone());
        scene.last()["material_ref"] = json!("specter");

        scene.mesh("sphere", &[0.29, 12.0, 8.0], [0.0, 1.62, 0.0], 0xc9f1ea, 0.16, "", [false, false]);
        scene.last()["id"] = json!(format!("specter-head-{index}"));
        scene.last()["parent"] = json!(root);
        scene.last()["material_ref"] = json!("specter");
    }
}
