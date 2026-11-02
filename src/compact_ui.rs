use crate::{
    game::Game,
    haunting::Haunting,
    haunting_ui,
    mspa_ui,
    scene::Avatar,
    society::Society,
    ui as legacy_ui,
};
use bevy_egui::egui::{self, Color32, Id, RichText};

const PANEL: Color32 = Color32::from_rgba_premultiplied(247, 245, 236, 232);
const INK: Color32 = Color32::from_rgb(42, 50, 52);
const MUTED: Color32 = Color32::from_rgb(103, 121, 123);

fn toggle(ctx: &egui::Context, id: &'static str, default: bool) -> bool {
    ctx.data_mut(|data| data.get_temp::<bool>(Id::new(id)).unwrap_or(default))
}

fn set_toggle(ctx: &egui::Context, id: &'static str, value: bool) {
    ctx.data_mut(|data| data.insert_temp(Id::new(id), value));
}

pub fn draw(
    ctx: &egui::Context,
    game: &mut Game,
    avatar: &mut Avatar,
    society: &mut Society,
    haunting: &mut Haunting,
) -> Option<u8> {
    if game.creator {
        return None;
    }

    let screen = ctx.content_rect();
    let width = screen.width();

    let bar_w = 520.0_f32.min((width - 24.0).max(260.0));
    let bar_pos = egui::pos2(screen.center().x - bar_w * 0.5, screen.top() + 10.0);
    egui::Area::new(Id::new("compact-topbar"))
        .fixed_pos(bar_pos)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(PANEL)
                .corner_radius(12.0)
                .inner_margin(egui::Margin::symmetric(10, 7))
                .show(ui, |ui| {
                    ui.set_min_width(bar_w - 20.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("CASA DE {}", avatar.name.to_uppercase())).strong().color(INK));
                        ui.separator();
                        if ui.small_button(if game.paused { "▶" } else { "Ⅱ" }).clicked() {
                            game.paused = !game.paused;
                        }
                        ui.label(RichText::new(&game.displayed_clock).strong().color(INK));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(&game.action).small().color(MUTED));
                        });
                    });
                });
        });

    let status_pos = egui::pos2(screen.left() + 12.0, screen.bottom() - 96.0);
    egui::Area::new(Id::new("compact-status"))
        .fixed_pos(status_pos)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(PANEL)
                .corner_radius(12.0)
                .inner_margin(egui::Margin::symmetric(9, 7))
                .show(ui, |ui| {
                    ui.set_min_width(196.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&avatar.name).strong().color(INK));
                        ui.label(RichText::new(game.reaction.expression.label()).small().color(MUTED));
                    });
                    let mut worst: Vec<(usize, f64)> = game.displayed_values.iter().copied().enumerate().collect();
                    worst.sort_by(|a, b| a.1.total_cmp(&b.1));
                    for (index, value) in worst.into_iter().take(2) {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(legacy_ui::NEED_NAMES[index]).small().color(INK));
                            ui.add(egui::ProgressBar::new((value as f32 / 100.0).clamp(0.0, 1.0)).desired_width(105.0).show_percentage());
                        });
                    }
                });
        });

    let mut mspa_open = toggle(ctx, "compact-mspa-open", false);
    let mut social_open = toggle(ctx, "compact-social-open", false);
    let mut haunting_open = toggle(ctx, "compact-haunting-open", false);

    let tools_pos = egui::pos2((screen.right() - 300.0).max(screen.left() + 8.0), screen.top() + 58.0);
    egui::Area::new(Id::new("compact-tools"))
        .fixed_pos(tools_pos)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(Color32::from_rgba_premultiplied(247, 245, 236, 220))
                .corner_radius(10.0)
                .inner_margin(egui::Margin::symmetric(7, 5))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.selectable_label(mspa_open, "MSPA").clicked() { mspa_open = !mspa_open; }
                        if ui.selectable_label(social_open, format!("Personas · {}", society.residents.len())).clicked() { social_open = !social_open; }
                        if ui.selectable_label(haunting_open, format!("Presencia {}/20", haunting.level)).clicked() { haunting_open = !haunting_open; }
                    });
                });
        });

    set_toggle(ctx, "compact-mspa-open", mspa_open);
    set_toggle(ctx, "compact-social-open", social_open);
    set_toggle(ctx, "compact-haunting-open", haunting_open);

    if mspa_open {
        mspa_ui::draw(ctx, game, society, &avatar.name);
    }
    if social_open && !society.residents.is_empty() {
        legacy_ui::draw_social(ctx, game, society);
    }
    if haunting_open {
        return haunting_ui::draw(ctx, haunting);
    }

    None
}
