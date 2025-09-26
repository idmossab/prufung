use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut res =HashMap::new();
    for word in words.to_lowercase()
    .split(|ch :char| !ch.is_alphanumeric() && ch !='\'')
    .map(|ch| ch.trim_matches('\''))
    .filter(|ch| !ch.is_empty()){
        *res.entry(word.to_string()).or_insert(0)+=1;
    }
    res
}