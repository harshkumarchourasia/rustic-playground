use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let sorted_word = get_sorted(&lower_word);
    possible_anagrams
        .iter()
        .filter(
            |candidate| {
                let lower_candidate = candidate.to_lowercase();
                lower_candidate.len() == lower_word.len()
                    && lower_candidate != lower_word
                    && get_sorted(&lower_candidate) == sorted_word
            }
        ).copied()
        .collect()

}

fn get_sorted(word: &str) -> Vec<char>{
    let mut word_sorted: Vec<char> = word.chars().collect();
    word_sorted.sort_unstable();
    word_sorted
}