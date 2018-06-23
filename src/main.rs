use std::fs::File;
use std::io::BufReader;
use std::env;
extern crate exif;


fn read_raw(filename: &str) -> Result<String, String> {
    let file = File::open(&filename).expect("opening file");

    let reader = match exif::Reader::new(&mut BufReader::new(&file)){
        Ok(r) => r,
        Err(e) => return Err("couldn't read file".to_string()),
    };

    let mut accum = String::new();

    for f in reader.fields() {
            let thumb = if f.thumbnail {"1/"} else {"0/"};
            accum += &format!(" {} --{}--: {}\n", thumb, f.tag, f.value.display_as(f.tag));
        }

    Ok(accum)
}

fn main() {
    let filename: String = env::args().nth(1).expect("must supply filename");

    let dat = read_raw(&filename).expect("reading file");

    println!("{}", dat)

}
