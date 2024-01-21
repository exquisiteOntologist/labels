use crate::utils::strings::str_last_char;
use crate::vocab::common::UNWANTED_CONJUGATES;

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

        // we don't need this because we are already filtering out non-alphanumerics above
        // if word.len() == 1 && PUNCTUATION.contains(&word) {
        //     // may be better just to check if it's alpha-numeric?
        //     continue;
        // }

        let last_char = str_last_char(word);
        let last_in_sentence = last_char == '.' || last_char == ',';

        let word_clean: &str = clean_word(word).unwrap_or(word);
        if word_clean.is_empty() {
            continue;
        }

        phrases_out.push(vec![word_without_extensions(word_clean).unwrap()]);

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

    phrases_out.sort();

    // we don't want conjugates as labels
    phrases_out = sans_conjugates(phrases_out);

    phrases_out
}

/// Removes conjugates by mutating the vector.
/// **Requires** the phrases input to be sorted alphabetically.
/// It works by continuing on the last inner loop index from the prior outer item (conjugate),
/// avoiding going over the previous outer item's identical inner items more than once.
pub fn sans_conjugates<'a>(mut phrases: Vec<Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    let mut p_i: usize = 0;
    let mut for_removal: Vec<usize> = vec![];
    for conjugate in UNWANTED_CONJUGATES {
        // println!("pi {:1} {:2}", p_i, phrases.len());
        let mut has_matched: bool = false;
        for i in p_i..phrases.len() {
            // println!("pi inside xxx {:1} {:2} {:3}", p_i, i, has_matched);
            if phrases[i].len() != 1 {
                continue;
            }

            if phrases[i][0] == conjugate {
                for_removal.push(i);
                has_matched = true;
                continue;
            }

            if has_matched {
                p_i = i + 1;
                break;
            }
        }
    }

    for_removal.reverse();

    for i in for_removal {
        phrases.remove(i);
    }

    phrases
}
