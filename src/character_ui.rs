use crate::{character_catalog::{self, CHARACTERS}, scene::Avatar};
use bevy_egui::egui::{self, Align2, Color32, FontId, Id, Order, Rect, Sense, Stroke, StrokeKind, vec2};

#[derive(Default)]
pub struct CharacterUiOutcome { pub changed: bool, pub start: bool }

fn card_color(selected: bool, hovered: bool) -> Color32 {
    if selected { Color32::from_rgb(78, 124, 119) }
    else if hovered { Color32::from_rgb(224, 235, 229) }
    else { Color32::from_rgb(242, 241, 233) }
}

pub fn draw(ctx: &egui::Context, avatar: &mut Avatar) -> CharacterUiOutcome {
    let screen = ctx.content_rect();
    let panel_w = 690.0_f32.min((screen.width() - 28.0).max(320.0));
    let panel_h = 520.0_f32.min((screen.height() - 28.0).max(420.0));
    let origin = screen.center() - vec2(panel_w, panel_h) * 0.5;
    let mut out = CharacterUiOutcome::default();
    let shade = ctx.layer_painter(egui::LayerId::new(Order::Foreground, Id::new("character-picker-shade")));
    shade.rect_filled(screen, 0.0, Color32::from_rgba_unmultiplied(24, 38, 43, 190));
    egui::Area::new(Id::new("character-picker")).fixed_pos(origin).order(Order::Foreground).movable(false).show(ctx, |ui| {
        ui.set_min_size(vec2(panel_w, panel_h));
        let rect = Rect::from_min_size(origin, vec2(panel_w, panel_h));
        ui.painter().rect(rect, 24.0, Color32::from_rgb(248, 246, 238), Stroke::new(1.0, Color32::from_rgb(212, 215, 208)), StrokeKind::Inside);
        ui.painter().text(origin + vec2(30.0, 28.0), Align2::LEFT_TOP, "ELIGE QUIÉN VIVE AQUÍ", FontId::proportional(13.0), Color32::from_rgb(88, 125, 122));
        ui.painter().text(origin + vec2(30.0, 50.0), Align2::LEFT_TOP, "Personaje", FontId::proportional(30.0), Color32::from_rgb(40, 48, 51));
        ui.painter().text(origin + vec2(30.0, 100.0), Align2::LEFT_CENTER, "Nombre", FontId::proportional(13.0), Color32::from_rgb(40, 48, 51));
        if ui.put(Rect::from_min_size(origin + vec2(112.0, 80.0), vec2(panel_w - 142.0, 40.0)), egui::TextEdit::singleline(&mut avatar.name).font(FontId::proportional(13.0))).changed() { out.changed = true; }
        let gap = 12.0;
        let card_w = (panel_w - 60.0 - gap * 2.0) / 3.0;
        let card_h = 112.0;
        for (display_index, (i, spec)) in character_catalog::available_characters().enumerate() {
            let row = display_index / 3; let col = display_index % 3;
            let min = origin + vec2(30.0 + col as f32 * (card_w + gap), 145.0 + row as f32 * (card_h + gap));
            let r = Rect::from_min_size(min, vec2(card_w, card_h));
            let response = ui.interact(r, Id::new(("character-card", i)), Sense::click());
            let selected = avatar.character == i;
            ui.painter().rect(r, 16.0, card_color(selected, response.hovered()), Stroke::new(1.0, Color32::from_rgb(205, 210, 202)), StrokeKind::Inside);
            ui.painter().text(r.center_top() + vec2(0.0, 22.0), Align2::CENTER_TOP, spec.name, FontId::proportional(17.0), if selected { Color32::WHITE } else { Color32::from_rgb(40,48,51) });
            let traits = match spec.voice {
                crate::character_catalog::Voice::SharpSchoolgirl => "mordaz · impulsiva",
                crate::character_catalog::Voice::SweetSchoolgirl => "dulce · impulsiva",
                crate::character_catalog::Voice::Reserved => "reservada · amable",
                crate::character_catalog::Voice::Excitable => "intensa · curiosa",
                crate::character_catalog::Voice::Playful => "juguetona · segura",
                crate::character_catalog::Voice::Gentle => "tranquila · empática",
            };
            ui.painter().text(r.center_bottom() - vec2(0.0, 24.0), Align2::CENTER_BOTTOM, traits, FontId::proportional(11.0), if selected { Color32::from_rgb(225,243,237) } else { Color32::from_rgb(102,116,117) });
            if response.clicked() && avatar.character != i {
                avatar.character = i;
                if avatar.name.trim().is_empty() || CHARACTERS.iter().any(|c| c.name == avatar.name) { avatar.name = spec.name.to_string(); }
                out.changed = true;
            }
        }
        ui.painter().text(origin + vec2(30.0, panel_h - 104.0), Align2::LEFT_BOTTOM,
            "Cada modelo conserva su personalidad MSPA; las demás pasean por el exterior.", FontId::proportional(10.5), Color32::from_rgb(102,116,117));
        let start = Rect::from_min_size(origin + vec2(30.0, panel_h - 76.0), vec2(panel_w - 60.0, 48.0));
        let response = ui.interact(start, Id::new("start-life"), Sense::click());
        ui.painter().rect_filled(start, 14.0, if response.hovered() { Color32::from_rgb(55, 105, 101) } else { Color32::from_rgb(71, 121, 118) });
        ui.painter().text(start.center(), Align2::CENTER_CENTER, "ENTRAR A LA CASA", FontId::proportional(14.0), Color32::WHITE);
        if response.clicked() { if avatar.name.trim().is_empty() { avatar.name = CHARACTERS[avatar.character].name.to_string(); out.changed = true; } out.start = true; }
    });
    out
}
