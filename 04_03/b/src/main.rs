use std::collections::HashMap;
fn main() {
    let contents = String::from("This is the first line\nThe second line is a little longer\nLine 3 is short\nThe 4th line is the first non-prime\nThe 5th line has the starting five");

    let words = get_words(&contents);
    let counter = word_count(&words);

    println!("{:#?}", counter);
}

fn get_words(contents: &str) -> Vec<String> {
    contents
        .split_whitespace()
        .map(|word| word.to_string())
        .collect()
}

fn word_count(words: &Vec<String>) -> HashMap<String, i32> {
    // let mut words_counter: HashMap<String, i32> = HashMap::new();
    let mut words_counter = HashMap::new();

    for word in words {
        words_counter
            .entry(word.to_lowercase())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    words_counter
}
