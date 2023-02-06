use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file_path = "file_with_lines";
    let file: File = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
    Ok(())
}
