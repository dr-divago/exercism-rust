use std::{collections::HashSet, iter::Map};

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowered_word = word.to_lowercase();
    let sorted_word = sort_word(&lowered_word);
    possible_anagrams.iter()
        .filter(|anagram| {
            let lowered_anagram = anagram.to_lowercase();
            lowered_anagram != lowered_word && sort_word(&lowered_anagram) == sorted_word
        })
        .cloned()
        .collect()
}

fn sort_word(lowered_case_word: &str) -> Vec<char> {
    let mut chars: Vec<char> = lowered_case_word.chars().collect();
    chars.sort_unstable();
    chars
}
