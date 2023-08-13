use eframe::egui;
use egui::{ColorImage, Ui};
use gif::{Encoder, Repeat, Frame};
use image::{self, ImageFormat, Pixel};
use screenshots::Screen;
use std::fs::File;

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

        let my_rect = egui::Rect::from_two_pos(egui::pos2(10.0, 34.0), max_size);
        ui.painter().image(
            texture.id(),
            my_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }
}

pub fn full_screen() -> Vec<screenshots::Image> {
    let screens = Screen::all().unwrap();
    let mut screen_image = Vec::new();
    for screen in screens {
        screen_image.push(screen.capture().unwrap());
    }
    screen_image
}

pub fn visualize_image(screens: &mut Vec<screenshots::Image>, ui: &mut Ui, size: egui::Vec2) {
    for image in screens {
        let image_rgba = image.rgba();
        let mut my_image = MyImage { texture: None };
        let im = egui::ColorImage::from_rgba_unmultiplied(
            [image.width() as usize, image.height() as usize],
            image_rgba,
        );
        my_image.ui(ui, im, size);
    }
}

pub fn save_image(screens: &mut Vec<screenshots::Image>, format: &mut String) {

    let image_format = if format == ".jpg" {
        ImageFormat::Jpeg
    } else if format==".png"{
        ImageFormat::Png
    }else{
        ImageFormat::Gif
    };
if format==".gif"{
    save_images_as_gif(screens)
}
else{


for image in screens {
        let image_rgba = image.rgba();
        let img_buf = image::ImageBuffer::<image::Rgba<u8>, _>::from_vec(
            image.width(),
            image.height(),
            image_rgba.to_vec(),
        )
        .expect("impossibile creare l'immagine");

            img_buf
                .save_with_format("./image".to_string() + &(*format).to_string(), image_format)
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

fn save_images_as_gif(screens: &mut Vec<screenshots::Image>) {
    // Crea un nuovo file GIF
    let mut output_file = File::create("./image.gif").expect("Impossibile creare il file GIF");


    // Configura l'encoder GIF
    let mut encoder = Encoder::new(&mut output_file, screens[0].width() as u16, screens[0].height() as u16, &[0xFF, 0xFF, 0xFF]).expect("Impossibile creare l'encoder GIF");

    // Aggiungi i frame all'encoder
    for image in screens {
        let image_rgba = image.rgba();
        let img_buf = image::ImageBuffer::<image::Rgba<u8>, _>::from_vec(
            image.width(),
            image.height(),
            image_rgba.to_vec(),
        ).expect("impossibile creare l'immagine");
   
        let mut my_img = image::RgbaImage::from(img_buf).into_raw();


        let frame = Frame::from_rgba(image.width() as u16, image.height() as u16, my_img.as_mut_slice());
        encoder.write_frame(&frame).expect("Impossibile scrivere il frame");
    
    }


}