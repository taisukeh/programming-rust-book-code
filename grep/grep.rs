use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::error::Error;
use std::path::PathBuf;

fn grep<T>(target: &str, reader: T) -> io::Result<bool> where T: BufRead {
    let mut found = false;
    let stdout = io::stdout();
    let mut lock = stdout.lock();

    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            writeln!(lock, "{}", line);
            found = true;
        }
    }
    Ok(found)
}


fn grep_main() -> Result<(), Box<Error>> {
    let mut args = std::env::args().skip(1);

    let target = match args.next() {
        Some(s) => s,
        None => Err("usage: grep PATTERN FILE...")?,
    };
        
    let files: Vec<_> = args.map(PathBuf::from).collect();

    if files.is_empty() {
        let stdin = io::stdin();
        grep(&target, stdin.lock())?;
    } else {
        for file in files {
            let f = File::open(file)?;
            grep(&target, BufReader::new(f))?;
        }
    }
    Ok(())
}

fn main() {
    match grep_main() {
        Ok(_) => (),
        Err(e) => {
            writeln!(io::stderr(), "Error: {}", e);
            std::process::exit(1);
        }
    }
}
