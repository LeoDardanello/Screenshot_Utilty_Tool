use eframe::{egui, run_native};
use hotkeys::HotkeysConfig;
mod draw;
mod gui;
mod hotkeys;
mod screenshot;

#[derive(Clone)]
pub struct MyScreen {
    screens: Vec<u8>,
    size: (usize, usize),
}

pub struct MyApp {
    hotkey_conf: HotkeysConfig,
    output_format: String,
    mode: i32,
    image: Vec<MyScreen>,
    default_name_index: i32,
    area: (f32, f32, f32, f32),
    delay_time: u32,
    n_monitor: usize,
    enable_screenshot: bool,
    default_path: String,
    init_pos: Option<egui::Pos2>,
    final_pos: Option<egui::Pos2>,
    paint: Vec<(gui::Paints, egui::Pos2, egui::Pos2)>,
    time: f64
}

impl MyApp {
    //costructor for MyApp
    fn new() -> MyApp {
        let default_output_format = String::from(".jpg"); //default output format
                                                          //initial static hotkeys list
        MyApp {
            hotkey_conf: HotkeysConfig::new(),
            output_format: default_output_format,
            mode: 0,
            image: Vec::new(),
            area: (0.0, 0.0, 0.0, 0.0),
            default_name_index: 0,
            delay_time: 0,
            n_monitor: 0,
            enable_screenshot: true,
            //use backslashes to be compatible with different OS
            default_path: String::from(".\\..\\screenshot_default"), //default screenshot save location, used by save hotkey

            init_pos: None,
            final_pos: None,
            paint: Vec::new(),
            time: 0.0
        }
    }
}

//implementing eframe::App trait for MyApp

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    //mandatory function for App trait
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        //custom window frame

        if self.mode == 0 {
            gui::custom_window_frame(
                self,
                ctx,
                frame,
                "Screenshot Utility Tool", //the title in this row is used
                |my_app: &mut Self, frame: &mut eframe::Frame, ui| {
         
                    gui::gui_mode0(my_app, frame, ui);
                },
            );

            //self.hotkey_conf.listen_to_event();
        }else if self.mode==1{
            gui::custom_window_frame(
                self,
                ctx,
                frame,
                "Screenshot Utility Tool", //the title in this row is used
                |my_app: &mut Self, frame: &mut eframe::Frame, ui| {
                    println!("{:?}", ui.input(|i| i.time)-my_app.time);
                   if ui.input(|i| i.time)-my_app.time>=0.2 || frame.info().window_info.focused{
                        my_app.mode = 2;
                     }

                },
            );

        }

        else if self.mode == 2 {
            gui::custom_window_frame(
                self,
                ctx,
                frame,
                "Screenshot Utility Tool", //the title in this row is used
                |my_app: &mut Self, frame: &mut eframe::Frame, ui| {
                 
                    println!("finish{:?}", ui.input(|i| i.time)-my_app.time);
                        frame.set_minimized(false);
                        my_app.mode = 3;
                        my_app.image = screenshot::full_screen();
                        
                        
                        if my_app.image.len() > 1 {
                            my_app.mode = 4;
                        }
                     
                },
            );
        } else if self.mode == 3 {
            gui::custom_window_frame(
                self,
                ctx,
                frame,
                "Screenshot Utility Tool", //the title in this row is used
                |my_app: &mut Self, frame: &mut eframe::Frame, ui| {
                    gui::gui_mode3(my_app, frame, ui);
                },
            );
        } else if self.mode == 4 {
            gui::custom_window_frame(
                self,
                ctx,
                frame,
                "Screenshot Utility Tool", //the title in this row is used
                |my_app: &mut Self, _frame: &mut eframe::Frame, ui| {
                    gui::gui_mode4(my_app, ui);
                },
            );
        } else if self.mode == 5 {
            gui::custom_window_frame(
                self,
                ctx,
                frame,
                "Screenshot Utility Tool", //the title in this row is used
                |my_app: &mut Self, frame: &mut eframe::Frame, ui| {
                    gui::gui_mode5(my_app, frame, ui);
                },
            );
        }
    }
}

fn main() {
    //GUI(eframe) setup

    let native_options = eframe::NativeOptions {
        //options for GUI window
        decorated: false,  //roundede corners
        transparent: true, //no OS-specific bar
        follow_system_theme: false,
        default_theme: eframe::Theme::Light,
        resizable: true,
        ..Default::default()
    };
    //let native_options=eframe::NativeOptions::default();

    //app_name,native_options,app_creator, when usign the custom frame window the name in the first field is not used
    run_native(
        "Screenshot Utility Tool",
        native_options,
        Box::new(|_cc| Box::new(MyApp::new())),
    )
    .expect("A probelem has occurred while starting up!");
    //|_cc| dummy closure, needed to make
    // on the fly function
}
