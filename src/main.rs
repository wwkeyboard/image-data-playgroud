use std::fs::File;
use std::io::BufReader;
extern crate exif;

fn main() {
    println!("Hello, world!");

    let filename = "test.ARW";
    let file = File::open(filename).expect("opening file");

    let reader = exif::Reader::new(&mut BufReader::new(&file)).expect("building reader");

    for f in reader.fields() {
        
            let thumb = if f.thumbnail {"1/"} else {"0/"};
            println!(" {} --{}--: {}", thumb, f.tag, f.value.display_as(f.tag));
        }
}
