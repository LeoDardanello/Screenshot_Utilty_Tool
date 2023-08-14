use crate::{screenshot, MyApp};
use eframe::egui;

pub fn cut_rect(
    position: Option<egui::Pos2>,
    info: eframe::WindowInfo,
    my_self: &mut MyApp,
    ui: &mut egui::Ui,
    limits: (f32, f32, f32, f32)
) {
    
    let mut valid = false;
    let mut pos: egui::Pos2;

    match position {
        Some(_) => {
            pos = position.unwrap();
            if pos.x >= limits.0 && pos.y >= limits.1 && pos.x <= limits.2 && pos.y <= limits.3 {
                valid = true;
            }
        }
        None => {
            pos = egui::Pos2::default();
            valid = false;
        }
    }

    if position.is_some() {
        pos = position.unwrap();
    }

    if valid {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
        if ui.input(|i| i.pointer.primary_pressed()) {
            my_self.area = (0.0, 0.0, 0.0, 0.0);
            let start_pos = ui.input(|i| i.pointer.press_origin()).unwrap();
            my_self.area.0 = start_pos.x;
            my_self.area.1 = start_pos.y;
        }
        if ui.input(|i| i.pointer.primary_released()) {
            my_self.area.2 = pos.x;
            my_self.area.3 = pos.y;
        }
    }
    screenshot::visualize_image(&mut my_self.image, ui, info.size);
    if my_self.area.0 != 0.0 || my_self.area.1 != 0.0 {
        let mut my_stroke = egui::Stroke::default();
        my_stroke.color = egui::Color32::WHITE;
        my_stroke.width = 2.0;
        let mut my_rect = egui::Rect::NOTHING;

        if my_self.area.2 == 0.0 && my_self.area.3 == 0.0 {
            if valid {
                my_rect = egui::Rect::from_two_pos(egui::pos2(my_self.area.0, my_self.area.1), pos);
            }
        } else {
            my_rect = egui::Rect::from_two_pos(
                egui::pos2(my_self.area.0, my_self.area.1),
                egui::pos2(my_self.area.2, my_self.area.3),
            );
        }
        if my_rect.is_positive() {
            ui.painter().rect(
                my_rect,
                egui::Rounding::none(),
                egui::Color32::from_white_alpha(5),
                my_stroke,
            );
        }
    }
}