use crate::{draw, screenshot, MyApp, hotkeys};
use arboard;
use eframe::egui;
use egui::Color32;
use native_dialog::{FileDialog, MessageDialog};
use std::borrow::Cow;
use std::thread;
use std::time::Duration;

#[derive(PartialEq)]
pub enum Paints {
    Arrow,
    Line,
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
                my_app.mode = 1;
            } else {
                frame.set_visible(false);
                my_app.mode = 1;
            }

            my_app.area = (0.0, 0.0, 0.0, 0.0);

            my_app.mode = 1;
        }
    });

    let ev = my_app.hotkey_conf.listen_to_event();
    match ev {
        None => {}
        Some(i) => {
            if i == 0 {
                //Take Screenshot hotkey
                frame.set_visible(false);
                my_app.mode = 1;
            }
            if i == 1 {
                MessageDialog::new()
                    .set_title("Error")
                    .set_text("Can't save before taking screenshot!")
                    .show_alert()
                    .unwrap();
            }
        }
    }
}

pub fn gui_mode4(my_app: &mut MyApp, ui: &mut egui::Ui){
    ui.label(
        egui::RichText::new(
            "There are multiple monitors\nChoose which monitor screen",
        )
        .font(egui::FontId::proportional(17.5)));
    ui.vertical(|ui| {
        for i in 0..my_app.image.len(){

        
        if ui
            .add(egui::RadioButton::new(
                my_app.n_monitor == i,
                (i+1).to_string(),
            ))
            .clicked()
        {
            my_app.n_monitor = i;
        }
    }
    if ui.button("Conferma").clicked(){
        my_app.mode=3;
    }
    });

}
pub fn gui_mode3(my_app: &mut MyApp, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        
                    screenshot::visualize_image(&mut my_app.image[my_app.n_monitor], ui, frame.info().window_info.size);
                    ui.horizontal(|ui|{
                    if ui.button("return").clicked() {
                        
                        my_app.mode = 0;
                        my_app.enable_screenshot=true;
                        
                    }
                    let window_name=String::from(String::from("screenshot")+&(my_app.default_name_index.to_string()));
                    if ui.button("save").clicked() {
                        let mut format_for_dialog="";
                        let mut format="";
                        if  my_app.output_format==".png"{
                            format_for_dialog="PNG";
                            format="png";
                        }else if my_app.output_format==".jpg"{
                            format_for_dialog="JPG";
                            format="jpg";
                        }
                        else if my_app.output_format==".gif"{
                            format_for_dialog="GIF";
                            format="gif";
                        }
                        //leave SOME as path wrapper!!!!!!!!
                        //format without the "." in front
                        let file_path=FileDialog::new().set_filename(&window_name).add_filter(format_for_dialog,&[format]).show_save_single_file().ok().unwrap();
                        match file_path{
                            Some(file_path)=>{
                                                let path_for_thread:String=file_path.to_string_lossy().to_string();
                                                let image_for_thread=my_app.image[my_app.n_monitor].clone();
                                                let output_format_for_thread=my_app.output_format.clone();
                                                thread::spawn(move ||{
                                                screenshot::save_image(&path_for_thread,&image_for_thread,&output_format_for_thread,false);
                                                println!("ho finito di salvare");
                                                });
                                                println!("path:{:?}",file_path);
                                                my_app.default_name_index=my_app.default_name_index+1;
                                                my_app.mode = 0;
                                            },
                            None=>my_app.mode=3//return to visualize the image
                        
                        }    
                    
                    }
                         if my_app.area.2 == 0.0 && my_app.area.3 == 0.0 {
                            if ui.button("crop").clicked() {
                                   my_app.mode = 5;
                                    frame.set_fullscreen(true);
                        }
                    }
                        if ui.button("copy").clicked(){
                            let mut clipboard= arboard::Clipboard::new().unwrap();
                          
                            let image_data=arboard::ImageData{
                                width:my_app.image[my_app.n_monitor].size.0,
                                height:my_app.image[my_app.n_monitor].size.1,
                                bytes:Cow::from(&my_app.image[my_app.n_monitor].screens)
                            };
                            clipboard.set_image(image_data).expect("Errore nel copy");
                            
                            
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

                let path_for_thread=String::from(String::from(&my_app.default_path)+&String::from("\\screenshot")+&(my_app.default_name_index.to_string()));
                let image_for_thread=my_app.image[my_app.n_monitor].clone();
                let output_format_for_thread=my_app.output_format.clone();
                thread::spawn(move ||{
                    screenshot::save_image(&path_for_thread,&image_for_thread,&output_format_for_thread,true);
                    println!("ho finito di salvare");
                            });
                
                my_app.default_name_index = my_app.default_name_index + 1;
                my_app.mode = 0;
            }
        }
    }
    ui.input(|i| {
        if i.pointer.is_decidedly_dragging() && i.pointer.primary_down() {
            if my_app.init_pos.is_none() {
                my_app.init_pos = i.pointer.press_origin();
                println!("{:?}", my_app.init_pos);
            } else {
                my_app.final_pos = i.pointer.hover_pos();
                //ui.painter().arrow(my_app.init_pos.unwrap(), my_app.final_pos.unwrap()-my_app.init_pos.unwrap(), ui.visuals().widgets.noninteractive.bg_stroke);
            }
        } else if i.pointer.primary_released() && my_app.init_pos.is_some() {
            my_app.final_pos = i.pointer.hover_pos();
            //my_app.paint = true;
            println!("{:?}, {:?}", my_app.init_pos, my_app.final_pos);
            my_app.paint.push((
                Paints::Arrow,
                my_app.init_pos.unwrap(),
                my_app.final_pos.unwrap(),
            ));
            my_app.init_pos = None;
            my_app.final_pos = None;
        }
    });

    for figure in &my_app.paint {
        if figure.0 == Paints::Arrow {
            ui.painter().arrow(
                figure.1,
                figure.2 - figure.1,
                egui::Stroke {
                    width: 1.5,
                    color: Color32::RED,
                },
            );
        } else if figure.0 == Paints::Line {
            ui.painter().line_segment(
                [figure.1, figure.2],
                egui::Stroke {
                    width: 1.5,
                    color: Color32::RED,
                },
            );
        }
    }
}

