use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let wanted_string = "a";
    let file_path = "file_with_lines";
    let file = File::open(file_path)?;
    let buffer = BufReader::new(file);

    for line in buffer.lines() {
        match line {
            Ok(line) if line.contains(wanted_string) => println!("{}", line),
            _ => continue,
        };
    }

    Ok(())
}
