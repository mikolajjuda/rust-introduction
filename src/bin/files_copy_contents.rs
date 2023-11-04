use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let argv: Vec<_> = std::env::args().collect();
    if argv.len() < 3 {
        eprintln!("Usage: {} in_file out_file", argv[0]);
        std::process::exit(1);
    }
    let mut in_f = match File::open(&argv[1]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening input file: {}", e);
            std::process::exit(1);
        }
    };
    let mut out_f = match File::create(&argv[2]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening output file: {}", e);
            std::process::exit(1);
        }
    };
    let mut buffer = [0u8; 1024];
    loop {
        match in_f.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                match out_f.write(&buffer[..bytes_read]) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("Error writing output file: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input file: {}", e);
                std::process::exit(1);
            }
        }
    }

    if let Err(e) = out_f.sync_all() {
        eprintln!("Error syncing output file: {}", e);
        std::process::exit(1);
    }
}
