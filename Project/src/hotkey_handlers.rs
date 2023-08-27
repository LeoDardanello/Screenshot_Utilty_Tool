use crate::{MyApp,MyScreen,screenshot};
use native_dialog::MessageDialog;
use std::path::Path;
use std::borrow::Cow;
use std::fs;
use std::thread;

//Hotkey handler for settings mode
pub fn hotkey_handler_mode0(ev:Option<usize>,my_app:&mut MyApp,ui:&mut egui::Ui,frame:&mut eframe::Frame){
    match ev {
        None => {}
        Some(i) => {
            if i == 0 {
                frame.set_window_size(egui::Vec2 { x: 0.0, y: 0.0 });

                my_app.time = ui.input(|i| i.time);
                my_app.area = (None, None);
                my_app.def_paint.clear();
                my_app.paint.clear();
                my_app.edit_image=MyScreen::new(None, None);
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

//Hotkey handler for visualization mode
pub fn hotkey_handler_mode4(ev:Option<usize>,my_app:&mut MyApp,frame:&mut eframe::Frame){
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
                    + &String::from("/screenshot")
                    + &(my_app.default_name_index.to_string()),
            );
            let mut image=& my_app.image[my_app.n_monitor];
            if my_app.edit_image.screens.len()>0{
                image=&my_app.edit_image;
            }
            let dir_path_for_thread=my_app.default_path.clone();
            let image_for_thread = image.clone();
            let output_format_for_thread = my_app.output_format.clone();

            thread::spawn(move || {
                if Path::new(&dir_path_for_thread).exists(){
                    //if dir already exists save the image
                    screenshot::save_image(
                        &path_for_thread,
                        &image_for_thread,
                        &output_format_for_thread,
                        true,
                    );
                    println!("ho finito di salvare");
                }else{
                    let result=fs::create_dir(dir_path_for_thread);
                    match result{
                        Ok(_)=>{//directory created succesffuly, save the image
                            screenshot::save_image(
                                &path_for_thread,
                                &image_for_thread,
                                &output_format_for_thread,
                                true,
                            )},

                        Err(_)=>{println!("Errore nel salvataggio dell'immagine!")}
                    }
                }
             
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