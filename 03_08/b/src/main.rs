use std::fs;

fn main() {
    let file_path = "words_to_file";
    let words = vec![
        "Words".to_string(),
        "of".to_string(),
        "the".to_string(),
        "first".to_string(),
        "line.".to_string(),
    ];

    write_words_to_file(&file_path, &words).unwrap();
}

fn write_words_to_file(file_path: &str, words: &Vec<String>) -> Result<(), std::io::Error> {
    let text: String = words.join(" ");
    fs::write(file_path, text)?;
    Ok(())
}
