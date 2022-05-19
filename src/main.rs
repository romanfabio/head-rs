use std::io::{BufRead, BufReader};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {

    #[clap(short='n', long="lines", default_value_t=10)]
    lines: usize,

    #[clap()]
    files: Vec<std::path::PathBuf>
}


fn main() {
    let args = Args::parse();

    let show_header = args.files.len() > 1;

    let mut iter = args.files.into_iter();
    if let Some(first_file) = iter.next() {
        if let Err(e) = head(args.lines, first_file, show_header) {
            eprintln!("{}",e);
            return;
        }
    } else {
        if let Err(e) = head_from_stdin(args.lines) {
            eprintln!("{}",e);
        }
        return;
    }

    for path in iter {
        println!();
        match head(args.lines, path, show_header) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}",e);
                break;
            }
        }
    }
}

fn head(lines: usize, path: std::path::PathBuf, header: bool) -> Result<(), std::io::Error> {

    let file = std::fs::File::open(&path)?;
    let buf = std::io::BufReader::new(file);

    if header {
        if let Some(name) = path.file_name() {
            if let Some(name) = name.to_str() {
                println!("==> {} <==", name);
            }
        }
    }

    print_content(lines, Box::new(buf))
}

fn head_from_stdin(lines: usize) -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();

    print_content(lines, Box::new(BufReader::new(stdin)))
}

fn print_content(lines: usize, mut buf: Box<dyn BufRead>) -> Result<(), std::io::Error> {
    let mut count = 0;
    while count < lines {
        let mut line = String::new();
        match buf.read_line(&mut line) {
            Ok(0) => {break;},
            Ok(_) => {
                print!("{}",line);
            },
            Err(e) => {
                return Err(e);
            }
        }
        count += 1;
    }
    Ok(())
}