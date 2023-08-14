use eframe::{egui, run_native};
use hotkeys::HotkeysConfig;
mod gui;
mod hotkeys;
mod screenshot;

pub struct MyScreen {
    screens: Vec<u8>,
    size: (usize, usize),
}

pub struct MyApp {
    hotkey_conf: HotkeysConfig,
    output_format: String,
    mode: i32,
    image: Vec<MyScreen>,
    default_name_index:i32,
    delay_time:u32,
    enable_screenshot:bool,
    default_path:String
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
            default_name_index:0,
            delay_time:0,
            enable_screenshot:true,
            default_path:String::from("./../screenshot_default")//default screenshot save location, used by save hotkey
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
        } else if self.mode == 1 {
            self.mode = 2;
        } else if self.mode == 2 {
            self.mode = 3;
            self.image = screenshot::full_screen();
            frame.set_visible(true);
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
