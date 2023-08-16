use eframe::egui;
use egui::{ColorImage, Ui};
// use gif::{Encoder, Frame};  servono per save as gif (forse non necessaria)
//use std::fs::File;
use image::{self, ImageFormat};
use screenshots::Screen;

use crate::MyScreen;

struct MyImage {
    texture: Option<egui::TextureHandle>,
}

impl MyImage {
    fn ui(&mut self, ui: &mut egui::Ui, im: ColorImage, size: egui::Vec2) {
        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx().load_texture("my-image", im, Default::default())
        });
        let max_size = egui::vec2(size.x - 20.0, size.y - 44.0).to_pos2();

        let my_rect = egui::Rect::from_two_pos(egui::pos2(10.0, 80.0), max_size);
        ui.painter().image(
            texture.id(),
            my_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }
}

pub fn full_screen() -> Vec<MyScreen> {
    let screens = Screen::all().unwrap();
    let mut screen_image = Vec::new();
    for screen in screens {
        let img = screen.capture().unwrap();
        let image = MyScreen {
            screens: img.rgba().to_vec(),
            size: (img.width() as usize, img.height() as usize),
        };
        screen_image.push(image);
    }
    screen_image
}

pub fn visualize_image(screens: &mut Vec<MyScreen>, ui: &mut Ui, size: egui::Vec2) {
    for image in screens {
        let mut my_image = MyImage { texture: None };
        let im =
            egui::ColorImage::from_rgba_unmultiplied([image.size.0, image.size.1], &image.screens);
        my_image.ui(ui, im, size);
    }
}

pub fn screen_area(screens: &mut Vec<MyScreen>, x0: u32, y0: u32, width: u32, height: u32) {
    let mut screen_image = Vec::new();

    for image in &mut *screens {
        let rgba_img = image::RgbaImage::from_raw(
            image.size.0 as u32,
            image.size.1 as u32,
            image.screens.to_vec(),
        )
        .expect("Errore nella conversione dell'immagine");
        let cropped_img = image::ImageBuffer::from_fn(width, height, |x, y| {
            rgba_img.get_pixel(x0 + x, y0 + y).clone()
        });

        let mut cropped_bytes = Vec::new();

        for pixel in cropped_img.pixels() {
            cropped_bytes.push(pixel[0]); // Red
            cropped_bytes.push(pixel[1]); // Green
            cropped_bytes.push(pixel[2]); // Blue
            cropped_bytes.push(pixel[3]); // Alpha
        }
        let img = MyScreen {
            screens: cropped_bytes,
            size: (width as usize, height as usize),
        };
        screen_image.push(img);
    }
    *screens = screen_image;
}

pub fn save_image(path: &String, screens: &Vec<MyScreen>, format: &String,use_format:bool) {
    let image_format = if format == ".jpg" {
        ImageFormat::Jpeg
    } else if format == ".png" {
        ImageFormat::Png
    } else {
        ImageFormat::Gif
    };

    for image in screens {
        let img_buf = image::ImageBuffer::<image::Rgba<u8>, _>::from_vec(
            image.size.0 as u32,
            image.size.1 as u32,
            image.screens.to_vec(),
        )
        .expect("impossibile creare l'immagine");
        if use_format==true{
            img_buf
            .save_with_format(path.to_string()+format, image_format)
            .expect("impossibile salvare l'immagine");
        }else{
        img_buf
            .save_with_format(path.to_string(), image_format)
            .expect("impossibile salvare l'immagine");
        }
    }
}

// fn save_images_as_gif2(image: screenshots::Image){

//         let mut encoder = gif::Encoder::new(image.rgba().to_vec(), image.width() as u16, image.height() as u16,&[0xFF, 0xFF, 0xFF]).expect("msg");
//         let img_buf = image::ImageBuffer::<image::Rgba<u8>, _>::from_vec(
//             image.width(),
//             image.height(),
//             image.rgba().to_vec(),
//         ).expect("impossibile creare l'immagine");
//         let mut my_img = image::RgbaImage::from(img_buf).into_raw();
//         let mut gif_frame = gif::Frame::from_rgba_speed(
//             image.width() as u16,
//             image.height() as u16,
//             &mut my_img,
//             10, // Adjust the speed (in hundredths of a second) as needed
//         );

//             gif_frame.dispose = gif::DisposalMethod::Background;
//             encoder.write_frame(&gif_frame).expect("Failed to write GIF frame");
//             std::fs::

// }

// fn save_images_as_gif(path: &String, screens: &mut Vec<screenshots::Image>) {
//     // Crea un nuovo file GIF
//     let mut output_file = File::create(path).expect("Impossibile creare il file GIF");

//     // Configura l'encoder GIF
//     let mut encoder = Encoder::new(
//         &mut output_file,
//         screens[0].width() as u16,
//         screens[0].height() as u16,
//         &[0xFF, 0xFF, 0xFF],
//     )
//     .expect("Impossibile creare l'encoder GIF");

//     // Aggiungi i frame all'encoder
//     for image in screens {
//         let image_rgba = image.rgba();
//         let img_buf = image::ImageBuffer::<image::Rgba<u8>, _>::from_vec(
//             image.width(),
//             image.height(),
//             image_rgba.to_vec(),
//         )
//         .expect("impossibile creare l'immagine");

//         let mut my_img = image::RgbaImage::from(img_buf).into_raw();

//         let frame = Frame::from_rgba(
//             image.width() as u16,
//             image.height() as u16,
//             my_img.as_mut_slice(),
//         );
//         encoder
//             .write_frame(&frame)
//             .expect("Impossibile scrivere il frame");
//     }
// }
