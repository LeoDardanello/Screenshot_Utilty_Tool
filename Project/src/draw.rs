use crate::{gui::Paints, screenshot, HighlighterLine, MyApp, MyDraw};
use eframe::egui;

pub fn cut_rect(
    position: Option<egui::Pos2>,
    info: eframe::WindowInfo,
    my_self: &mut MyApp,
    ui: &mut egui::Ui,
    limits: (f32, f32, f32, f32),
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

            if my_self.area.0 > pos.x {
                my_self.area.2 = my_self.area.0;
                my_self.area.0 = pos.x;
            }
            if my_self.area.1 > pos.y {
                my_self.area.3 = my_self.area.1;
                my_self.area.1 = pos.y;
            }
        }
    }
    screenshot::visualize_image(&mut my_self.image[my_self.n_monitor], ui, info.size, None);
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

pub fn draw_shape(ui: &mut egui::Ui, my_app: &mut MyApp, _frame: &mut eframe::Frame) {
    ui.input(|i| {
        //println!("start");
        let u = my_app.paint.len();
        //println!("prima{:?}", u);
        if i.pointer.is_decidedly_dragging() && i.pointer.primary_down() {
            if my_app.paint[u - 1].start.is_none() {
                my_app.paint[u - 1].start = i.pointer.press_origin();
                //println!("prima{:?}", my_app.paint[u-1].1);
            } else {
                my_app.paint[u - 1].end = i.pointer.hover_pos();
                //println!("dopo1{:?}", my_app.paint[u-1].2);
                //ui.painter().arrow(my_app.init_pos.unwrap(), my_app.final_pos.unwrap()-my_app.init_pos.unwrap(), ui.visuals().widgets.noninteractive.bg_stroke);
            }
        } else if i.pointer.primary_released() && my_app.paint[u - 1].start.is_some() {
            my_app.paint[u - 1].end = i.pointer.hover_pos();
            //println!("dopo2{:?}", my_app.paint[u-1].1);
            my_app
                .paint
                .push(MyDraw::new(my_app.paint[u - 1].draw, my_app.edit_color));

            //my_app.paint = true;
            //println!("{:?} {:?}", my_app.paint[u-1].1, my_app.paint[u-1].2);
        }
    });
}

pub fn write_text(ui: &mut egui::Ui, my_app: &mut MyApp, _frame: &mut eframe::Frame) {
    let u = my_app.paint.len() - 1;

    ui.input(|i| {
        if i.pointer.any_pressed() {
            my_app.paint[u].start = i.pointer.press_origin();
        }
    });

    let str_ref: &mut String = &mut my_app.paint[u].text;

    ui.add(egui::TextEdit::singleline(str_ref).text_color(my_app.edit_color));
}


pub fn highlight(
    current_line: &mut HighlighterLine,
    ui: &mut egui::Ui,
    rect: egui::Rect,
) -> HighlighterLine {
    //println!("{:?}",ui.available_size_before_wrap());

    let mut response = ui.allocate_rect(rect, egui::Sense::drag());
    let mut l = Vec::new();
    l.append(&mut current_line.line);

    // let to_screen = egui::emath::RectTransform::from_to(
    //     egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.square_proportions()),
    //     response.rect,
    // );
    // let from_screen = to_screen.inverse();

    //println!("pointer-pos:{:?}",response.interact_pointer_pos());
    if let Some(pointer_pos) = response.interact_pointer_pos() {
        //if the mouse is being clicked or dragged
        // let canvas_pos = from_screen * pointer_pos;
        //println!("canvas pos:{:?}", canvas_pos);
        if l.last() != Some(/*&canvas_pos*/&pointer_pos) {
            // println!("entro dentro:");
            // println!("prima del push:{:?}",current_line.line);
            l.push(/*to_screen * canvas_pos*/ pointer_pos);
            // println!("dopo del push:{:?}",current_line.line);
            response.mark_changed();
        }
    }
    //     let line=egui::Shape::line(points, current_line.stroke);
    //     println!("{:?}",line);
    //     painter.add(line);
    //     println!("response:{:?}",response);
    
    return HighlighterLine {
        line: l,
        width: current_line.width,
    };
}

pub fn draw_button(paint: Paints, ui: &mut egui::Ui, el: &mut Vec<MyDraw>, color: egui::Color32, eraser: &mut bool) {
    let mut icon: &str = "";
    if paint == Paints::Square {
        icon = "⬜";
    } else if paint == Paints::Circle {
        icon = "⭕";
    } else if paint == Paints::Arrow {
        icon = "↗";
    } else if paint == Paints::Text {
        icon = "Text";
    } else if paint == Paints::Highlighter {
        icon = "Highlighter";
    }

    let mut button = egui::Button::new(egui::RichText::new(icon));
    let last = el.last();
    if last.is_some() && last.unwrap().draw == paint {
        button = egui::Button::new(egui::RichText::new(icon).underline());
    }

    if ui.add(button).clicked() {
        *eraser=false;
        if last.is_some() && last.unwrap().start.is_none() {
            el.pop();
        }
        el.push(MyDraw::new(paint, color));
    }
}
