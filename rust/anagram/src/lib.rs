use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // Let's calculate this only once
    let mut word_chars: Vec<char> = word.chars().collect();
    word_chars.sort();

    let matches = possible_anagrams
        .iter()
        .cloned()
        .filter(|c| is_anagram(&word_chars, c));

    HashSet::from_iter(matches)
}

fn is_anagram(word_chars: &Vec<char>, candidate: &str) -> bool {
    let mut candidate_chars: Vec<char> = candidate.chars().collect();
    candidate_chars.sort();

    candidate_chars == *word_chars
}
