pub fn abbreviate(phrase: &str) -> String {
    phrase
        .chars()
        .filter(|c| *c != '\'')
        .collect::<String>()
        .split(|c: char| !c.is_alphabetic())
        .map(|word| -> String {
            let uppers: String = word
                .chars()
                .filter(|c| *c == c.to_ascii_uppercase())
                .collect();
            if uppers == word {
                uppers.chars().take(1).collect()
            } else if uppers == "" {
                word.chars()
                    .take(1)
                    .map(|c| c.to_ascii_uppercase())
                    .collect()
            } else {
                uppers
            }
        })
        .collect()
}
