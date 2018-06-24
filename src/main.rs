use std::fs::File;
use std::io::BufReader;
use std::env;

use exif::{Tag,Value};

extern crate exif;

#[derive(Debug)]
struct Picture {
    make: String,
    model: String,
    exposure_time: f64,
    aperture: f64,
}

impl Picture {
    fn new() -> Picture {
        Picture {
            make: "".to_string(),
            model: "".to_string(),
            exposure_time: 0_f64,
            aperture: 0_f64,
        }
    }

    fn set_make(&mut self, make: String) {
        self.make = make;
    }

    fn set_model(&mut self, model: String) {
        self.model = model;
    }

    fn set_exposure_time(&mut self, exptime: f64) {
        self.exposure_time = exptime;
    }

    fn set_aperture(&mut self, ap: f64) {
        self.aperture = ap;
    }
}

fn read_raw(filename: &str) -> Result<Picture, String> {
    let file = File::open(&filename).expect("opening file");

    let reader = match exif::Reader::new(&mut BufReader::new(&file)){
        Ok(r) => r,
        Err(_e) => return Err("couldn't read file".to_string()),
    };

    let mut pic = Picture::new();
     
    if let Some(field) = reader.get_field(Tag::ExposureTime, false) {
        match field.value {
            Value::Rational(ref vec) if !vec.is_empty() =>
                pic.set_exposure_time(vec[0].to_f64()),
            _ => {},
        }
    }

    if let Some(field) = reader.get_field(Tag::ApertureValue, false) {
        match field.value {
            Value::Rational(ref vec) if !vec.is_empty() =>
                pic.set_aperture(vec[0].to_f64()),
            _ => {},

        }
    }

    if let Some(field) = reader.get_field(Tag::Make, false) {
        pic.set_make(field.value.display_as(field.tag).to_string())
    }

    if let Some(field) = reader.get_field(Tag::Model, false) {
        pic.set_model(field.value.display_as(field.tag).to_string())
    }

    Ok(pic)
}

fn main() {
    let filename: String = env::args().nth(1).expect("must supply filename");

    let dat = read_raw(&filename).expect("reading file");

    println!("{:?}", dat)

}
