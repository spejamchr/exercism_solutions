use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let target_chars = sorted_chars(&word);

    possible_anagrams
        .iter()
        .fold(HashSet::new(), |mut acc, candidate| {
            if candidate.to_lowercase() != word && target_chars == sorted_chars(candidate) {
                acc.insert(candidate);
            }
            acc
        })
}

fn sorted_chars(word: &str) -> Vec<char> {
    let mut chars: Vec<_> = word.to_lowercase().chars().collect();
    chars.sort_unstable();
    chars
}
