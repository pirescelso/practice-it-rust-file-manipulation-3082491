use std::fs;

fn main() {
    let file_path = "file_with_lines";

    let lines: Vec<String> =
        read_file(&file_path).expect(&format!("Error reading file <{}>", &file_path));

    println!("{:#?}", lines);
}

fn read_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let contents = fs::read_to_string(file_path)?;

    let lines = contents.split('\n').map(|line| line.to_string()).collect();
    Ok(lines)
}
