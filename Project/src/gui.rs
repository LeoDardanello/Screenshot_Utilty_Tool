use crate::{draw, hotkeys, screenshot, MyApp, MyScreen};
use arboard;
use eframe::egui;
use native_dialog::{FileDialog, MessageDialog};
use std::borrow::Cow;
use std::{thread, cmp};
use std::time::Duration;

#[derive(PartialEq,Clone,Copy)]
pub enum Paints {
    Arrow,
    Text,
    Square,
    Circle,
    Highlighter,
    NoFigure
}

pub fn gui_mode0(my_app: &mut MyApp, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
    ui.label(
        egui::RichText::new(
            "Welcome to the Screenshot Utility Tool, everything is ready to take a screenshot!",
        )
        .font(egui::FontId::proportional(17.5)),
    );
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Hotkey List:").font(egui::FontId::proportional(17.0)));
        ui.add_space(250.0);
        ui.label(egui::RichText::new("Format Selection:").font(egui::FontId::proportional(17.0)));
    });
    ui.add_space(10.0);

    hotkeys::edit_shortcut(my_app, ui);
    ui.add_space(40.0); //space between first and second group of widget
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(egui::RichText::new("Set delay:").font(egui::FontId::proportional(17.0)));
            ui.add_space(10.0);
            ui.add(egui::Slider::new(&mut my_app.delay_time, 0..=10).text("Delay in seconds"));
        });
        //space between delay setting and default path setting
        ui.add_space(80.0);
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("Current default path:").font(egui::FontId::proportional(17.0)),
            );
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(&my_app.default_path)
                        .font(egui::FontId::proportional(15.0)),
                );
                if ui.button("Change Default Path").clicked() {
                    let path = FileDialog::new().show_open_single_dir().unwrap();
                    match path {
                        Some(path_ok) => {
                            my_app.default_path = path_ok.to_string_lossy().to_string()
                        }
                        None => my_app.mode = 0,
                    }
                }
            })
        });
    });

    ui.horizontal(|ui| {
        //to place widgets on the same row

        if ui
            .add_enabled(
                my_app.enable_screenshot,
                egui::Button::new("Take Screenshot!"),
            )
            .clicked()
        {
            if my_app.delay_time != 0 {
                my_app.enable_screenshot = false;
                thread::sleep(Duration::new(u64::from(my_app.delay_time), 0));
            }
            frame.set_window_size(egui::Vec2 { x: 0.0, y: 0.0 });
            

            my_app.time = ui.input(|i| i.time);
            my_app.area = (0.0, 0.0, 0.0, 0.0);
            my_app.edit_image=MyScreen{screens:Vec::new(), size:(0,0)};
            my_app.def_paint.clear();
            my_app.paint.clear();
            my_app.mode = 1;
        }
    });
    

    let ev = my_app.hotkey_conf.listen_to_event();
    match ev {
        None => {}
        Some(i) => {
            if i == 0 {
                frame.set_window_size(egui::Vec2 { x: 0.0, y: 0.0 });

                my_app.time = ui.input(|i| i.time);
                my_app.area = (0.0, 0.0, 0.0, 0.0);
                my_app.def_paint.clear();
                my_app.paint.clear();
                my_app.edit_image=MyScreen{screens:Vec::new(), size:(0,0)};
                my_app.mode = 1;
            }
            if i == 1 {
                MessageDialog::new()
                    .set_title("Error")
                    .set_text("Can't save before taking screenshot!")
                    .show_alert()
                    .unwrap();
            }
            if i == 2 {
                MessageDialog::new()
                    .set_title("Error")
                    .set_text("Can't copy before taking screenshot!")
                    .show_alert()
                    .unwrap();
            }
        }
    }
}

