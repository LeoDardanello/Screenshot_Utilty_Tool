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

            if my_self.area.0>pos.x{
                my_self.area.2 = my_self.area.0;
                my_self.area.0 = pos.x;

            }
            if my_self.area.1>pos.y{
                my_self.area.3 = my_self.area.1;
                my_self.area.1 = pos.y;

            }

        }
    }
    screenshot::visualize_image(&mut my_self.image[my_self.n_monitor], ui, info.size);
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


pub fn draw_shape(ui: &mut egui::Ui, my_app:&mut MyApp, _frame: &mut eframe::Frame){
     ui.input(|i| {
        //println!("start");
        let u = my_app.paint.len();
        //println!("prima{:?}", u);
        if i.pointer.is_decidedly_dragging() && i.pointer.primary_down() {
            if my_app.paint[u-1].1.is_none(){
                my_app.paint[u-1].1=  i.pointer.press_origin();
                //println!("prima{:?}", my_app.paint[u-1].1);
            } else {
                my_app.paint[u-1].2 = i.pointer.hover_pos();
                //println!("dopo1{:?}", my_app.paint[u-1].2);
                //ui.painter().arrow(my_app.init_pos.unwrap(), my_app.final_pos.unwrap()-my_app.init_pos.unwrap(), ui.visuals().widgets.noninteractive.bg_stroke);
            }
        } else if i.pointer.primary_released() && my_app.paint[u-1].1.is_some() {
            
            //println!("dopo2{:?}", my_app.paint[u-1].1);
           my_app.paint.push((
                my_app.paint[u-1].0,
                None,
                None,
                Some(my_app.edit_color)
            ));
            
            my_app.paint[u-1].2=i.pointer.hover_pos();
            //my_app.paint = true;
            //println!("{:?} {:?}", my_app.paint[u-1].1, my_app.paint[u-1].2);

        }
    });

}

