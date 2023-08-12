use minifb::{Key, KeyRepeat,/*  MouseButton, MouseMode,*/ Window, WindowOptions};
use screenshots::Screen;
use std::fs;
use egui::{Ui, ColorImage, /*TextureId*/};
use eframe::egui;
// use std::collections::HashMap;

struct MyImage {
    texture: Option<egui::TextureHandle>,
}

impl MyImage {
    

    fn ui(&mut self, ui: &mut egui::Ui, im: ColorImage, size: egui::Vec2) {
        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx().load_texture(
                "my-image",
                im,
                Default::default()
            )
        });
        // Show the image: 
        // ui.add();

        // // Shorter version:
        let max_size= egui::vec2(size.x*0.5, size.y*0.5).to_pos2();
        
        let my_rect= egui::Rect::from_two_pos(egui::pos2(10.0,  34.0), max_size);
        ui.painter().image(texture.id(),my_rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);

        // ui.add_sized(size, egui::Image::new(texture, texture.size_vec2()));
    
    }
}


pub fn full_screen( )->Vec<screenshots::Image>{
    
    
    let screens= Screen::all().unwrap();
    let mut screen_image= Vec::new();
    for screen in screens {
       screen_image.push(screen.capture().unwrap());

        
    }
    screen_image

}

pub fn visualize_image(screens: &mut Vec<screenshots::Image>, ui: &mut Ui, size: egui::Vec2){
    
    for image in screens {

            let image_rgba= image.rgba();
            let mut my_image = MyImage {
                texture: None,
            };
            let im=egui::ColorImage::from_rgba_unmultiplied([image.width() as usize, image.height() as usize], image_rgba);
            my_image.ui(ui, im, size);

     
            // let buffer = image.to_png(None).unwrap();
            // fs::write(format!("./{}.png", screen.display_info.id), buffer).unwrap();
            
    
                
            // show_screen(screen, ui);
            
        }
    }



// fn show_screen(screen: screenshots::Screen, ui: &mut Ui) {
//     let mut image = screen.capture().unwrap();
//     let mut image_data = visualize_image(&image);

//     let mut window = Window::new(
//         "Rust Image Viewer",
//         image.width() as usize,
//         image.height() as usize,
//         WindowOptions::default(),
//     )
//     .unwrap_or_else(|e| {
//         panic!("{}", e);
//     });
//     window.set_position(10, 10);


//     while window.is_open() && !window.is_key_down(Key::Escape) {
//         window
//             .update_with_buffer(&image_data, image.width() as usize, image.height() as usize)
//             .expect("Impossibile aggiornare la finestra");

//         if window.is_key_pressed(Key::S, KeyRepeat::No) {
//             let buffer = image.to_png(None).unwrap();
//             fs::write(format!("./{}.png", screen.display_info.id), buffer).unwrap();
//         }

//         if window.is_key_pressed(Key::A, KeyRepeat::No) && !window.is_key_down(Key::X) {
//             println!("Screenshot di un'area");
            
//         image = screen_area(screen, &mut image_data);
//     } 

//         if window.is_key_pressed(Key::C, KeyRepeat::No) {
//             println!("Copia");
//         }

//         if window.is_key_pressed(Key::M, KeyRepeat::No) {
//             println!("Modifica");
//         }
//         window.update()
//     }
// }

// fn screen_area(screen: screenshots::Screen, image_data: &mut Vec<u32>) -> screenshots::Image {
//     let dimensions = (10, 20, 150, 140);
//     let area = screen
//         .capture_area(dimensions.0, dimensions.1, dimensions.2, dimensions.3)
//         .unwrap();
//     *image_data = visualize_image(&area);
//     area
// }

// fn find_dimension(window: MutexGuard<minifb::Window>) ->  Option<(i32, i32, u32, u32)> {
//     let mut d: (i32, i32, u32, u32)=(0,0,0,0);
//     while window.get_mouse_down(MouseButton::Left) {
//         if d.0==0 && d.1==0 {
//             let pos = window.get_mouse_pos(MouseMode::Clamp);
//             if let Some((x, y)) = pos {
//                 d.0 = x as i32;
//                 d.1 = y as i32;
//             }
//         }
//     }
//     if (d.0!=0 || d.1!=0) && d.2==0 && d.3==0{
//         let pos = window.get_mouse_pos(MouseMode::Clamp);
//             if let Some((x, y)) = pos {
//                 d.2 = (d.0- (x as i32)) as u32;
//                 d.3 = (d.1-(y as i32)) as u32;
//             }
//     }

//     Some(d)
// }
