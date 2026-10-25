use crate::game::Task;
use bevy_egui::egui::{self, Align2, Color32, FontId, Id, Order, Rect, Sense, Stroke, StrokeKind, vec2};

/// Covers the legacy resident-action strip with actions performed by the unseen
/// environmental agency. The returned Task identifies the household cue only.
pub fn draw(ctx: &egui::Context) -> Option<Task> {
    let screen = ctx.content_rect();
    let mobile = screen.width() <= 760.0;
    let size = if mobile { vec2(164.0, 207.0) } else { vec2(462.0, 92.0) };
    let origin = egui::pos2(screen.right()-size.x-14.0, screen.bottom()-size.y-14.0);
    let mut selected = None;
    egui::Area::new(Id::new("poltergeist-actions"))
        .fixed_pos(origin)
        .order(Order::Foreground)
        .movable(false)
        .show(ctx,|ui| {
            ui.set_min_size(size);
            let panel=Rect::from_min_size(origin,size);
            ui.painter().rect(panel,18.0,Color32::from_rgba_unmultiplied(31,39,41,235),Stroke::new(1.0,Color32::from_rgb(85,103,101)),StrokeKind::Inside);
            ui.painter().text(origin+vec2(12.0,8.0),Align2::LEFT_TOP,"INTERVENIR EN LA CASA",FontId::proportional(10.0),Color32::from_rgb(180,216,207));
            let actions=[
                (Task::Eat,"Vajilla","◌"),
                (Task::Tv,"Televisión","▣"),
                (Task::Bed,"Cama","▰"),
                (Task::Shower,"Ducha","♨"),
                (Task::Toilet,"Cisterna","◉"),
            ];
            for(i,(task,label,icon))in actions.into_iter().enumerate(){
                let rect=if mobile{
                    let row=i/2;let col=i%2;
                    Rect::from_min_size(origin+vec2(10.0+col as f32*72.0,31.0+row as f32*55.0),vec2(66.0,48.0))
                }else{
                    Rect::from_min_size(origin+vec2(10.0+i as f32*89.0,31.0),vec2(82.0,50.0))
                };
                let response=ui.interact(rect,Id::new(("haunt-house",task.key())),Sense::click());
                ui.painter().rect_filled(rect,11.0,if response.hovered(){Color32::from_rgb(79,128,120)}else{Color32::from_rgb(50,62,64)});
                ui.painter().text(rect.center_top()+vec2(0.0,6.0),Align2::CENTER_TOP,icon,FontId::proportional(16.0),Color32::WHITE);
                ui.painter().text(rect.center_bottom()-vec2(0.0,6.0),Align2::CENTER_BOTTOM,label,FontId::proportional(10.0),Color32::WHITE);
                if response.clicked(){selected=Some(task);}
            }
        });
    selected
}
