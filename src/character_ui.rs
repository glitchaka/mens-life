use crate::{character_catalog::{self, CHARACTERS}, scene::Avatar};
use bevy_egui::egui::{self, Align, Color32, Id, Layout, Order, RichText, vec2};

#[derive(Default)]
pub struct CharacterUiOutcome { pub changed: bool, pub start: bool }

pub fn draw(ctx: &egui::Context, avatar: &mut Avatar) -> CharacterUiOutcome {
    let screen = ctx.content_rect();
    let width = (screen.width() - 32.0).clamp(520.0, 920.0);
    let origin = egui::pos2(screen.center().x - width * 0.5, screen.bottom() - 142.0);
    let mut out = CharacterUiOutcome::default();

    egui::Area::new(Id::new("character-dock"))
        .fixed_pos(origin)
        .order(Order::Foreground)
        .movable(false)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(Color32::from_rgba_premultiplied(20, 27, 29, 235))
                .corner_radius(14.0)
                .inner_margin(egui::Margin::symmetric(12, 10))
                .show(ui, |ui| {
                    ui.set_min_width(width - 24.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("¿Quién vive aquí?").strong().size(15.0).color(Color32::WHITE));
                        ui.add_space(12.0);
                        let name = ui.add_sized(
                            [210.0, 28.0],
                            egui::TextEdit::singleline(&mut avatar.name).hint_text("Nombre"),
                        );
                        if name.changed() { out.changed = true; }
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui.add_sized([128.0, 30.0], egui::Button::new(RichText::new("Entrar").strong())).clicked() {
                                if avatar.name.trim().is_empty() {
                                    avatar.name = CHARACTERS[avatar.character].name.to_string();
                                    out.changed = true;
                                }
                                out.start = true;
                            }
                        });
                    });

                    ui.add_space(8.0);
                    ui.horizontal_wrapped(|ui| {
                        for (i, spec) in character_catalog::available_characters() {
                            let selected = avatar.character == i;
                            let label = if selected { format!("● {}", spec.name) } else { spec.name.to_string() };
                            if ui.selectable_label(selected, RichText::new(label).size(13.0)).clicked() && !selected {
                                let old_name_is_character = avatar.name.trim().is_empty() || CHARACTERS.iter().any(|c| c.name == avatar.name);
                                avatar.character = i;
                                if old_name_is_character { avatar.name = spec.name.to_string(); }
                                out.changed = true;
                            }
                        }
                    });

                    ui.add_space(3.0);
                    ui.label(RichText::new("La casa queda visible; elige residente y entra.").small().color(Color32::from_rgb(169, 184, 182)));
                });
        });

    let _ = vec2(0.0, 0.0);
    out
}
