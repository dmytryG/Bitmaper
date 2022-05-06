use std::fmt::format;
use image::{self, open, image_dimensions, RgbImage, RgbaImage};



pub fn picture_to_layer(img: RgbaImage, pixel_size: u8, layer_name: String, layer_number: u8) -> String {
    let pixels = img.pixels();
    let (widht, height) = img.dimensions();
    let mut posX = 0;
    let mut posY = 0;
    let mut res = String::new();
    for pixel in pixels {
        res = res + get_pixel_code(pixel.0.to_vec(), (posX, posY), pixel_size).as_str();
        if (posY+1 == height as i32) && (posX+1 == widht as i32) {

        } else {
            res = res + ", ";
        }
        posX = posX + 1;
        if (posX >= widht as i32) {
            posX = 0;
            posY = posY + 1;
        }
    }
    get_css(res, layer_name, pixel_size, ((layer_number * 10) as i8))

}

fn get_color_code(color: Vec<u8>) -> String{
    format!("#{:X?}{:X?}{:X?}", color.get(0).unwrap(), color.get(1).unwrap(), color.get(2).unwrap()).to_string()
}

fn get_color_code_rgba(color: Vec<u8>) -> String{
    format!("rgba({},{},{},{})", color.get(0).unwrap(), color.get(1).unwrap(), color.get(2).unwrap(), (color.get(3).unwrap() / 255) as f32).to_string()
}
fn get_pixel_code(pixcel: Vec<u8>, position: (i32, i32), pixel_size: u8) -> String {
    format!("{}px {}px {}",
            position.0 * pixel_size as i32 + pixel_size as i32,
            position.1 * pixel_size as i32 + pixel_size as i32,
            get_color_code_rgba(pixcel.clone()))
}
fn get_css(pixel_code: String, layer: String, pixel_size: u8, layer_number: i8) -> String {
    format!(".{} {}\nbackground: #fff; \nheight: {}px; \nleft: 50%; \nposition: absolute; \ntop: 0px; \nwidth: {}px; \nz-index: {}; \nbox-shadow: {}; \n{}"
            , layer, "{", pixel_size, pixel_size, layer_number, pixel_code, "}")
}