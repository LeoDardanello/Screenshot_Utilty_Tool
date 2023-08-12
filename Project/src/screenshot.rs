use screenshots::Screen;
use std::cmp::Ordering;
use egui::{Ui, ColorImage, Image, Order};
use eframe::egui;
use image::{self, ImageFormat};
use png;
use gif;


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
   
        let max_size= egui::vec2(size.x-20.0, size.y-44.0).to_pos2();
        
        let my_rect= egui::Rect::from_two_pos(egui::pos2(10.0,  34.0), max_size);
        ui.painter().image(texture.id(),my_rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);

    
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

            
        }
    }

pub fn save_image(screens: &mut Vec<screenshots::Image>, format: &mut String){
    if format== ".gif"{
        *format=String::from(".png");//problema con .gif
    }
    let image_format= if format == ".jpg" {
        ImageFormat::Jpeg
    } else {
        ImageFormat::Png
    } ;
    for image in screens {
        let image_rgba= image.rgba();
        let img_buf=image::ImageBuffer::<image::Rgba<u8>, _>::from_vec(image.width(), image.height(), image_rgba.to_vec()).expect("impossibile creare l'immagine");
        img_buf.save_with_format("./image".to_string()+&(*format).to_string(), image_format).expect("impossibile salvare l'immagine");
    }

}
