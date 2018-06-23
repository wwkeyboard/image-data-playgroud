use std::fs::File;
use std::io::BufReader;
use std::env;
extern crate exif;

fn main() {
    let filename: String = env::args().nth(1).expect("must supply filename");

    println!("reading {}", filename);
    let file = File::open(filename).expect("opening file");

    let reader = exif::Reader::new(&mut BufReader::new(&file)).expect("building reader");

    for f in reader.fields() {
            let thumb = if f.thumbnail {"1/"} else {"0/"};
            println!(" {} --{}--: {}", thumb, f.tag, f.value.display_as(f.tag));
        }
}
