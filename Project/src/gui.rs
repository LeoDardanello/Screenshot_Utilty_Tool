use crate::{screenshot, MyApp};
use eframe::egui;

pub fn gui_mode0(my_app:&mut MyApp,frame: &mut eframe::Frame,ui:&mut egui::Ui) {
    if my_app.take_screen == false {
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
                for h in &my_app.hotkeys {
                    let parts: Vec<&str> = h.split(":").collect();

                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(parts[0]).font(egui::FontId::proportional(14.0)),
                        );
                        if ui
                            .link(
                                egui::RichText::new(parts[1])
                                    .font(egui::FontId::proportional(14.0)),
                            )
                            .clicked()
                        {
                            //modify hotkey
                        }
                    });
                }
            });
            ui.add_space(185.0);
            //radio button for format selection
            ui.vertical(|ui| {
                if ui
                    .add(egui::RadioButton::new(my_app.output_format == ".jpg", ".jpg"))
                    .clicked()
                {
                    my_app.output_format = String::from(".jpg");
                }

                if ui
                    .add(egui::RadioButton::new(my_app.output_format == ".png", ".png"))
                    .clicked()
                {
                    my_app.output_format = String::from(".png");
                }
                if ui
                    .add(egui::RadioButton::new(my_app.output_format == ".gif", ".gif"))
                    .clicked()
                {
                    my_app.output_format = String::from(".gif");
                }
            })
        });
        ui.add_space(10.0);
    }
    ui.horizontal(|ui| {
        //to place widgets on the same row

        if ui.button("Take Screenshot!").clicked() {
            println!("pressed");
            my_app.mode=1;
            
        }

    });
}

pub fn gui_mode2(
    my_self: &mut MyApp,

    frame: &mut eframe::Frame,
    ui: &mut egui::Ui,
) {
    if ui.button("screen").clicked() {

            screenshot::full_screen(ui);
            my_self.take_screen= false;

            
            my_self.mode = 3;
            frame.set_fullscreen(false);
        
    }
}


pub fn custom_window_frame(
    my_app:&mut MyApp,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut MyApp, &mut eframe::Frame, &mut egui::Ui),
) {
    let panel_frame = egui::Frame {
        fill: 
            egui::Color32::LIGHT_BLUE, //background color
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };
    if panel_frame.fill == egui::Color32::TRANSPARENT {
        my_app.mode=2;
    }

    //Central Panel Component that implements custom panel_frame
    let mut central=egui::CentralPanel::default();
        
    if my_app.mode!=1{
        central= central.frame(panel_frame);
    }

        central.show(ctx, |ui| {
            let app_rect = ui.max_rect();

            let mut title_bar_height = 32.0;
            if my_app.take_screen {
                title_bar_height = 0.0 as f32;
            }

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
