use crate::{draw, screenshot, MyApp};
use native_dialog::FileDialog;
use keyboard_types::{Code, Modifiers};
use eframe::egui;


pub  fn gui_mode0(my_app:&mut MyApp,frame: &mut eframe::Frame,ui:&mut egui::Ui) {
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
            ui.label(
                egui::RichText::new("Format Selection:").font(egui::FontId::proportional(17.0)),
            );
        });
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Click on the shortcut to edit it")
                .font(egui::FontId::proportional(17.0)),
        );
        ui.add_space(10.0);

    ui.horizontal(|ui| {
        //hotkeys display
        ui.vertical(|ui| {
            for i in 0..my_app.hotkey_conf.get_hotkeys_len() {
                ui.horizontal(|ui| {
                    let u = my_app.hotkey_conf.get_hotkey_as_string(i);
                    ui.label(
                        egui::RichText::new(my_app.hotkey_conf.get_command(i))
                            .font(egui::FontId::proportional(14.0)),
                    );

                    if my_app.hotkey_conf.get_enable() {
                        if ui
                            .link(egui::RichText::new(u).font(egui::FontId::proportional(14.0)))
                            .clicked()
                        {
                            //If I click on the link, I unregister the hotkey
                            my_app.hotkey_conf.delete_hotkey(i);
                        };
                    } else {
                        let mut new_mod = my_app.hotkey_conf.get_new_mod();
                        let mut new_key = my_app.hotkey_conf.get_new_key();
                        if i != my_app.hotkey_conf.get_changed_hotkey() {
                            ui.label(u);
                        } else {
                            ui.vertical(|ui| {
                                //println!("{:?} + {:?}", self.modif, new_key);
                                egui::ComboBox::from_label("Set new modifier")
                                    .selected_text(format!("{:?}", new_mod.1))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut new_mod,
                                            (None, "".to_string()),
                                            "".to_string(),
                                        );
                                        ui.selectable_value(
                                            &mut new_mod,
                                            (Some(Modifiers::SHIFT), "SHIFT".to_string()),
                                            "SHIFT",
                                        );
                                        ui.selectable_value(
                                            &mut new_mod,
                                            (Some(Modifiers::CONTROL), "CTRL".to_string()),
                                            "CTRL",
                                        );
                                    });
                                egui::ComboBox::from_label("Set new key")
                                    .selected_text(format!("{:?}", new_key.1))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyA, "A".to_string()),
                                            "A",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyB, "B".to_string()),
                                            "B",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyC, "C".to_string()),
                                            "C",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyD, "D".to_string()),
                                            "D",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyE, "E".to_string()),
                                            "E",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyF, "F".to_string()),
                                            "F",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyG, "G".to_string()),
                                            "G",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyH, "H".to_string()),
                                            "H",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyI, "I".to_string()),
                                            "I",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyJ, "J".to_string()),
                                            "J",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyK, "K".to_string()),
                                            "K",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyL, "L".to_string()),
                                            "L",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyM, "M".to_string()),
                                            "M",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyN, "N".to_string()),
                                            "N",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyO, "O".to_string()),
                                            "O",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyP, "P".to_string()),
                                            "P",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyQ, "Q".to_string()),
                                            "Q",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyR, "R".to_string()),
                                            "R",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyS, "S".to_string()),
                                            "S",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyT, "T".to_string()),
                                            "T",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyU, "U".to_string()),
                                            "U",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyV, "V".to_string()),
                                            "V",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyW, "W".to_string()),
                                            "W",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyX, "X".to_string()),
                                            "X",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyY, "Y".to_string()),
                                            "Y",
                                        );
                                        ui.selectable_value(
                                            &mut new_key,
                                            (Code::KeyZ, "Z".to_string()),
                                            "Z",
                                        );
                                    });

                                if ui.button("Save").clicked() {
                                    let success = my_app.hotkey_conf.change_hotkey(
                                        i,
                                        new_mod.clone(),
                                        new_key.clone(),
                                    );
                                    if success {
                                        //modification could fail if for example I try to set an already registered hotkey
                                        my_app.hotkey_conf.set_enable(true);
                                    }
                                }

                                my_app.hotkey_conf.set_new_hotkey(new_mod, new_key);
                            });
                        }
                    }
                });
            }
        });
        ui.add_space(185.0);
        //radio button for format selection
        ui.vertical(|ui| {
            if ui
                .add(egui::RadioButton::new(
                    my_app.output_format == ".jpg",
                    ".jpg",
                ))
                .clicked()
            {
                my_app.output_format = String::from(".jpg");
            }

            if ui
                .add(egui::RadioButton::new(
                    my_app.output_format == ".png",
                    ".png",
                ))
                .clicked()
            {
                my_app.output_format = String::from(".png");
            }
            if ui
                .add(egui::RadioButton::new(
                    my_app.output_format == ".gif",
                    ".gif",
                ))
                .clicked()
            {
                my_app.output_format = String::from(".gif");
            }
        })
    });
    ui.add_space(10.0);
    ui.label(egui::RichText::new("Set delay:").font(egui::FontId::proportional(17.0)));
    ui.add_space(10.0);
    ui.add(egui::Slider::new(&mut my_app.delay_time, 0..=10).text("Delay in seconds"));
    ui.add_space(10.0);
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        //to place widgets on the same row

        if ui.add_enabled(my_app.enable_screenshot ,egui::Button::new("Take Screenshot!")).clicked() {
            if my_app.delay_time!=0{
                //tokio::time::delay_for(tokio::time::Duration::new(u64::from(my_app.delay_time),0)).await;
                my_app.enable_screenshot=false;
            }else{
                frame.set_visible(false);
                my_app.mode=1;
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
                frame.set_visible(false); //Per ora sono uguali

                my_app.mode = 1;
            } else if i == 1 {
                frame.set_visible(false);

                my_app.mode = 1;
            }
        }
    }
}

