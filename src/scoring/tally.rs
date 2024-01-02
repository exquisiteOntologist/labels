use crate::utils::vecs::vec_eq;

/// Count Similars. Requires the words to vec passed in to be sorted alphabetical.
pub fn tally_words<'a>(words_wanted: &Vec<Vec<&'a str>>) -> Vec<(Vec<&'a str>, i32)> {
    let mut word_tallies: Vec<(Vec<&str>, i32)> = vec![];
    let mut prior_word: Vec<&str> = vec![];
    let mut tally: i32 = 0;
    for word in words_wanted {
        // Doesn't handle last word in text
        if vec_eq(&word, &prior_word) {
            tally += 1;
        } else {
            word_tallies.push((prior_word, tally));
            tally = 0;
        }
        prior_word = word.to_vec();
    }
    word_tallies
}
