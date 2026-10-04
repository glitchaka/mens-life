//! Native controls painted directly into the GPU frame.
use crate::{
    game::{Game, Task},
    scene::Avatar,
};
use bevy_egui::egui::{
    self, Align2, Color32, CornerRadius, FontId, Id, Order, Pos2, Rect, Sense, Stroke, StrokeKind,
    Vec2, pos2, vec2,
};
pub const NEED_NAMES: [&str; 5] = ["Energía", "Hambre", "Higiene", "Diversión", "Vejiga"];
pub const NEED_COLORS: [u32; 5] = [0x76b7b2, 0xe9ad62, 0x72a9d8, 0xd080a7, 0x9aafdd];
const INK: u32 = 0x283033;
pub fn color(h: u32) -> Color32 {
    Color32::from_rgb((h >> 16) as u8, (h >> 8) as u8, h as u8)
}
fn text(p: &egui::Painter, at: Pos2, align: Align2, s: &str, size: f32, h: u32) {
    p.text(
        at,
        align,
        s,
        FontId::new(
            size,
            if s == "Tu nueva vida"
                || s.starts_with("CASA DE")
                || NEED_NAMES.contains(&s)
                || [
                    "Nombre",
                    "Complexión",
                    "CREAR PERSONAJE",
                    "ENTRAR A LA CASA",
                    "Piel",
                    "Cabello",
                    "Camisa",
                    "Pantalón",
                    "Comer",
                    "Televisión",
                    "Dormir",
                    "Ducha",
                    "Baño",
                ]
                .contains(&s)
                || size == 15.0
                || (size == 16.0 && s.contains(':'))
            {
                egui::FontFamily::Name("strong".into())
            } else {
                egui::FontFamily::Proportional
            },
        ),
        color(h),
    );
}
fn panel(p: &egui::Painter, r: Rect, radius: u8, h: u32, alpha: u8) {
    p.add(
        egui::epaint::Shadow {
            offset: [0, 12],
            blur: 30,
            spread: 0,
            color: Color32::from_black_alpha(45),
        }
        .as_shape(r, CornerRadius::same(radius)),
    );
    let c = color(h);
    p.rect(
        r,
        CornerRadius::same(radius),
        Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), alpha),
        Stroke::new(1.0, Color32::from_white_alpha(180)),
        StrokeKind::Inside,
    );
}
fn diamond(p: &egui::Painter, c: Pos2, r: f32, h: u32) {
    p.add(egui::Shape::convex_polygon(
        vec![
            c + vec2(0.0, -r),
            c + vec2(r, 0.0),
            c + vec2(0.0, r),
            c + vec2(-r, 0.0),
        ],
        color(h),
        Stroke::NONE,
    ));
}
fn button(ui: &mut egui::Ui, r: Rect, id: &str, label: &str, size: f32, fill: u32) -> bool {
    let response = ui.interact(r, Id::new(id), Sense::click());
    let hover = response.hovered() && ui.is_enabled();
    ui.painter()
        .rect_filled(r, 12, color(if hover { 0x547f7d } else { fill }));
    text(
        ui.painter(),
        r.center(),
        Align2::CENTER_CENTER,
        label,
        size,
        if hover { 0xffffff } else { 0x344244 },
    );
    response.clicked()
}
#[derive(Default)]
pub struct UiOutcome {
    pub avatar_changed: bool,
}
pub fn draw(ctx: &egui::Context, game: &mut Game, avatar: &mut Avatar) -> UiOutcome {
    let screen = ctx.content_rect();
    let w = screen.width();
    let h = screen.height();
    let mobile = w <= 760.0;
    let tw = 760.0_f32.min(w - 28.0);
    let top = pos2((w - tw) / 2.0, 18.0);
    egui::Area::new(Id::new("topbar"))
        .fixed_pos(top)
        .order(Order::Middle)
        .movable(false)
        .show(ctx, |ui| {
            ui.set_min_size(vec2(tw, 64.0));
            if game.creator {
                ui.disable();
            }
            let r = Rect::from_min_size(top, vec2(tw, 64.0));
            panel(ui.painter(), r, 18, 0xf8f6ed, 230);
            let house = Rect::from_min_size(top + vec2(12.0, 12.0), vec2(40.0, 40.0));
            ui.painter().rect_filled(house, 12, color(0x527f7e));
            diamond(ui.painter(), house.center(), 9.0, 0xdff3e6);
            text(
                ui.painter(),
                top + vec2(62.0, 20.0),
                Align2::LEFT_TOP,
                &format!("CASA DE {}", avatar.name.to_uppercase()),
                if mobile { 11.0 } else { 13.0 },
                INK,
            );
            text(
                ui.painter(),
                top + vec2(62.0, 37.0),
                Align2::LEFT_TOP,
                "Martes · Primavera",
                if mobile { 10.0 } else { 12.0 },
                0x718083,
            );
            let tx = if mobile {
                r.right() - 155.0
            } else {
                r.center().x - 82.0
            };
            let tr = Rect::from_min_size(pos2(tx, top.y + 12.0), vec2(166.0, 40.0));
            ui.painter().rect_filled(tr, 13, color(0xe3e3d9));
            if button(
                ui,
                Rect::from_min_size(tr.min + vec2(5.0, 3.0), vec2(32.0, 34.0)),
                "pause",
                if game.paused { "▶" } else { "Ⅱ" },
                18.0,
                0xe3e3d9,
            ) {
                game.paused = !game.paused;
            }
            text(
                ui.painter(),
                tr.min + vec2(47.0, 20.0),
                Align2::LEFT_CENTER,
                &game.displayed_clock,
                16.0,
                INK,
            );
            text(
                ui.painter(),
                tr.min + vec2(106.0, 20.0),
                Align2::LEFT_CENTER,
                "▶ ▶ ▶",
                9.0,
                0x6b9890,
            );
            if !mobile
                && button(
                    ui,
                    Rect::from_min_size(pos2(r.right() - 54.0, top.y + 11.0), vec2(42.0, 42.0)),
                    "menu",
                    "☻",
                    22.0,
                    0xe4e5dc,
                )
            {
                game.creator = true;
            }
        });
    let nw = if mobile { 190.0 } else { 246.0 };
    let rh = if mobile { 34.0 } else { 39.0 };
    let pad = if mobile { 10.0 } else { 13.0 };
    let nh = pad * 2.0 + 57.0 + rh * game.needs_visible as f32;
    let np = pos2(
        if mobile { 10.0 } else { 22.0 },
        h - (if mobile { 10.0 } else { 24.0 }) - nh,
    );
    egui::Area::new(Id::new("needs"))
        .fixed_pos(np)
        .order(Order::Middle)
        .movable(false)
        .show(ctx, |ui| {
            ui.set_min_size(vec2(nw, nh));
            let r = Rect::from_min_size(np, vec2(nw, nh));
            panel(ui.painter(), r, 20, 0xf6f4ea, 234);
            let p = np + Vec2::splat(pad);
            diamond(ui.painter(), p + vec2(10.0, 22.0), 8.0, 0x59bd68);
            let ac = p + vec2(if mobile { 41.0 } else { 53.0 }, 22.0);
            ui.painter()
                .circle_filled(ac, if mobile { 18.0 } else { 22.0 }, color(0x315f79));
            let letter = avatar
                .name
                .chars()
                .next()
                .map(|c| c.to_uppercase().to_string())
                .unwrap_or_default();
            text(
                ui.painter(),
                ac,
                Align2::CENTER_CENTER,
                &letter,
                18.0,
                0xffffff,
            );
            let nx = if mobile { 65.0 } else { 85.0 };
            text(
                ui.painter(),
                p + vec2(nx, 6.0),
                Align2::LEFT_TOP,
                &avatar.name,
                15.0,
                INK,
            );
            text(
                ui.painter(),
                p + vec2(nx, 27.0),
                Align2::LEFT_TOP,
                &game.action,
                if mobile { 10.0 } else { 12.0 },
                0x718083,
            );
            ui.painter().line_segment(
                [p + vec2(0.0, 56.0), pos2(r.right() - pad, p.y + 56.0)],
                Stroke::new(1.0, color(0xccd1ca)),
            );
            for i in 0..game.needs_visible {
                let y = p.y + 66.0 + i as f32 * rh;
                text(
                    ui.painter(),
                    pos2(p.x + 12.0, y + 15.0),
                    Align2::CENTER_CENTER,
                    ["⚡", "●", "◈", "★", "◉"][i],
                    16.0,
                    INK,
                );
                text(
                    ui.painter(),
                    pos2(p.x + 32.0, y),
                    Align2::LEFT_TOP,
                    NEED_NAMES[i],
                    12.0,
                    INK,
                );
                let b = Rect::from_min_size(
                    pos2(p.x + 32.0, y + 19.0),
                    vec2(nw - pad * 2.0 - 32.0, 8.0),
                );
                ui.painter().rect_filled(b, 4, color(0xcdd0ca));
                ui.painter().rect_filled(
                    Rect::from_min_size(
                        b.min,
                        vec2(
                            b.width() * (game.displayed_values[i] as f32 / 100.0).clamp(0.0, 1.0),
                            8.0,
                        ),
                    ),
                    4,
                    color(NEED_COLORS[i]),
                );
            }
        });
    let az = if mobile {
        vec2(148.0, 191.0)
    } else {
        vec2(444.0, 77.0)
    };
    let ap = pos2(
        w - az.x - (if mobile { 10.0 } else { 22.0 }),
        h - az.y - (if mobile { 10.0 } else { 24.0 }),
    );
    egui::Area::new(Id::new("actions"))
        .fixed_pos(ap)
        .order(Order::Middle)
        .movable(false)
        .show(ctx, |ui| {
            ui.set_min_size(az);
            if game.creator || game.paused {
                ui.disable();
            }
            panel(ui.painter(), Rect::from_min_size(ap, az), 18, 0xf6f4ea, 234);
            for (i, (task, label, icon)) in [
                (Task::Eat, "Comer", "◌"),
                (Task::Tv, "Televisión", "▣"),
                (Task::Bed, "Dormir", "▰"),
                (Task::Shower, "Ducha", "♨"),
                (Task::Toilet, "Baño", "◉"),
            ]
            .into_iter()
            .enumerate()
            {
                let z = if mobile {
                    vec2(62.0, 53.0)
                } else {
                    vec2(if i == 1 { 92.0 } else { 76.0 }, 61.0)
                };
                let offset = if mobile {
                    vec2(8.0 + (i % 2) as f32 * 70.0, 8.0 + (i / 2) as f32 * 61.0)
                } else {
                    vec2(8.0 + i as f32 * 84.0 + if i > 1 { 16.0 } else { 0.0 }, 8.0)
                };
                let r = Rect::from_min_size(ap + offset, z);
                let response = ui.interact(r, Id::new(task.key()), Sense::click());
                let hover = response.hovered() && ui.is_enabled();
                let c = if hover { 0xffffff } else { 0x344244 };
                ui.painter()
                    .rect_filled(r, 12, color(if hover { 0x547f7d } else { 0xe1e2d8 }));
                text(
                    ui.painter(),
                    r.center_top() + vec2(0.0, 8.0),
                    Align2::CENTER_TOP,
                    icon,
                    if mobile { 16.0 } else { 20.0 },
                    c,
                );
                text(
                    ui.painter(),
                    r.center_bottom() - vec2(0.0, 9.0),
                    Align2::CENTER_BOTTOM,
                    label,
                    if mobile { 11.0 } else { 12.0 },
                    c,
                );
                if response.clicked() {
                    game.command(task);
                }
            }
        });
    let hp = pos2(w / 2.0, if mobile { 107.0 } else { h - 42.0 });
    let p = ctx.layer_painter(egui::LayerId::new(Order::Background, Id::new("hint")));
    p.rect_filled(
        Rect::from_center_size(hp, vec2(358.0_f32.min(w - 20.0), 34.0)),
        12,
        Color32::from_rgba_unmultiplied(39, 56, 60, 187),
    );
    text(
        &p,
        hp,
        Align2::CENTER_CENTER,
        "Autonomía activa · Pulsa un mueble para darle una indicación",
        12.0,
        0xffffff,
    );
    let mut out = UiOutcome::default();
    if game.creator {
        egui::Area::new(Id::new("creator-shade"))
            .fixed_pos(screen.min)
            .order(Order::Foreground)
            .movable(false)
            .show(ctx, |ui| {
                ui.allocate_rect(screen, Sense::hover());
                let mut m = egui::epaint::Mesh::default();
                for (at, c) in [
                    (
                        screen.left_top(),
                        Color32::from_rgba_unmultiplied(32, 52, 62, 217),
                    ),
                    (
                        screen.right_top(),
                        Color32::from_rgba_unmultiplied(109, 144, 142, 170),
                    ),
                    (
                        screen.right_bottom(),
                        Color32::from_rgba_unmultiplied(109, 144, 142, 170),
                    ),
                    (
                        screen.left_bottom(),
                        Color32::from_rgba_unmultiplied(32, 52, 62, 217),
                    ),
                ] {
                    m.colored_vertex(at, c);
                }
                m.add_triangle(0, 1, 2);
                m.add_triangle(0, 2, 3);
                ui.painter().add(m);
            });
        let size = vec2(520.0_f32.min(w - 28.0), 398.0_f32.min(h - 28.0));
        let origin = screen.center() - size / 2.0;
        egui::Area::new(Id::new("creator"))
            .fixed_pos(origin)
            .order(Order::Foreground)
            .movable(false)
            .show(ctx, |ui| {
                ui.set_min_size(size);
                let r = Rect::from_min_size(origin, size);
                panel(ui.painter(), r, 26, 0xf4f1e7, 245);
                let p = origin + vec2(28.0, 28.0);
                text(
                    ui.painter(),
                    p,
                    Align2::LEFT_TOP,
                    "CREAR PERSONAJE",
                    11.0,
                    0x64807e,
                );
                text(
                    ui.painter(),
                    p + vec2(0.0, 19.0),
                    Align2::LEFT_TOP,
                    "Tu nueva vida",
                    32.0,
                    INK,
                );
                let fw = (size.x - 178.0).max(100.0);
                text(
                    ui.painter(),
                    p + vec2(0.0, 102.0),
                    Align2::LEFT_CENTER,
                    "Nombre",
                    13.0,
                    INK,
                );
                out.avatar_changed |= ui
                    .put(
                        Rect::from_min_size(p + vec2(122.0, 80.0), vec2(fw, 44.0)),
                        egui::TextEdit::singleline(&mut avatar.name)
                            .font(FontId::proportional(13.0))
                            .margin(vec2(13.0, 12.0)),
                    )
                    .changed();
                text(
                    ui.painter(),
                    p + vec2(0.0, 158.0),
                    Align2::LEFT_CENTER,
                    "Complexión",
                    13.0,
                    INK,
                );
                ui.scope_builder(
                    egui::UiBuilder::new()
                        .max_rect(Rect::from_min_size(p + vec2(122.0, 136.0), vec2(fw, 44.0))),
                    |ui| {
                        ui.spacing_mut().interact_size = vec2(fw, 44.0);
                        egui::ComboBox::from_id_salt("body")
                            .selected_text(&avatar.body)
                            .width(fw - 8.0)
                            .show_ui(ui, |ui| {
                                for b in ["delgado", "adulto", "atlético"] {
                                    out.avatar_changed |= ui
                                        .selectable_value(&mut avatar.body, b.into(), b)
                                        .changed();
                                }
                            });
                    },
                );
                let sw = (size.x - 83.0) / 4.0;
                for (i, (label, value)) in [
                    ("Piel", &mut avatar.skin),
                    ("Cabello", &mut avatar.hair),
                    ("Camisa", &mut avatar.shirt),
                    ("Pantalón", &mut avatar.pants),
                ]
                .into_iter()
                .enumerate()
                {
                    let at = p + vec2(i as f32 * (sw + 9.0), 201.0);
                    text(ui.painter(), at, Align2::LEFT_TOP, label, 11.0, INK);
                    let hex = u32::from_str_radix(value.trim_start_matches('#'), 16).unwrap_or(0);
                    let mut rgb = [(hex >> 16) as u8, (hex >> 8) as u8, hex as u8];
                    ui.scope_builder(
                        egui::UiBuilder::new()
                            .max_rect(Rect::from_min_size(at + vec2(0.0, 21.0), vec2(sw, 40.0))),
                        |ui| {
                            ui.spacing_mut().interact_size = vec2(sw, 40.0);
                            if ui.color_edit_button_srgb(&mut rgb).changed() {
                                *value = format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]);
                                out.avatar_changed = true;
                            }
                        },
                    );
                }
                let play =
                    Rect::from_min_size(pos2(p.x, r.bottom() - 80.0), vec2(size.x - 56.0, 52.0));
                let response = ui.interact(play, Id::new("play"), Sense::click());
                ui.painter().rect_filled(
                    play,
                    14,
                    color(if response.hovered() {
                        0x386864
                    } else {
                        0x477976
                    }),
                );
                text(
                    ui.painter(),
                    play.center(),
                    Align2::CENTER_CENTER,
                    "ENTRAR A LA CASA",
                    13.0,
                    0xffffff,
                );
                if response.clicked() {
                    if avatar.name.trim().is_empty() {
                        avatar.name = "Alex".into();
                        out.avatar_changed = true;
                    }
                    game.creator = false;
                }
            });
    }
    out
}
pub fn configure(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "inter".into(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/Inter-Regular.ttf")).into(),
    );
    fonts.font_data.insert(
        "inter-bold".into(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/Inter-Bold.ttf")).into(),
    );
    fonts.font_data.insert(
        "symbols".into(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/Symbols.ttf")).into(),
    );
    let fallback = fonts.families[&egui::FontFamily::Proportional].clone();
    let mut regular = vec!["inter".into(), "symbols".into()];
    regular.extend(fallback.clone());
    let mut strong = vec!["inter-bold".into(), "symbols".into()];
    strong.extend(fallback);
    fonts
        .families
        .insert(egui::FontFamily::Proportional, regular);
    fonts
        .families
        .insert(egui::FontFamily::Name("strong".into()), strong);
    ctx.set_fonts(fonts);

    let mut s = (*ctx.style()).clone();
    s.visuals = egui::Visuals::light();
    s.visuals.override_text_color = Some(color(INK));
    s.visuals.widgets.inactive.corner_radius = CornerRadius::same(12);
    s.visuals.widgets.inactive.bg_fill = Color32::WHITE;
    s.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, color(0xc6cbc4));
    s.visuals.widgets.hovered.corner_radius = CornerRadius::same(12);
    s.spacing.item_spacing = vec2(9.0, 9.0);
    ctx.set_style(s);
}
