use crate::utils::strings::str_last_char;
use crate::vocab::common::{PUNCTUATION, UNWANTED_CONJUGATES};

use super::prep::{clean_word, word_without_extensions};

/// From text extract individual words or phrases into a vector
pub fn phrase_extraction(text: &str) -> Vec<Vec<&str>> {
    let words: Vec<&str> = text.split(&[' ', '[', ']', '(', ')']).collect();
    let mut phrases_out: Vec<Vec<&str>> = vec![vec![]];
    let mut last_title_or_name: Vec<&str> = vec![];

    for word in words {
        if word.chars().count() == 0 {
            continue;
        }
        if word.chars().count() < 2 && word.chars().by_ref().any(|c| c.is_alphanumeric() == false) {
            continue;
        }

        let word_lower = word.to_lowercase();
        let word_lower_str = word_lower.as_str();
        if PUNCTUATION.contains(&word_lower_str) {
            continue;
        }

        // println!("chars {:?}", word.chars().as_str());

        let last_char = str_last_char(word);
        let last_in_sentence = last_char == '.' || last_char == ',';

        // TODO: Move the word cleansing to another function
        let word_clean: &str = clean_word(word).unwrap_or(word);
        if word_clean.is_empty() {
            continue;
        }

        // println!("word {:?}", word_clean);

        // if it's a single word phrase and it's a conjugate we don't want it as a label
        // I expect it to be faster to just add and remove these words from the list instead
        // since it's alphabetical, we can find the 1st conjugate in the list and remove it,
        // and then continue searching the list from that index for the 2nd conjugate...
        if UNWANTED_CONJUGATES.contains(&word_clean) == false {
            phrases_out.push(vec![word_without_extensions(word_clean).unwrap()]);
        }

        // Here we can create an iterator and slice the slice
        // based on where each unwanted character is
        // reconstructing a new &partial[partial..] ex unwanted
        // (although that may not even be necessary)

        // does not handle last word in text as part of name
        let is_title_case = word.chars().nth(0).unwrap().is_uppercase();
        if last_in_sentence && is_title_case {
            last_title_or_name.push(word_clean);
            phrases_out.push(last_title_or_name.to_vec());
            last_title_or_name.clear();
        } else if is_title_case {
            last_title_or_name.push(word_clean);
        } else if last_title_or_name.len() > 0 {
            // let combined_word = last_title_or_name.to_owned().join(" ").as_str();
            phrases_out.push(last_title_or_name.to_vec());
            last_title_or_name.clear();
        }
    }

    phrases_out
}
