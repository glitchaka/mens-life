//! Read-only MSPA inspector. It presents state; it never owns psychology logic.
use crate::{game::Game, psychology::{PsychState, Psychology}, society::Society};
use bevy_egui::egui::{self, Color32, RichText};

const BASE: [&str; 6] = [
    "Sociabilidad", "Estabilidad", "Empatía", "Persistencia", "Curiosidad", "Audacia",
];
const DERIVED: [&str; 6] = [
    "Rigidez de creencia", "Persistencia de objetivo", "Resistencia al cambio",
    "Sensibilidad a amenaza", "Sensibilidad social", "Exploración",
];
const EMOTIONS: [&str; 6] = ["Alegría", "Tristeza", "Miedo", "Ira", "Ansiedad", "Aversión"];

fn meter(ui: &mut egui::Ui, label: &str, value: f32) {
    ui.horizontal(|ui| {
        ui.set_min_width(330.0);
        ui.label(label);
        ui.add_space(4.0);
        ui.add(
            egui::ProgressBar::new(value.clamp(0.0, 1.0))
                .desired_width(155.0)
                .show_percentage(),
        );
    });
}

fn snapshot(ui: &mut egui::Ui, state: PsychState, error: Option<&str>) {
    if let Some(error) = error {
        ui.colored_label(Color32::from_rgb(180, 70, 65), format!("MSPA: {error}"));
    }

    ui.label(RichText::new("Personalidad base").strong());
    for (label, value) in BASE.into_iter().zip(state.personality) {
        meter(ui, label, value);
    }

    ui.add_space(5.0);
    ui.label(RichText::new("Rasgos derivados MSPA").strong());
    for (label, value) in DERIVED.into_iter().zip(state.derived_traits) {
        meter(ui, label, value);
    }

    ui.add_space(5.0);
    ui.label(RichText::new("Emociones").strong());
    for (label, value) in EMOTIONS.into_iter().zip(state.emotions) {
        meter(ui, label, value);
    }

    ui.add_space(5.0);
    egui::Grid::new(ui.id().with("mspa-summary"))
        .num_columns(2)
        .spacing([14.0, 4.0])
        .show(ui, |ui| {
            ui.label("Valencia"); ui.monospace(format!("{:+.3}", state.valence)); ui.end_row();
            ui.label("Activación"); ui.monospace(format!("{:.3}", state.activation)); ui.end_row();
            ui.label("Confianza de decisión"); ui.monospace(format!("{:.3}", state.confidence)); ui.end_row();
            ui.label("Recuerdos"); ui.monospace(state.memories.to_string()); ui.end_row();
            ui.label("Creencias"); ui.monospace(state.beliefs.to_string()); ui.end_row();
            ui.label("Patrones aprendidos"); ui.monospace(state.learned_patterns.to_string()); ui.end_row();
        });
}

fn person(ui: &mut egui::Ui, name: &str, psychology: &Psychology, open: bool) {
    egui::CollapsingHeader::new(name)
        .default_open(open)
        .show(ui, |ui| snapshot(ui, psychology.state, psychology.error.as_deref()));
}

/// Shows the real MSPA snapshot for every living character. Nothing in this
/// window is synthesized from UI state.
pub fn draw(ctx: &egui::Context, game: &Game, society: &Society) {
    if game.creator { return; }
    egui::Window::new("MSPA · rasgos por personaje")
        .default_width(390.0)
        .default_pos([18.0, 104.0])
        .resizable(true)
        .collapsible(true)
        .show(ctx, |ui| {
            ui.small("Valores 0–1 salvo valencia/activación y contadores.");
            ui.separator();
            person(ui, "Alex", &game.psychology, true);
            for resident in &society.residents {
                ui.separator();
                person(ui, &resident.avatar.name, &resident.psychology, false);
            }
        });
}