pub fn gui_mode4(my_app: &mut MyApp, ui: &mut egui::Ui) {
    //Multiple screen support
    ui.label(
        egui::RichText::new("Multiple monitors detected\nChoose the monitor to acquire")
            .font(egui::FontId::proportional(17.5)),
    );
    ui.vertical(|ui| {
        for i in 0..my_app.image.len() {
            if ui
                .add(egui::RadioButton::new(
                    my_app.n_monitor == i,
                    (i + 1).to_string(),
                ))
                .clicked()
            {
                my_app.n_monitor = i;
            }
        }
        if ui.button("Conferma").clicked() {
            my_app.mode = 3;
        }
    });
}
pub fn gui_mode3(my_app: &mut MyApp, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
    if my_app.edit_image.screens.len()>0{
        screenshot::visualize_image(
            &mut my_app.edit_image,
            ui,
            frame.info().window_info.size,
        );
    }
    else{
        screenshot::visualize_image(
        &mut my_app.image[my_app.n_monitor],
        ui,
        frame.info().window_info.size,
    );
    }
    
    ui.horizontal(|ui| {
        if ui.button("return").clicked() {
            frame.set_fullscreen(false);
            my_app.mode = 0;
            
            my_app.enable_screenshot = true;
        }

        let window_name =
            String::from(String::from("screenshot") + &(my_app.default_name_index.to_string()));
        if ui.button("save").clicked() {
            let mut format_for_dialog = "";
            let mut format = "";
            if my_app.output_format == ".png" {
                format_for_dialog = "PNG";
                format = "png";
            } else if my_app.output_format == ".jpg" {
                format_for_dialog = "JPG";
                format = "jpg";
            } else if my_app.output_format == ".gif" {
                format_for_dialog = "GIF";
                format = "gif";
            }
            //leave SOME as path wrapper!!!!!!!!
            //format without the "." in front
            let file_path = FileDialog::new()
                .set_filename(&window_name)
                .add_filter(format_for_dialog, &[format])
                .show_save_single_file()
                .ok()
                .unwrap();
            match file_path {
                Some(file_path) => {
                    let mut image=& my_app.image[my_app.n_monitor];
                if my_app.edit_image.screens.len()>0{
                    image=&my_app.edit_image;
                }
                    let path_for_thread: String = file_path.to_string_lossy().to_string();
                    let image_for_thread = image.clone();
                    let output_format_for_thread = my_app.output_format.clone();
                    thread::spawn(move || {
                        screenshot::save_image(
                            &path_for_thread,
                            &image_for_thread,
                            &output_format_for_thread,
                            false,
                        );
                        println!("ho finito di salvare");
                    });
                    println!("path:{:?}", file_path);
                    frame.set_fullscreen(false);
                    my_app.default_name_index = my_app.default_name_index + 1;
                    my_app.mode = 0;
                }
                None => my_app.mode = 3, //return to visualize the image
            }
        }
        if my_app.area.2 == 0.0 && my_app.area.3 == 0.0 {
            if ui.button("crop").clicked() {
                my_app.mode = 5;
                frame.set_fullscreen(true);
            }
        }
        if ui.button("copy").clicked() {
            let mut clipboard = arboard::Clipboard::new().unwrap();
            let mut image=& my_app.image[my_app.n_monitor];
            if my_app.edit_image.screens.len()>0{
                image=&my_app.edit_image;
            }
            let image_data = arboard::ImageData {
                width: image.size.0,
                height: image.size.1,
                bytes: Cow::from(&image.screens),
            };
            clipboard.set_image(image_data).expect("Errore nel copy");
        }
        if ui.button("edit").clicked(){
            frame.set_fullscreen(true);
            my_app.mode=6;

        }
    });
    let ev = my_app.hotkey_conf.listen_to_event();

    match ev {
        None => {}
        Some(i) => {
            if i == 1 {
                //Save Hotkey
                /*println!("salvo screen");
                println!("default path:{}",my_app.default_path);
                println!("output_format:{}",my_app.output_format);*/

                let path_for_thread = String::from(
                    String::from(&my_app.default_path)
                        + &String::from("\\screenshot")
                        + &(my_app.default_name_index.to_string()),
                );
                let mut image=& my_app.image[my_app.n_monitor];
                if my_app.edit_image.screens.len()>0{
                    image=&my_app.edit_image;
                }
                let image_for_thread = image.clone();
                let output_format_for_thread = my_app.output_format.clone();
                thread::spawn(move || {
                    screenshot::save_image(
                        &path_for_thread,
                        &image_for_thread,
                        &output_format_for_thread,
                        true,
                    );
                    println!("ho finito di salvare");
                });
                frame.set_fullscreen(false);
                my_app.default_name_index = my_app.default_name_index + 1;
                my_app.mode = 0;
            }
            if i == 2 {
                //copy hotkey
                let mut clipboard = arboard::Clipboard::new().unwrap();
                let mut image=& my_app.image[my_app.n_monitor];
                if my_app.edit_image.screens.len()>0{
                    image=&my_app.edit_image;
                }
                let image_data = arboard::ImageData {
                    width: image.size.0,
                    height: image.size.1,
                    bytes: Cow::from(&image.screens),
                };
                clipboard.set_image(image_data).expect("Errore nel copy");
            }
        }
    }
   
}

