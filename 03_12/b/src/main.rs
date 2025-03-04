use std::fs;

fn read_file(file_path: &str) -> Result<Vec<Vec<String>>, std::io::Error> {
    let contents = fs::read_to_string(file_path)?;

    let lines: Vec<Vec<String>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.to_string())
                .collect()
        })
        .collect();

    Ok(lines)
}

fn main() {
    let file_path = "file_with_lines";

    let lines = read_file(&file_path).expect(&format!("Unable to read file <{}>", &file_path));

    // println!("{:?}", lines);

    // lines.iter().for_each(|line| println!("{:?}", line));
    lines.iter().for_each(|line| println!("{:?}", line));
}
