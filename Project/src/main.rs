use eframe::{egui,run_native};  
mod screenshot;

#[derive(PartialEq)]
enum Enum { First, Second, Third }

struct MyApp{
    hotkeys:Vec<String>,
    output_format:String,
    mode:i32,
    take_screen: bool
}

impl MyApp{
    //costructor for MyApp
    fn new()->MyApp{
        let mut h=Vec::new();
        let default_output_format=String::from(".jpg");
        //initial static hotkeys list
        h.push(ToString::to_string("Take Screenshot: Ctrl+K"));
        h.push(ToString::to_string("Save: Maiusc+C+U"));
        h.push(ToString::to_string("Boh: LOLOLOLOLOL"));

        MyApp{
            hotkeys:h,
            output_format:default_output_format,
            mode:0,
            take_screen: false
        }
    }
}

//implementing eframe::App trait for MyApp
impl eframe::App for MyApp{
    
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    //mandatory function for App trait
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        //custom window frame
        custom_window_frame(ctx, frame, "Screenshot Utility Tool", |frame: &mut eframe::Frame, ui| {//the title in this row is used
            let mut my_enum = Enum::First;
            //ui is needed to place widgets
            if self.mode==0{
                ui.label(egui::RichText::new(
                    "Welcome to the Screenshot Utility Tool, everything is ready to take a screenshot!")
                    .font(egui::FontId::proportional(17.5)));
                ui.horizontal(|ui| {//to place widgets on the same row
                    
                    if ui.button("Take Screenshot!").clicked(){
                        println!("pressed"); 
                        
                        frame.set_minimized(true);
                        self.mode=1;
                        self.take_screen=true;
                    }
                });
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Hotkey List:").font(egui::FontId::proportional(17.0)));
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Click on the shortcut to edit it").font(egui::FontId::proportional(17.0)));
                ui.add_space(10.0);

                ui.horizontal(|ui|{

                //hotkeys display 
                ui.vertical(|ui|{

                for h in &self.hotkeys{
                    let parts:Vec<&str>=h.split(":").collect();
                    
                    ui.horizontal(|ui|{
                    ui.label(egui::RichText::new(parts[0]).font(egui::FontId::proportional(14.0)));
                    if ui.link(egui::RichText::new(parts[1]).font(egui::FontId::proportional(14.0))).clicked(){
                        //modify hotkey
                    }
                    });
                }   
                });
                ui.add_space(60.0);
                //radio button for format selection
                ui.vertical(|ui|{
                    /* 
                    if ui.radio(selected_option, 0, Label::new("Opzione 1")).clicked() {
                        selected_option = 0;
                    }
                    if ui.radio(selected_option, 1, Label::new("Opzione 2")).clicked() {
                        selected_option = 1;
                    }
                    */
                })

                });

            }
            else if self.mode==1{
                println!("cambiato modalita!");
                if ui.button("return").clicked(){
                    self.mode=0;
                }
                if self.take_screen==true{
                    screenshot::full_screen(ui);
                    self.take_screen=false;
                }
                
            }
            });
    
    }
}

fn custom_window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut eframe::Frame, &mut egui::Ui),
) {


    let panel_frame = egui::Frame {
        fill: egui::Color32::LIGHT_BLUE, //background color
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };

    //Central Panel Component that implements custom panel_frame
    egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
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
        add_contents(frame, &mut content_ui);
    });
}

fn title_bar_ui(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    title_bar_rect: eframe::epaint::Rect,
    title: &str,
) {

    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, egui::Id::new("title_bar"), egui::Sense::click());

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        egui::Align2::CENTER_CENTER,
        title,
        egui::FontId::proportional(20.0),//title dimension 
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
        .add(egui::Button::new(egui::RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        frame.close();
    }

    if frame.info().window_info.maximized {
        let maximized_response = ui
            .add(egui::Button::new(egui::RichText::new("🗗").size(button_height)))
            .on_hover_text("Restore window");
        if maximized_response.clicked() {
            frame.set_maximized(false);
        }
    } else {
        let maximized_response = ui
            .add(egui::Button::new(egui::RichText::new("🗗").size(button_height)))
            .on_hover_text("Maximize window");
        if maximized_response.clicked() {
            frame.set_maximized(true);
        }
    }

    let minimized_response = ui
        .add(egui::Button::new(egui::RichText::new("🗕").size(button_height)))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        frame.set_minimized(true);
    }
}



fn main() {
//GUI(eframe) setup

let native_options=eframe::NativeOptions{
    //options for GUI window
    decorated: false,//roundede corners
    transparent: true,//no OS-specific bar
    ..Default::default()
};
//let native_options=eframe::NativeOptions::default();

//app_name,native_options,app_creator, when usign the custom frame window the name in the first field is not used
run_native("Screenshot Utility Tool",native_options, 
                Box::new(|_cc| Box::new(MyApp::new()))).expect("A probelem has occurred while starting up!");
                //|_cc| dummy closure, needed to make 
                // on the fly function
}
