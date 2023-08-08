use screenshots::Screen;
// use std::fs;
use minifb::{Key, Window, WindowOptions};

pub fn full_screen() {
    let screens = Screen::all().unwrap();
    for screen in screens {
        let image = screen.capture().unwrap();
        show_screen(image)
    }
}

fn show_screen(image: screenshots::Image) {
    let image_rgba = image.rgba();
    let new_width= image.width() as f32 * 0.3;
    let new_heigth= image.height() as f32 * 0.3;

    let mut image_data: Vec<u32> = Vec::new();
    for pixel in image_rgba.chunks_exact(4) {
        let u32_pixel = ((pixel[3] as u32) << 24)
            | ((pixel[0] as u32) << 16)
            | ((pixel[1] as u32) << 8)
            | (pixel[2] as u32);
        image_data.push(u32_pixel);
    }

    let mut window = Window::new(
        "Rust Image Viewer",
        new_width as usize,
        new_heigth as usize,
        WindowOptions::default()
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.set_position(300, 10);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&image_data, image.width()  as usize, image.height() as usize)
            .expect("Impossibile aggiornare la finestra");
    }
}

// fn screen_area(x: u8 ,y: u8, width: u8,eigth: u8){
//     let screens = Screen::all().unwrap();
//     for screen in screens {
//         let  image = screen.capture_area(x,y,widt,heigth).unwrap();
//         let  buffer = image.to_png(None).unwrap();
//         fs::write(format!("./{}-2.png", screen.display_info.id), buffer).unwrap();
//     }
// }
