use std::collections::HashMap;
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

fn replace_main_actor(
    mut lines: Vec<Vec<String>>,
    replacement_map: &HashMap<String, String>,
) -> Vec<Vec<String>> {
    for words in lines.iter_mut() {
        for word in words.iter_mut() {
            if let Some(new_word) = replacement_map.get(word) {
                *word = new_word.to_string();
            }
        }
    }

    lines
}

fn word_count(lines: &Vec<Vec<String>>) -> HashMap<String, i32> {
    let mut hashmap: HashMap<String, i32> = HashMap::new();

    lines.iter().for_each(|line| {
        line.iter().for_each(|word| {
            hashmap
                .entry(word.to_string())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        })
    });

    hashmap
}

fn build_lines(old_lines: &Vec<Vec<String>>) -> Vec<String> {
    let mut new_lines: Vec<String> = Vec::new();

    for old_line in old_lines {
        let new_line = old_line.join(" ");
        new_lines.push(new_line);
    }

    new_lines
}

fn write_lines_to_file(file_path: &str, lines: &[String]) -> Result<(), std::io::Error> {
    let contents = lines.join("\n");

    fs::write(file_path, contents)?;

    Ok(())
}

fn main() {
    let replacement_map = HashMap::from([
        ("herself".to_string(), "himself".to_string()),
        ("herself,".to_string(), "himself,".to_string()),
        ("her.".to_string(), "him.".to_string()),
        ("she".to_string(), "he".to_string()),
        ("(she".to_string(), "(he".to_string()),
        ("her".to_string(), "his".to_string()),
        ("Alice's".to_string(), "Celso's".to_string()),
        ("Alice!".to_string(), "Celso!".to_string()),
        ("Alice,".to_string(), "Celso,".to_string()),
        ("Alice;".to_string(), "Celso;".to_string()),
        ("She".to_string(), "He".to_string()),
        ("(Alice".to_string(), "(Celso".to_string()),
        ("Alice,)".to_string(), "Celso,)".to_string()),
        ("she'll".to_string(), "he'll".to_string()),
        ("she’ll".to_string(), "he’ll".to_string()),
        ("Alice".to_string(), "Celso".to_string()),
        ("her,".to_string(), "him,".to_string()),
        ("Alice’s".to_string(), "Celso’s".to_string()),
        ("girl".to_string(), "boy".to_string()),
    ]);

    // Format data
    let file_path = "alice_chapter_1";
    let lines =
        read_file(file_path).unwrap_or_else(|_| panic!("Error reading file <{}>", &file_path));

    // Manipulate data
    let new_lines = replace_main_actor(lines, &replacement_map);

    // Used for Discovery
    let counter = word_count(&new_lines);
    println!("{:#?}", counter);

    // Reconstruct data
    let vec_of_lines = build_lines(&new_lines);

    // Write data to file
    let file_path = "celso_chapter_1";
    write_lines_to_file(file_path, &vec_of_lines).expect("Error writing file");
}