pub fn gui_mode5(my_app: &mut MyApp, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
    let position = ui.input(|i| i.pointer.hover_pos());
    let info = frame.info().window_info;
    let mut limits = (10.0, 80.0, info.size[0] - 10.0, ((info.size[0] - 20.0)*my_app.image[my_app.n_monitor].size.1 as f32)/my_app.image[my_app.n_monitor].size.0 as f32);
    if limits.3>=info.size[1]{
        limits.3=info.size[1]-10.0;
    }
    let props = (
        (my_app.image[my_app.n_monitor].size.0 as f32) / (limits.2 - limits.0),
        (my_app.image[my_app.n_monitor].size.1 as f32) / (limits.3 - limits.1),
    );

    draw::cut_rect(position, info, my_app, ui, limits);

    ui.horizontal(|ui| {
        if ui.button("Conferma").clicked() {
            frame.set_fullscreen(false);
            let width = ((my_app.area.2 - my_app.area.0).abs() * props.0) as u32;
            let height = ((my_app.area.3 - my_app.area.1).abs() * props.1) as u32;
            my_app.image[my_app.n_monitor] = screenshot::screen_area(
                &mut my_app.image[my_app.n_monitor],
                ((my_app.area.0 - limits.0) * props.0) as u32,
                ((my_app.area.1 - limits.1) * props.1) as u32,
                width,
                height,
            );
            my_app.mode = 3;
        }
        if ui.button("Return").clicked() {
            my_app.mode = 3;
            frame.set_fullscreen(false);
            my_app.area = (0.0, 0.0, 0.0, 0.0);
        }
    });
}