pub fn gui_mode3(my_self: &mut MyApp, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
    screenshot::visualize_image(&mut my_self.image, ui, frame.info().window_info.size);
    ui.horizontal(|ui| {
        if ui.button("return").clicked() {
                        
            my_self.mode = 0;
            
        }
        let window_name=String::from(String::from("screenshot")+&(my_self.default_name_index.to_string()));
        if ui.button("save").clicked() {
            let mut format_for_dialog="";
            let mut format="";
            if  my_self.output_format==".png"{
                format_for_dialog="PNG";
                format="png";
            }else if my_self.output_format==".jpg"{
                format_for_dialog="JPG";
                format="jpg";
            }
            else if my_self.output_format==".gif"{
                format_for_dialog="GIF";
                format="gif";
            }
            //leave SOME as path wrapper!!!!!!!!
            //format without the "." in front
            if let Some(file_path)=FileDialog::new().set_filename(&window_name).add_filter(format_for_dialog,&[format]).show_save_single_file().ok().unwrap(){
                //if path_file inserted by user is valid enter here
                screenshot::save_image(&file_path.to_string_lossy().to_string(),&mut my_self.image,&mut  my_self.output_format);
                    println!("path:{:?}",file_path);
                my_self.default_name_index=my_self.default_name_index+1;
            }
            my_self.mode = 0;
        }
        if my_self.area.2 == 0.0 && my_self.area.3 == 0.0 {
            if ui.button("cattura").clicked() {
                my_self.mode = 4;
                frame.set_fullscreen(true);
            }
        }
    });
}

pub fn gui_mode4(my_self: &mut MyApp, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
    let position = ui.input(|i| i.pointer.hover_pos());
    let info = frame.info().window_info;
    let limits = (10.0, 80.0, info.size[0] - 20.0, info.size[1] - 44.0);
    let props=((my_self.image[0].size.0 as f32)/(limits.2-limits.0), (my_self.image[0].size.1 as f32)/(limits.3-limits.1));

    draw::cut_rect(position, info, my_self, ui, limits);

    ui.horizontal(|ui| {
        if ui.button("Conferma").clicked() {
            frame.set_fullscreen(false);
            let width = ((my_self.area.2 - my_self.area.0).abs()* props.0) as u32;
            let height = ((my_self.area.3 - my_self.area.1).abs() *props.1) as u32;
            screenshot::screen_area(
                &mut my_self.image,
                ((my_self.area.0 - limits.0)* props.0) as u32,
                ((my_self.area.1 - limits.1)*props.1) as u32,
                width,
                height,
            );
            my_self.mode = 3;
        }
        if ui.button("Return").clicked() {
            my_self.mode = 3;
            frame.set_fullscreen(false);
            my_self.area = (0.0, 0.0, 0.0, 0.0);
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
