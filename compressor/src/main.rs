use std::error::Error;
use std::io::{BufReader, copy};
use std::time::Instant;
use flate2::{write::GzEncoder, Compression};
use std::fs::File;
use std::env::args;

fn compress_file() -> Result<(), Box<dyn Error>> {
    let mut output_file_name = args().nth(2).ok_or("invalid file name").unwrap();
    if !output_file_name.contains(".gz"){
        output_file_name+=".gz"
    }
    let file = File::open(args().nth(1).ok_or("No input file provided")?)?;
    let mut file_buffer = BufReader::new(file);

    let output = File::create(output_file_name)?;
    let start = Instant::now();

    let mut encoder = GzEncoder::new(output, Compression::default());
    copy(&mut file_buffer, &mut encoder)?;

    let result = encoder.finish()?;
    println!("source len: {:?}", file_buffer.get_ref().metadata()?.len());
    println!("compressed len: {:?}", result.metadata()?.len());
    println!("time:{:?}", start.elapsed());

    Ok(())
}

fn main() {
    if let Err(error) = compress_file() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
