use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let matches = possible_anagrams
        .iter()
        .cloned()
        .filter(|c| is_anagram(word, c));

    HashSet::from_iter(matches)
}

fn is_anagram(_word: &str, _candidate: &str) -> bool {
    false
}
