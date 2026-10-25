use crate::psychology::SocialKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Voice {
    Gentle,
    Excitable,
    SharpSchoolgirl,
    Reserved,
    Playful,
    SweetSchoolgirl,
}

#[derive(Clone, Copy, Debug)]
pub struct CharacterSpec {
    pub key: &'static str,
    pub name: &'static str,
    pub asset: &'static str,
    pub available: bool,
    pub scale: f32,
    pub rotation_x: f32,
    pub vertical_offset: f32,
    pub idle_animation: Option<usize>,
    pub walk_animation: Option<usize>,
    pub personality: [f32; 6],
    pub voice: Voice,
    pub outdoor_spawn: [f64; 3],
}

// Slot 0 keeps the personality identity reserved for the original standing-girl
// reference. It is intentionally unavailable until its actual 3D file is supplied;
// the rejected kneeling OBJ is never substituted for it.
pub const CHARACTERS: [CharacterSpec; 6] = [
    CharacterSpec { key: "original", name: "Aiko", asset: "characters/original_girl.glb", available: false, scale: 1.0, rotation_x: 0.0, vertical_offset: 0.0, idle_animation: None, walk_animation: None, personality: [0.70, 0.72, 0.80, 0.58, 0.74, 0.54], voice: Voice::Gentle, outdoor_spawn: [3.8, 0.0, 4.6] },
    CharacterSpec { key: "mia", name: "Mia", asset: "anime_girl_mia_ter_excited_preview.glb", available: true, scale: 1.02, rotation_x: std::f32::consts::FRAC_PI_2, vertical_offset: 0.0, idle_animation: Some(0), walk_animation: None, personality: [0.86, 0.54, 0.64, 0.46, 0.86, 0.82], voice: Voice::Excitable, outdoor_spawn: [5.0, 0.0, -3.6] },
    CharacterSpec { key: "kurumi", name: "Kurumi", asset: "kurumi_schoolgirl_-_bourin.glb", available: true, scale: 0.465, rotation_x: 0.0, vertical_offset: 0.0, idle_animation: Some(0), walk_animation: Some(9), personality: [0.78, 0.38, 0.34, 0.64, 0.58, 0.90], voice: Voice::SharpSchoolgirl, outdoor_spawn: [1.6, 0.0, -5.0] },
    CharacterSpec { key: "mai", name: "Mai", asset: "mai_maid_-bourin.glb", available: true, scale: 0.476, rotation_x: 0.0, vertical_offset: 0.0, idle_animation: Some(0), walk_animation: Some(9), personality: [0.56, 0.84, 0.84, 0.74, 0.54, 0.40], voice: Voice::Reserved, outdoor_spawn: [-5.2, 0.0, -4.5] },
    CharacterSpec { key: "outfits", name: "Rin", asset: "rigged_anime_girl_outfits__facial_expressions.glb", available: true, scale: 0.0147, rotation_x: 0.0, vertical_offset: 0.78, idle_animation: Some(0), walk_animation: None, personality: [0.66, 0.62, 0.56, 0.66, 0.78, 0.62], voice: Voice::Playful, outdoor_spawn: [-5.2, 0.0, 4.5] },
    CharacterSpec { key: "schoolgirl_sweet", name: "Yui", asset: "3d_anime_girl..glb", available: true, scale: 0.89, rotation_x: 0.0, vertical_offset: 0.0, idle_animation: None, walk_animation: None, personality: [0.86, 0.56, 0.88, 0.48, 0.72, 0.74], voice: Voice::SweetSchoolgirl, outdoor_spawn: [5.4, 0.0, 2.2] },
];

pub fn character(index: usize) -> &'static CharacterSpec {
    &CHARACTERS[index.min(CHARACTERS.len() - 1)]
}

pub fn available_characters() -> impl Iterator<Item = (usize, &'static CharacterSpec)> {
    CHARACTERS.iter().enumerate().filter(|(_, spec)| spec.available)
}

pub fn dialogue(index: usize, kind: SocialKind, turn: u8) -> &'static str {
    use SocialKind::{Chat, Greet, Joke};
    match (character(index).voice, kind, turn) {
        (Voice::SharpSchoolgirl, Greet, 0) => "¿Qué miras? ...Hola.",
        (Voice::SharpSchoolgirl, Chat, 0) => "Habla rápido, que me aburro fácil.",
        (Voice::SharpSchoolgirl, Joke, 0) => "Como sea malo el chiste, te lo voy a decir.",
        (Voice::SharpSchoolgirl, _, _) => "Tch... no estuvo tan mal. No te acostumbres.",
        (Voice::SweetSchoolgirl, Greet, 0) => "¡Hola! Me alegra que hayas venido.",
        (Voice::SweetSchoolgirl, Chat, 0) => "Cuéntame algo. Hoy tengo demasiada energía.",
        (Voice::SweetSchoolgirl, Joke, 0) => "A ver, hazme reír. Prometo intentarlo también.",
        (Voice::SweetSchoolgirl, _, _) => "Je... contigo se pasa rápido el rato.",
        (Voice::Excitable, Greet, 0) => "¡Hey! Justo estaba buscando compañía.",
        (Voice::Excitable, Chat, 0) => "Tengo mil cosas en la cabeza. ¿Por cuál empiezo?",
        (Voice::Excitable, Joke, 0) => "Dale, sorpréndeme.",
        (Voice::Excitable, _, _) => "Eso estuvo mejor de lo que esperaba.",
        (Voice::Reserved, Greet, 0) => "Hola. Qué bueno verte.",
        (Voice::Reserved, Chat, 0) => "Podemos hablar un rato, si quieres.",
        (Voice::Reserved, Joke, 0) => "No prometo reírme... pero inténtalo.",
        (Voice::Reserved, _, _) => "Gracias. Me hizo bien conversar.",
        (Voice::Playful, Greet, 0) => "Mira quién apareció. Hola.",
        (Voice::Playful, Chat, 0) => "¿Vienes con chisme o con conversación seria?",
        (Voice::Playful, Joke, 0) => "Tienes una oportunidad. No la desperdicies.",
        (Voice::Playful, _, _) => "Bien, te concedo esa.",
        (Voice::Gentle, Greet, 0) => "Hola. Me alegra verte por aquí.",
        (Voice::Gentle, Chat, 0) => "Quédate un rato. Podemos hablar tranquilos.",
        (Voice::Gentle, Joke, 0) => "A ver si consigues hacerme reír.",
        (Voice::Gentle, _, _) => "Fue agradable. Hablemos de nuevo luego.",
    }
}
