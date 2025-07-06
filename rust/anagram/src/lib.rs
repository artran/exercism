use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // Let's calculate this only once
    let word_chars = get_sorted(word);

    let matches = possible_anagrams
        .iter()
        .cloned()
        .filter(|c| is_anagram(word, &word_chars, c));

    HashSet::from_iter(matches)
}

fn get_sorted(word: &str) -> Vec<char> {
    let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();
    word_chars.sort_unstable();

    word_chars
}

fn is_anagram(word: &str, word_chars: &Vec<char>, candidate: &str) -> bool {
    // Early return if the candidate is the same as the word
    if word.to_lowercase() == candidate.to_lowercase() {
        return false;
    }

    *word_chars == get_sorted(candidate)
}
