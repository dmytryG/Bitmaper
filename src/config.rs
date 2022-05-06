use std::env;


pub struct Config {
    pub filename: String,
    pub pixel_size: u8,
    pub layers: Vec<String>
}

impl Config {
    pub fn get_config() -> Config {
        let mut conf = env::args().into_iter().collect::<Vec<String>>();
        let mut conf_iter = conf.iter();

        let _ = conf_iter.next().clone();
        let filename = conf_iter.next().unwrap().clone();
        let pixel_size: u8 = conf_iter.next().unwrap().clone().parse().unwrap();

        let mut layers: Vec<String> = vec![];

        while let Some(i) = conf_iter.next() {
            layers.push(i.clone());
        }
        Self::new(filename, pixel_size, layers)
    }

    pub fn new(filename: String, pixel_size: u8, layers: Vec<String>) -> Config {
        Self {
            filename,
            pixel_size,
            layers
        }
    }
}