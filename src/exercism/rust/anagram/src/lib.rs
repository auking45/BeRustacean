use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let mut sorted_word = lowercase_word.chars().collect::<Vec<char>>();
    sorted_word.sort_unstable();

    let mut output: HashSet<&'a str> = HashSet::new();

    for &s in possible_anagrams.iter() {
        let lowercase_anagram = s.to_lowercase();
        let mut sorted_anagram = lowercase_anagram.chars().collect::<Vec<char>>();
        sorted_anagram.sort_unstable();

        if sorted_word == sorted_anagram {
            if lowercase_word != lowercase_anagram {
                output.insert(s);
            }
        }
    }

    output
}
