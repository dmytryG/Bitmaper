mod config;
mod leveler;
mod fronter;

use image::{self, open, image_dimensions};
use crate::fronter::{generate_HTML, generate_CSS};

fn main() {
    let mut config = config::Config::get_config();
    let mut layer_number = 1;
    let mut layers: Vec<(String, String)> = vec![]; // (name, class)
    for layer in config.layers {
        let img = image::open(layer.as_str()).unwrap().to_rgba8();
        let pic = leveler::picture_to_layer(img, config.pixel_size.clone(), layer.clone().split('.').next().unwrap().to_string(), layer_number);
        println!("{:#?}", pic);
        layers.push((layer, pic));
        layer_number = layer_number + 1;
    }
    std::fs::write(format!("{}.html", config.filename), generate_HTML(layers.clone(), config.filename.clone()));
    std::fs::write(format!("{}.css", config.filename), generate_CSS(layers.clone()));
}
