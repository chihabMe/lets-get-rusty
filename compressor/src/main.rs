extern  crate flate2;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;


fn main() {
    if args().len()<3{
        println!("-------------------------\n");
        println!("Usage: `file path` `name` \n");
        eprintln!("Example: ./mgcomp hello.py hello \n");
        println!("-------------------------");
        return ;
    }
    let filename = args().nth(1).unwrap_or_else(||{
        eprintln!("a file path is required");
        std::process::exit(1);
    });
    println!("{}",filename);
    let file = File::open(filename).unwrap_or_else(|error|{
        println!("{}",error);
        std::process::exit(1);
    });
    println!("{:?}",file);
    // // let mut file_buffer = BufReader::new(inner)
}
