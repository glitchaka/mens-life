//! Read-only MSPA inspector. All displayed values come from the selected
//! character's runtime snapshot; this module does not derive psychology.
use crate::{game::Game, psychology::{PsychState, Psychology}, society::Society};
use bevy_egui::egui::{self, Color32, Id, RichText};

const BASE: [&str; 6] = [
    "Sociabilidad", "Estabilidad", "Empatía", "Persistencia", "Curiosidad", "Audacia",
];
const DERIVED: [&str; 6] = [
    "Rigidez de creencias", "Persistencia de metas", "Resistencia al cambio",
    "Sensibilidad a amenaza", "Sensibilidad social", "Exploración",
];
const EMOTIONS: [&str; 6] = ["Alegría", "Tristeza", "Miedo", "Ira", "Ansiedad", "Aversión"];

fn bar(ui: &mut egui::Ui, value: f32, width: f32) {
    let value = value.clamp(0.0, 1.0);
    ui.add(egui::ProgressBar::new(value).desired_width(width).text(format!("{:.0}%", value * 100.0)));
}

fn trait_grid(ui: &mut egui::Ui, state: PsychState) {
    ui.columns(2, |columns| {
        columns[0].label(RichText::new("Personalidad base").strong());
        columns[1].label(RichText::new("Rasgos derivados").strong());
    });
    ui.add_space(3.0);
    egui::Grid::new(ui.id().with("mspa-traits"))
        .num_columns(4)
        .spacing([8.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            for i in 0..6 {
                ui.label(BASE[i]);
                bar(ui, state.personality[i], 105.0);
                ui.label(DERIVED[i]);
                bar(ui, state.derived_traits[i], 105.0);
                ui.end_row();
            }
        });
}

fn state_for<'a>(
    game: &'a Game,
    society: &'a Society,
    id: u64,
    protagonist_name: &'a str,
) -> (&'a str, &'a Psychology) {
    if id == 1 {
        (protagonist_name, &game.psychology)
    } else if let Some(resident) = society.residents.iter().find(|resident| resident.id == id) {
        (&resident.avatar.name, &resident.psychology)
    } else {
        (protagonist_name, &game.psychology)
    }
}

pub fn draw(ctx: &egui::Context, game: &Game, society: &Society, protagonist_name: &str) {
    if game.creator { return; }

    let key = Id::new("mspa-inspected-character");
    let mut inspected = ctx.data_mut(|data| data.get_temp::<u64>(key).unwrap_or(1));
    if inspected != 1 && !society.residents.iter().any(|resident| resident.id == inspected) {
        inspected = 1;
    }

    egui::Window::new("MSPA · rasgos por personaje")
        .default_width(565.0)
        .default_pos([18.0, 106.0])
        .resizable(true)
        .collapsible(true)
        .show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.selectable_label(inspected == 1, protagonist_name).clicked() {
                    inspected = 1;
                }
                for resident in &society.residents {
                    if ui.selectable_label(inspected == resident.id, &resident.avatar.name).clicked() {
                        inspected = resident.id;
                    }
                }
            });
            ctx.data_mut(|data| data.insert_temp(key, inspected));
            ui.separator();

            let (name, psychology) = state_for(game, society, inspected, protagonist_name);
            let state = psychology.state;
            ui.horizontal(|ui| {
                ui.label(RichText::new(name).strong().size(17.0));
                ui.separator();
                ui.label(format!("Valencia {:+.2}", state.valence));
                ui.label(format!("Activación {:.0}%", state.activation.clamp(0.0, 1.0) * 100.0));
                ui.label(format!("Confianza {:.0}%", state.confidence.clamp(0.0, 1.0) * 100.0));
            });
            if let Some(error) = psychology.error.as_deref() {
                ui.colored_label(Color32::from_rgb(185, 72, 66), format!("MSPA: {error}"));
            }
            ui.small(format!(
                "Recuerdos {} · Creencias {} · Patrones aprendidos {}",
                state.memories, state.beliefs, state.learned_patterns
            ));
            ui.add_space(6.0);

            trait_grid(ui, state);
            ui.add_space(6.0);
            egui::CollapsingHeader::new("Estado emocional MSPA")
                .default_open(false)
                .show(ui, |ui| {
                    egui::Grid::new(ui.id().with("mspa-emotions"))
                        .num_columns(2)
                        .spacing([10.0, 4.0])
                        .show(ui, |ui| {
                            for (label, value) in EMOTIONS.into_iter().zip(state.emotions) {
                                ui.label(label);
                                bar(ui, value, 160.0);
                                ui.end_row();
                            }
                        });
                });
            ui.small("Los doce rasgos se leen directamente de MSPA; la interfaz no los recalcula.");
        });
}