pub fn gui_mode5(my_app: &mut MyApp, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
    let position = ui.input(|i| i.pointer.hover_pos());
    let info = frame.info().window_info;
    let limits = (10.0, 80.0, info.size[0] - 20.0, info.size[1] - 44.0);
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
            my_app.image[my_app.n_monitor]=screenshot::screen_area(
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

pub fn custom_window_frame(
    my_app: &mut MyApp,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut MyApp, &mut eframe::Frame, &mut egui::Ui),
) {
    let panel_frame = egui::Frame {
        fill: egui::Color32::LIGHT_BLUE, //background color
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };

    //Central Panel Component that implements custom panel_frame
    egui::CentralPanel::default()
        .frame(panel_frame)
        .show(ctx, |ui| {
            let app_rect = ui.max_rect();

            let title_bar_height = 32.0;

            let title_bar_rect = {
                let mut rect = app_rect;
                rect.max.y = rect.min.y + title_bar_height;
                rect
            };

            title_bar_ui(ui, frame, title_bar_rect, title);

            // Add the contents:
            let content_rect = {
                let mut rect = app_rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
            .shrink(4.0);
            let mut content_ui = ui.child_ui(content_rect, *ui.layout());
            add_contents(my_app, frame, &mut content_ui);
        });
    }

fn title_bar_ui(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    title_bar_rect: eframe::epaint::Rect,
    title: &str,
) {
    let painter = ui.painter();

    let title_bar_response = ui.interact(
        title_bar_rect,
        egui::Id::new("title_bar"),
        egui::Sense::click(),
    );

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        egui::Align2::CENTER_CENTER,
        title,
        egui::FontId::proportional(20.0), //title dimension
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + egui::vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + egui::vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        frame.set_maximized(!frame.info().window_info.maximized);
    } else if title_bar_response.is_pointer_button_down_on() {
        frame.drag_window();
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui, frame);
        });
    });
}

//function to show the close/minimize/expand icon on the frame window
fn close_maximize_minimize(ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    let button_height = 12.0;

    let close_response = ui
        .add(egui::Button::new(
            egui::RichText::new("❌").size(button_height),
        ))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        frame.close();
    }

    if frame.info().window_info.maximized {
        let maximized_response = ui
            .add(egui::Button::new(
                egui::RichText::new("🗗").size(button_height),
            ))
            .on_hover_text("Restore window");
        if maximized_response.clicked() {
            frame.set_maximized(false);
        }
    } else {
        let maximized_response = ui
            .add(egui::Button::new(
                egui::RichText::new("🗗").size(button_height),
            ))
            .on_hover_text("Maximize window");
        if maximized_response.clicked() {
            frame.set_maximized(true);
        }
    }

    let minimized_response = ui
        .add(egui::Button::new(
            egui::RichText::new("🗕").size(button_height),
        ))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        frame.set_minimized(true);
    }
  
}
