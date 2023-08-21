use crate::{gui::Paints, screenshot, HighlighterLine, MyApp, MyDraw};
use eframe::egui;
use std::cmp;

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

pub fn draw_shape(ui: &mut egui::Ui, my_app: &mut MyApp, rect: egui::Rect) {
    ui.input(|i| {
        //println!("start");
        let u = my_app.paint.len();
        //println!("prima{:?}", u);
        if i.pointer.is_decidedly_dragging() && i.pointer.primary_down() {
            if my_app.paint[u - 1].start.is_none() {
                let pos=i.pointer.press_origin();
                if pos.is_some() && rect.contains(pos.unwrap()){   
                    my_app.paint[u - 1].start = pos;
                }
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

pub fn write_text(ui: &mut egui::Ui, my_app: &mut MyApp,  rect: egui::Rect) {
    let u = my_app.paint.len() - 1;

    ui.input(|i| {
        
        if i.pointer.primary_down() &&  my_app.paint[u].start.is_none(){
            let pos=i.pointer.hover_pos();
            if pos.is_some() && rect.contains(pos.unwrap()) {
                my_app.paint[u].start = pos ;
                my_app.paint[u].end = pos ;
                my_app.paint.push(MyDraw::new(Paints::Text, my_app.edit_color))
                
            }
            
        }
    });

    let str_ref: &mut String = &mut my_app.paint[u].text;
    ui.add_space(250.0);
    ui.label(egui::RichText::new("Click in the image to clip text").font(egui::FontId::proportional(15.0)));
    ui.add(egui::TextEdit::singleline(str_ref).text_color(my_app.edit_color).font(egui::FontId::proportional(15.0)));

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
    let mut u=el.len();
    if u>0 && el[u-1].draw == paint {
        button = egui::Button::new(egui::RichText::new(icon).underline());
    }

    if ui.add(button).clicked() {
        

        if u>0 && el[u-1].start.is_none() {
            if el[u-1].draw==paint{
                el[u-1].draw=Paints::NoFigure;
            }
            else{
                el.pop();
                u=u-1;
            }
            
        }
        if u>0{
            println!("{}", u);
          println!("{:?}", el[u-1].start);  
        }
        
        if  !(!*eraser && u>0 && el[u-1].draw==Paints::NoFigure){
            el.push(MyDraw::new(paint, color));
        }
        
        *eraser=false;
    }
}


pub fn eraser_square(start: egui::Pos2, end: egui::Pos2, limits: (f32, f32, f32, f32), ui: &mut egui::Ui){
                let mut p1=start;
                let mut p2=end;

                if p1.x > p2.x {
                    p2.x = p1.x;
                    p1.x = end.x;
                }
                if p1.y > p2.y {
                    p2.y = p1.y;
                    p1.y = end.y;
                }

                if p1.x<limits.0{
                    p1.x=limits.0;
                }
                else if p1.x>limits.2{
                    p1.x=limits.2;
                }

                if p1.y<limits.1{
                    p1.y=limits.1;
                }
                else if p1.y>limits.3{
                    p1.y=limits.3;
                }

                if p2.x<limits.0{
                    p2.x=limits.0;
                }
                else if p2.x>limits.2{
                    p2.x=limits.2;
                }
                
                if p2.y<limits.1{
                    p2.y=limits.1;
                }
                else if p2.y>limits.3{
                    p2.y=limits.3;
                }
                let rect= egui::Rect::from_two_pos(p1, p2);
                ui.painter().rect(
                    rect,
                    egui::Rounding::none(),
                    egui::Color32::TRANSPARENT,
                    egui::Stroke {
                        width: 1.5,
                        color: egui::Color32::RED,
                    },
                );
                if ui.rect_contains_pointer(rect){
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                
                

}

pub fn eraser(ui: &mut egui::Ui,  erased_draw: &mut (Paints, String), rect: egui::Rect, paint: &mut Vec<MyDraw>){
   let combo=egui::ComboBox::from_label("Figures to eliminate")
    .selected_text(format!("{:?}", erased_draw.1))
    .show_ui(ui, |ui| {
        ui.selectable_value(
            erased_draw,
            (Paints::NoFigure, "None".to_string()),
            "None",
        );
        ui.selectable_value(
            erased_draw,
            (Paints::Arrow, "Arrow".to_string()),
            "Arrow",
        );
        ui.selectable_value(
            erased_draw,
            (Paints::Square, "Square".to_string()),
            "Square",
        );
        ui.selectable_value(
            erased_draw,
            (Paints::Circle, "Circle".to_string()),
            "Circle",
        );
        ui.selectable_value(
            erased_draw,
            (Paints::Text, "Text".to_string()),
            "Text",
        );
        ui.selectable_value(
            erased_draw,
            (Paints::Highlighter, "Highlighter".to_string()),
            "Highlighter"
        );
    });
    if combo.inner.is_none(){
    ui.input(|i|{
        let p=i.pointer.hover_pos();
        if p.is_some() && i.pointer.primary_clicked() {
            let pos=p.unwrap();
            if rect.contains(pos){
                paint.retain(|x|{
                    if x.start.is_some() && x.end.is_some(){
                        if x.draw == erased_draw.0{   //Cancel only the figures you have selected 
                            if x.draw == Paints::Circle{
                                let c = x.start.unwrap();
                                let r = c.distance(x.end.unwrap());
                                if pos.x as usize>=(c.x-r) as usize && pos.x as usize<=(c.x+r) as usize && pos.y as usize>=(c.y-r) as usize && pos.y as usize<=(c.y+r) as usize{
                                    return false;
                                }
                                else{
                                    return true;
                                }
                            }
                            else if x.draw == Paints::Highlighter{
                                if let Some(a) = &x.points{
                                    for i in 0..a.line.len()-1{
                                        let p1 = a.line[i];
                                        let p2 = a.line[i+1];
                                        if pos.x as usize>=cmp::min(p1.x as usize, p2.x as usize)-10 && pos.x as usize<=cmp::max(p1.x as usize, p2.x as usize)+10
                                        && pos.y as usize>=cmp::min(p1.y as usize, p2.y as usize)-10 && pos.y as usize<=cmp::max(p1.y as usize, p2.y as usize)+10{
                                            return false;
                                        }
                                    }
                                    return true;
                                }
                                else{
                                    return true;
                                }

                            }
                            else{
                                let p1 = x.start.unwrap();
                                let p2 = x.end.unwrap();
                                if pos.x as usize>=cmp::min(p1.x as usize, p2.x as usize) && pos.x as usize<=cmp::max(p1.x as usize, p2.x as usize)
                                && pos.y as usize>=cmp::min(p1.y as usize, p2.y as usize) && pos.y as usize<=cmp::max(p1.y as usize, p2.y as usize){
                                    return false;
                                }
                                else{
                                    return true;
                                }
                            }
                        }
                        else{
                            return true;
                        }
                    }
                    else{
                        return true;
                    }
                });
            }
            
        }
    });

    }
}