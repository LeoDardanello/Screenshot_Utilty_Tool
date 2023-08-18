use crate::{draw, hotkeys, screenshot, MyApp, MyScreen};
use arboard;
use eframe::egui;
use native_dialog::{FileDialog, MessageDialog};
use std::borrow::Cow;
use std::thread;
use std::time::Duration;

#[derive(PartialEq,Clone,Copy)]
pub enum Paints {
    Arrow,
    Text,
    Square,
    Circle,
    Highlighter,
}


#[derive(Clone,Debug)]
pub struct HighlighterLine{
    pub line: Vec<egui::Pos2>,//Vec containing points of the highliter
    pub stroke:egui::Stroke
}

impl HighlighterLine {
    fn new() -> Self {
        Self {
            line: Vec::new(),
            stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(25, 200, 100)),
        }
    }

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
    if limits.3>=info.size[1]{
        limits.3=info.size[1]-10.0;
    }
    if my_app.def_paint.len()>0 && my_app.paint.len()==0{
        my_app.paint.clear();
        my_app.paint.append(&mut my_app.def_paint);
    }

    
     ui.horizontal(|ui| {

        if my_app.paint.last().is_some() && my_app.paint.last().unwrap().0==Paints::Highlighter{
            let hight=my_app.find_last_highliter_line().clone();
            if hight.is_some(){
                let mut line_struct=hight.unwrap();
                let u=my_app.paint.len();
            my_app.paint[u-1].4=Some(draw::highlight(&mut line_struct,ui));
            }
            
        }
        if ui.button("Return").clicked() {
            my_app.mode = 3;
            frame.set_fullscreen(false);
            my_app.paint.clear();
        }

        if draw::draw_button(Paints::Square,ui, my_app.paint.last()).clicked() {  
            if my_app.paint.len()>0 && my_app.paint[my_app.paint.len()-1].1.is_none(){
                my_app.paint.pop();
            }
            my_app.paint.push((
                Paints::Square,
                None,
                None,
                Some(my_app.edit_color),
                None
            ));
                
        }
        if draw::draw_button(Paints::Circle,ui, my_app.paint.last()).clicked() {
            if my_app.paint.len()>0 && my_app.paint[my_app.paint.len()-1].1.is_none(){
                my_app.paint.pop();
            }
            my_app.paint.push((
                Paints::Circle,
                None,
                None,
                Some(my_app.edit_color),
                None
            ));
        }
        if draw::draw_button(Paints::Arrow,ui, my_app.paint.last()).clicked() {

            if my_app.paint.len()>0 && my_app.paint[my_app.paint.len()-1].1.is_none(){
                my_app.paint.pop();
            }
            my_app.paint.push((
                Paints::Arrow,
                None,
                None,
                Some(my_app.edit_color),
                None
            )); 
 
        }
        if draw::draw_button(Paints::Text,ui, my_app.paint.last()).clicked() {
            my_app.paint.push((
                Paints::Text,
                None,
                None,
                Some(my_app.edit_color),
                None
            ));
        }

        if ui.add(egui::Button::new(egui::RichText::new("Highliter"))).clicked() {
            let new_line:HighlighterLine=HighlighterLine::new();
             my_app.paint.push((
                Paints::Highlighter,
                None,
                None,
                Some(my_app.edit_color),
                Some(new_line)
            ));
            my_app.highlighting=true;
        }
  
        let f=ui.color_edit_button_srgba(&mut my_app.edit_color);
       if f.clicked(){
        let u= my_app.paint.len();
            if u>0 {
                if my_app.paint[u-1].3.is_some(){
                    my_app.paint[u-1].3=None;
                }
                else{
                    my_app.paint[u-1].3=Some(my_app.edit_color);
                }
                
            }
       }
       if f.clicked_elsewhere(){
        let u= my_app.paint.len();
            if u>0{
                my_app.paint[u-1].3=Some(my_app.edit_color);
            }

       }
    

        if ui.button("Conferma").clicked() {
            frame.request_screenshot();
            my_app.def_paint.append(&mut my_app.paint.clone());

            my_app.mode = 3;
            


        }
        if my_app.paint.len()>0  && my_app.paint[my_app.paint.len()-1].3.is_some(){
        draw::draw_shape(ui, my_app, frame); 
        }
     });
     
    let my_rect=egui::Rect::from_two_pos(egui::pos2(limits.0, limits.1), egui::pos2(limits.2, limits.3));
    let painter= ui.painter().with_clip_rect(my_rect);
    for figure in &my_app.paint {
        if figure.1.is_some() && figure.2.is_some(){
            
        if figure.0 == Paints::Arrow {
            painter.arrow(
                figure.1.unwrap(),
                figure.2.unwrap() - figure.1.unwrap(),
                egui::Stroke {
                    width: 1.5,
                    color: figure.3.unwrap(),
                },
            );
        } else if figure.0 == Paints::Square {
            painter.rect(
                egui::Rect::from_two_pos(figure.1.unwrap(), figure.2.unwrap()),
                egui::Rounding::none(),
                egui::Color32::TRANSPARENT,
                egui::Stroke {
                    width: 1.5,
                    color: figure.3.unwrap(),//color selected with the color picker
                },
            );
        } else if figure.0==Paints::Circle{
            painter.circle(figure.1.unwrap(), figure.1.unwrap().distance(figure.2.unwrap()), egui::Color32::TRANSPARENT, egui::Stroke {
                    width: 1.5,
                    color: figure.3.unwrap(),//color selected with the color picker
                });
        }}
    }
}