//Annotation Tool 
pub fn gui_mode6(my_app: &mut MyApp, frame: &mut eframe::Frame, ui: &mut egui::Ui){
    screenshot::visualize_image(&mut my_app.image[my_app.n_monitor], ui, frame.info().window_info.size);
    let info = frame.info().window_info;
    let mut limits = (10.0, 80.0, info.size[0] - 10.0, ((info.size[0] - 20.0)*my_app.image[my_app.n_monitor].size.1 as f32)/my_app.image[my_app.n_monitor].size.0 as f32);
    let my_rect=egui::Rect::from_two_pos(egui::pos2(limits.0, limits.1), egui::pos2(limits.2, limits.3));
    
    if limits.3>=info.size[1]{
        limits.3=info.size[1]-10.0;
    }
    if my_app.def_paint.len()>0 && my_app.paint.len()==0{
        my_app.paint.clear();
        my_app.paint.append(&mut my_app.def_paint);
    }

    ui.horizontal(|ui| {

        if ui.button("Return").clicked() {
            my_app.mode = 3;
            frame.set_fullscreen(false);
            my_app.paint.clear();
        }
        draw::draw_button(Paints::Square,ui, &mut my_app.paint, my_app.edit_color, &mut my_app.eraser);
        draw::draw_button(Paints::Circle,ui, &mut my_app.paint, my_app.edit_color, &mut my_app.eraser);
        draw::draw_button(Paints::Arrow,ui, &mut my_app.paint, my_app.edit_color, &mut my_app.eraser);
        draw::draw_button(Paints::Text,ui, &mut my_app.paint, my_app.edit_color, &mut my_app.eraser);
        draw::draw_button(Paints::Highlighter,ui, &mut my_app.paint, my_app.edit_color, &mut my_app.eraser);
        if ui.button("eraser").clicked(){
            my_app.eraser=true;
            let u = my_app.paint.len();
            if u>0{
                my_app.paint[u-1].draw=Paints::NoFigure;
            }
        }

        /*if my_app.paint.len()>0{
        ui.add_enabled(my_app.paint.last().unwrap().draw==Paints::Highlighter ,{
            egui::Slider::new(&mut my_app.paint.last().unwrap().points.unwrap().width, 10..=30).text("Change Width")
        });
        }*/

        let f=ui.color_edit_button_srgba(&mut my_app.edit_color);
        if f.clicked(){
            if my_app.paint.len()>0 {
                let u= my_app.paint.len()-1;
                if my_app.paint[u].color.is_some(){
                    my_app.paint[u].color=None;
                }
                else{
                    my_app.paint[u].color=Some(my_app.edit_color);
                }
            }
       }
       if f.clicked_elsewhere(){
        let u=my_app.paint.len();
            if u>0{
                my_app.paint[u-1].color=Some(my_app.edit_color);
            }

       }
    
        if ui.button("Conferma").clicked() {
            frame.request_screenshot();
            my_app.def_paint.append(&mut my_app.paint.clone());

            my_app.mode = 3;
            
        }
        if my_app.paint.len()>0{
            if my_app.paint.last().unwrap().draw==Paints::Text{
                draw::write_text(ui, my_app, frame);
            }
            if my_app.paint.last().unwrap().color.is_some(){
            draw::draw_shape(ui, my_app, frame); 
            }

        }

     });
     if my_app.paint.last().is_some() && my_app.paint.last().unwrap().draw==Paints::Highlighter{
            let hight=my_app.find_last_highliter_line().clone();
            if hight.is_some(){
                let mut line_struct=hight.unwrap();
                let u=my_app.paint.len();
            my_app.paint[u-1].points=Some(draw::highlight(&mut line_struct,ui, my_rect));
            }
            
        }
    
    if my_app.eraser{
        ui.input(|i|{
            let p=i.pointer.hover_pos();
            if p.is_some() && i.pointer.primary_clicked(){
                let pos=p.unwrap();
                if pos.x >= limits.0 && pos.y >= limits.1 && pos.x <= limits.2 && pos.y <= limits.3{
                    my_app.paint.retain(|x|{
                        if x.start.is_some() && x.end.is_some(){
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
                    });
                }
                
            }
        });
    }
     
    let painter= ui.painter().with_clip_rect(my_rect);
    for figure in &my_app.paint {
        if figure.start.is_some() && figure.end.is_some(){
            
        if figure.draw == Paints::Arrow {
            if my_app.eraser{
                let mut p1=figure.start.unwrap();
                let mut p2=figure.end.unwrap();

                if p1.x > p2.x {
                    p2.x = p1.x;
                    p1.x = figure.end.unwrap().x;
                }
                if p1.y > p2.y {
                    p2.y = p1.y;
                    p1.y = figure.end.unwrap().y;
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

                ui.painter().rect(
                    egui::Rect::from_two_pos(p1, p2),
                    egui::Rounding::none(),
                    egui::Color32::TRANSPARENT,
                    egui::Stroke {
                        width: 1.5,
                        color: egui::Color32::RED,
                    },
                );
            }
            painter.arrow(
                figure.start.unwrap(),
                figure.end.unwrap() - figure.start.unwrap(),
                egui::Stroke {
                    width: 1.5,
                    color: figure.color.unwrap(),
                },
            );
        } else if figure.draw == Paints::Square {
            let col;
            if my_app.eraser{
                col = egui::Color32::RED;
            }
            else{
                col = figure.color.unwrap();
            }
            painter.rect(
                egui::Rect::from_two_pos(figure.start.unwrap(), figure.end.unwrap()),
                egui::Rounding::none(),
                egui::Color32::TRANSPARENT,
                egui::Stroke {
                    width: 1.5,
                    color: col,
                },
            );
        } else if figure.draw==Paints::Circle{
            if my_app.eraser{
                let c = figure.start.unwrap();
                let r = c.distance(figure.end.unwrap());

                ui.painter().rect(
                    egui::Rect::from_two_pos(egui::Pos2 { x: c.x-r , y: c.y-r }, egui::Pos2 { x: c.x+r , y: c.y+r }),
                    egui::Rounding::none(),
                    egui::Color32::TRANSPARENT,
                    egui::Stroke {
                        width: 1.5,
                        color: egui::Color32::RED,
                    },
                );
            }
            painter.circle(figure.start.unwrap(), figure.start.unwrap().distance(figure.end.unwrap()), egui::Color32::TRANSPARENT, egui::Stroke {
                    width: 1.5,
                    color: figure.color.unwrap(),//color selected with the color picker
            });
        }
        else if figure.draw==Paints::Highlighter{
            let points= figure.points.clone().unwrap();
            let stroke=egui::Stroke::new(points.width as f32, egui::Color32::from_rgba_unmultiplied(figure.color.unwrap().r(), figure.color.unwrap().g(), figure.color.unwrap().b(),50));
            let line=egui::Shape::line(points.line, stroke);
            painter.add(line);
        }
        else if figure.draw==Paints::Text{
            painter.text(figure.start.unwrap(), egui::Align2::LEFT_CENTER, figure.text.clone(), egui::FontId::default(),figure.color.unwrap());
        }
    
    }
    }
}

