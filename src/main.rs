use std::fs::File;
use std::io::BufReader;
use std::env;

use exif::Tag;

extern crate exif;

#[derive(Debug)]
struct Picture {
    make: String,
    model: String
}

impl Picture {
    fn new() -> Picture {
        Picture {
            make: "".to_string(),
            model: "".to_string(),
        }
    }

    fn set_make(&mut self, make: String) {
        self.make = make;
    }

    fn set_model(&mut self, model: String) {
        self.model = model;
    }
}

fn read_raw(filename: &str) -> Result<Picture, String> {
    let file = File::open(&filename).expect("opening file");

    let reader = match exif::Reader::new(&mut BufReader::new(&file)){
        Ok(r) => r,
        Err(_e) => return Err("couldn't read file".to_string()),
    };

    let mut pic = Picture::new();

    for f in reader.fields() {
            match f.tag {
                Tag::Make => pic.set_make(f.value.display_as(f.tag).to_string() ),
                Tag::Model => pic.set_model(f.value.display_as(f.tag).to_string() ),
                _ => continue
            }
        }

    Ok(pic)
}

fn main() {
    let filename: String = env::args().nth(1).expect("must supply filename");

    let dat = read_raw(&filename).expect("reading file");

    println!("{:?}", dat)

}
