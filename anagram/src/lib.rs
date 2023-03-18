use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let lowered_case_word = word.to_lowercase();
    let sorted_word = sort_word(&lowered_case_word);
    for anagram in possible_anagrams {
        let lowered_case_anagram = anagram.to_lowercase();
        let sorted_anagram = sort_word(&lowered_case_anagram);
        if sorted_anagram == sorted_word && lowered_case_anagram != lowered_case_word {
            anagrams.insert(*anagram);
        }
    }
    anagrams
}

fn sort_word(lowered_case_word: &str) -> String {
    let mut chars: Vec<char> = lowered_case_word.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}
