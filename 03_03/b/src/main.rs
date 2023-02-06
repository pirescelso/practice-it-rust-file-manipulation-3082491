use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file_path = "file_with_lines";

    let lines: Vec<String> =
        read_file(&file_path).expect(&format!("Error reading file <{}>", &file_path));

    println!("{:#?}", lines);
}

fn read_file(file_path: &str) -> Result<Vec<String>, io::Error> {
    let mut file_lines = Vec::new();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // for line in reader.lines() {
    //     file_lines.push(line.unwrap());
    // }

    reader
        .lines()
        .for_each(|line| file_lines.push(line.unwrap()));

    Ok(file_lines)
}
