use crate::utils::strings::str_last_char;
use crate::vocab::common::UNWANTED_CONJUGATES;

use super::prep::{clean_word, word_without_extensions};

/// From text extract individual words or phrases into a vector
pub fn phrase_extraction<'a>(text: &'a str) -> Vec<Vec<&'a str>> {
    // let words: Vec<&str> = text.split(&[' ', '[', ']', '(', ')', "”"]).collect();
    let words: Vec<&str> = text
        .split(|c: char| {
            c.is_whitespace()
                || ((c.is_ascii_punctuation() || !c.is_ascii_alphanumeric()) && c != '.')
        })
        .collect();
    let mut phrases_out: Vec<Vec<&str>> = vec![];
    let mut last_title_or_name: Vec<&str> = vec![];

    'words: for word in words {
        'word: {
            if word.chars().count() == 0 {
                break 'word;
            }

            if word.chars().count() < 2
                && word.chars().by_ref().any(|c| c.is_alphanumeric() == false)
            {
                break 'word;
            }

            let last_char = str_last_char(word);
            let last_in_sentence: bool = last_char == '.' || last_char == ',';

            let word_clean: &str = clean_word(word).unwrap_or(word);
            if word_clean.is_empty() {
                break 'word;
            }

            let word_sans_extensions: &str = word_without_extensions(word_clean).unwrap();
            phrases_out.push(vec![word_sans_extensions]);

            let is_title_case =
                !word_clean.is_empty() && word_clean.chars().nth(0).unwrap().is_uppercase();
            let conjoining = !is_title_case
                && last_title_or_name.len() > 1
                && (word_clean == "the" || word_clean == "of" || word_clean == "for");
            let add_to_phrase = is_title_case || conjoining;
            if add_to_phrase {
                last_title_or_name.push(word_clean);
            }
            if !last_in_sentence && add_to_phrase {
                continue 'words;
            }
        }

        // Pushing if not empty outside 'word means we may capture skipped phrase in theoretical scenario
        if last_title_or_name.len() != 0 {
            if last_title_or_name.len() > 1 {
                phrases_out.push(last_title_or_name.to_vec());
            }
            last_title_or_name.clear();
        }
    }

    // If the very last word was in a phrase, push it too
    if last_title_or_name.len() > 1 {
        phrases_out.push(last_title_or_name.to_vec());
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
