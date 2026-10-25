use crate::haunting::Haunting;
use bevy_egui::egui::{self, Align2, Color32, FontId, Id, Order, Rect, vec2};

pub fn draw(ctx: &egui::Context, haunting: &Haunting) {
    let screen = ctx.content_rect();
    let width = 330.0_f32.min((screen.width() - 28.0).max(240.0));
    let height = if haunting.event_remaining > 0.0 { 108.0 } else { 86.0 };
    let origin = egui::pos2(screen.center().x - width * 0.5, screen.top() + 92.0);
    egui::Area::new(Id::new("haunting-progress"))
        .fixed_pos(origin)
        .order(Order::Middle)
        .movable(false)
        .show(ctx, |ui| {
            ui.set_min_size(vec2(width, height));
            let rect = Rect::from_min_size(origin, vec2(width, height));
            ui.painter().rect_filled(rect, 17.0, Color32::from_rgba_unmultiplied(24, 31, 34, 205));
            ui.painter().text(
                origin + vec2(16.0, 13.0), Align2::LEFT_TOP,
                format!("PRESENCIA · NIVEL {}/20", haunting.level),
                FontId::proportional(12.0), Color32::from_rgb(194, 228, 220),
            );
            let title = haunting.current().map_or("Latente", |tier| tier.name);
            ui.painter().text(
                origin + vec2(16.0, 34.0), Align2::LEFT_TOP, title,
                FontId::proportional(16.0), Color32::WHITE,
            );
            let bar = Rect::from_min_size(origin + vec2(16.0, 61.0), vec2(width - 32.0, 8.0));
            ui.painter().rect_filled(bar, 4.0, Color32::from_rgb(67, 75, 77));
            ui.painter().rect_filled(
                Rect::from_min_size(bar.min, vec2(bar.width() * haunting.progress(), bar.height())),
                4.0, Color32::from_rgb(112, 177, 164),
            );
            if let Some(next) = haunting.next() {
                ui.painter().text(
                    origin + vec2(width - 16.0, 34.0), Align2::RIGHT_TOP,
                    format!("Siguiente: {}", next.name), FontId::proportional(10.0), Color32::from_rgb(169, 182, 181),
                );
            } else {
                ui.painter().text(
                    origin + vec2(width - 16.0, 34.0), Align2::RIGHT_TOP,
                    "Manifestación máxima", FontId::proportional(10.0), Color32::from_rgb(169, 182, 181),
                );
            }
            if haunting.event_remaining > 0.0 {
                ui.painter().text(
                    origin + vec2(16.0, 80.0), Align2::LEFT_TOP,
                    &haunting.last_event, FontId::proportional(10.5), Color32::from_rgb(226, 231, 226),
                );
            }
        });
}
