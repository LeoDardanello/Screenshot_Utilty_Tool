use minifb::{Key, KeyRepeat,/*  MouseButton, MouseMode,*/ Window, WindowOptions};
use screenshots::Screen;
use std::fs;
use egui::{Ui, /*TextureId*/};
use eframe::egui;
// use std::collections::HashMap;


pub fn full_screen(ui: &mut Ui) {
    
    
    let screens = Screen::all().unwrap();
    for screen in screens {
        let image = screen.capture().unwrap();
        let buffer = image.to_png(None).unwrap();
            fs::write(format!("./{}.png", screen.display_info.id), buffer).unwrap();
        // show_screen(screen, ui);
        
    }
    
}

fn visualize_image(image: &screenshots::Image) -> Vec<u32> {
    let image_rgba= image.rgba();

    // let mut textures: HashMap<TextureId, egui::Texture> = HashMap::new();
    // let texture_id = egui::TextureId::User(0);
    // let texture = egui::Texture {
    //     width: image.width() as usize,
    //     height: image.height() as usize,
    //     pixels: image.rgba()
    // };
        // textures.insert(texture_id, texture);

    let mut image_data: Vec<u32> = Vec::new();
    for pixel in image_rgba.chunks_exact(4) {
        let u32_pixel = ((pixel[3] as u32) << 24)
            | ((pixel[0] as u32) << 16)
            | ((pixel[1] as u32) << 8)
            | (pixel[2] as u32);
        image_data.push(u32_pixel);
    }
    image_data
}


fn show_screen(screen: screenshots::Screen, ui: &mut Ui) {
    let mut image = screen.capture().unwrap();
    let mut image_data = visualize_image(&image);

    let mut window = Window::new(
        "Rust Image Viewer",
        image.width() as usize,
        image.height() as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.set_position(10, 10);


    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&image_data, image.width() as usize, image.height() as usize)
            .expect("Impossibile aggiornare la finestra");

        if window.is_key_pressed(Key::S, KeyRepeat::No) {
            let buffer = image.to_png(None).unwrap();
            fs::write(format!("./{}.png", screen.display_info.id), buffer).unwrap();
        }

        if window.is_key_pressed(Key::A, KeyRepeat::No) && !window.is_key_down(Key::X) {
            println!("Screenshot di un'area");
            
        image = screen_area(screen, &mut image_data);
    } 

        if window.is_key_pressed(Key::C, KeyRepeat::No) {
            println!("Copia");
        }

        if window.is_key_pressed(Key::M, KeyRepeat::No) {
            println!("Modifica");
        }
        window.update()
    }
}

fn screen_area(screen: screenshots::Screen, image_data: &mut Vec<u32>) -> screenshots::Image {
    let dimensions = (10, 20, 150, 140);
    let area = screen
        .capture_area(dimensions.0, dimensions.1, dimensions.2, dimensions.3)
        .unwrap();
    *image_data = visualize_image(&area);
    area
}

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
