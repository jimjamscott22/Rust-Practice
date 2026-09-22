pub fn word_counts(text: &str) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_owned()).or_insert(0) += 1;
    }
    counts
}
