use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut hm: HashMap<String, u32> = HashMap::new();
    let allowed = "abcdefghijklmnopqrstuvwxyz0123456789'".to_string();
    words
        .to_lowercase()
        .replace(where_not(allowed), " ")
        .split_whitespace()
        .map(remove_surrounding_apostrophes)
        .for_each(|w| {
            hm.insert(w.to_string(), *hm.get(&w).unwrap_or(&0) + 1);
        });
    hm
}

fn where_not(allowed: String) -> impl Fn(char) -> bool {
    move |c: char| !allowed.contains(c)
}

fn remove_surrounding_apostrophes(word: &str) -> String {
    remove_beginning_apostrophes(rev(remove_beginning_apostrophes(rev(word.to_string()))))
}

fn remove_beginning_apostrophes(word: String) -> String {
    word.chars().skip_while(|c| c == &'\'').collect()
}

fn rev(word: String) -> String {
    word.chars().rev().collect()
}
