use crate::haunting::Haunting;
use bevy_egui::egui::{self, Align2, Color32, FontId, Id, Order, Rect, Sense, Stroke, StrokeKind, vec2};

/// Draws idle progress plus the active power selector. Returns a power level when
/// the unseen agency deliberately invokes one of the already unlocked effects.
pub fn draw(ctx: &egui::Context, haunting: &mut Haunting) -> Option<u8> {
    let screen = ctx.content_rect();
    let width = 370.0_f32.min((screen.width() - 28.0).max(260.0));
    let height = if haunting.event_remaining > 0.0 { 166.0 } else { 145.0 };
    let origin = egui::pos2(screen.center().x - width * 0.5, screen.top() + 92.0);
    let mut invoke = None;
    egui::Area::new(Id::new("haunting-progress"))
        .fixed_pos(origin)
        .order(Order::Middle)
        .movable(false)
        .show(ctx, |ui| {
            ui.set_min_size(vec2(width, height));
            let rect = Rect::from_min_size(origin, vec2(width, height));
            ui.painter().rect_filled(rect, 17.0, Color32::from_rgba_unmultiplied(24, 31, 34, 216));
            ui.painter().text(
                origin + vec2(16.0, 13.0), Align2::LEFT_TOP,
                format!("PRESENCIA · NIVEL {}/20", haunting.level),
                FontId::proportional(12.0), Color32::from_rgb(194, 228, 220),
            );
            let title = haunting.current().map_or("Latente", |tier| tier.name);
            ui.painter().text(origin + vec2(16.0, 34.0), Align2::LEFT_TOP, title,
                FontId::proportional(16.0), Color32::WHITE);
            if let Some(next) = haunting.next() {
                ui.painter().text(origin + vec2(width - 16.0, 34.0), Align2::RIGHT_TOP,
                    format!("Siguiente: {}", next.name), FontId::proportional(10.0), Color32::from_rgb(169,182,181));
            } else {
                ui.painter().text(origin + vec2(width - 16.0, 34.0), Align2::RIGHT_TOP,
                    "Manifestación máxima", FontId::proportional(10.0), Color32::from_rgb(169,182,181));
            }
            let bar = Rect::from_min_size(origin + vec2(16.0, 61.0), vec2(width - 32.0, 8.0));
            ui.painter().rect_filled(bar, 4.0, Color32::from_rgb(67,75,77));
            ui.painter().rect_filled(Rect::from_min_size(bar.min, vec2(bar.width()*haunting.progress(),bar.height())),
                4.0, Color32::from_rgb(112,177,164));

            if haunting.level > 0 {
                let y = 80.0;
                let left = Rect::from_min_size(origin + vec2(16.0,y), vec2(34.0,34.0));
                let right = Rect::from_min_size(origin + vec2(width-50.0,y), vec2(34.0,34.0));
                let action = Rect::from_min_size(origin + vec2(56.0,y), vec2(width-112.0,34.0));
                for (r,label,dir) in [(left,"‹",-1_i8),(right,"›",1_i8)] {
                    let response=ui.interact(r,Id::new(("power-arrow",dir)),Sense::click());
                    ui.painter().rect(r,10.0,Color32::from_rgb(48,61,63),Stroke::new(1.0,Color32::from_rgb(91,116,112)),StrokeKind::Inside);
                    ui.painter().text(r.center(),Align2::CENTER_CENTER,label,FontId::proportional(20.0),Color32::WHITE);
                    if response.clicked(){if dir<0{haunting.select_previous();}else{haunting.select_next();}}
                }
                let selected=haunting.selected().expect("unlocked haunting power");
                let response=ui.interact(action,Id::new("invoke-haunting-power"),Sense::click());
                ui.painter().rect_filled(action,10.0,if response.hovered(){Color32::from_rgb(92,146,136)}else{Color32::from_rgb(67,112,105)});
                ui.painter().text(action.center(),Align2::CENTER_CENTER,
                    format!("MANIFESTAR · {}",selected.name),FontId::proportional(11.0),Color32::WHITE);
                response.clone().on_hover_text(selected.description);
                if response.clicked(){invoke=Some(haunting.selected_power);}
            } else {
                ui.painter().text(origin+vec2(16.0,86.0),Align2::LEFT_TOP,
                    "La presencia todavía no puede intervenir.",FontId::proportional(10.5),Color32::from_rgb(184,194,191));
            }

            if haunting.event_remaining > 0.0 {
                ui.painter().text(origin + vec2(16.0, 124.0), Align2::LEFT_TOP,
                    &haunting.last_event, FontId::proportional(10.5), Color32::from_rgb(226,231,226));
            }
        });
    invoke
}
