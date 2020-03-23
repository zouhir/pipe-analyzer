use clap::{App, Arg};
use std::fs::File;
use std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

const CHUNK_SIZE: usize = 16 * 1024; //16kb chunk size

fn main() -> Result<()> {
    let arg_matches = App::new("pipe-analyzer")
        .arg(Arg::with_name("in").help("Read from a file instead of stdin"))
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .takes_value(true)
                .help("Write output to file instead of stdout"),
        )
        .arg(Arg::with_name("silent").short("s").long("silent"))
        .get_matches();

    let in_file = arg_matches.value_of("in").unwrap_or_default();
    let out_file = arg_matches.value_of("out").unwrap_or_default();

    let silent = if arg_matches.is_present("silent") {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };

    //dbg!(silent, in_file, out_file);

    let mut reader: Box<dyn Read> = if !in_file.is_empty() {
        Box::new(File::open(in_file)?)
    } else {
        Box::new(io::stdin())
    };

    let mut writer: Box<dyn Write> = if !out_file.is_empty() {
        Box::new(File::create(out_file)?)
    } else {
        Box::new(io::stdout())
    };

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        if !silent {
            eprint!("\rtotal bytes: {}", total_bytes);
        }
        //write all accepts a slice of bytes

        // TIP!: error check block can be replaced with question mark if you dont need to check for something special
        // io::stdout().write_all(&buffer[..num_read])?

        if let Err(e) = writer.write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }

            // eprintln!("oh shee, error happened {}", e.to_string());
            // std::process::exit(1);
            return Err(e);
        }
    }
    if !silent {
        eprintln!("\rtotal bytes: {}", total_bytes);
    }
    Ok(())
}
