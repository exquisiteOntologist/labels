use std::error::Error;

use gather::slicing::utf8_slice;

use crate::utils::strings::{str_first_char, str_get_len, str_last_char};

/// Cleans punctuation from the beginning and end of a word str.
/// It is able to handle characters of non-conforming byte sizes.
/// It doesn't panic.
pub fn clean_word<'a>(word: &'a str) -> Result<&'a str, Box<dyn Error>> {
    let mut word_clean: &str = word;
    while !word_clean.is_empty()
        && str_first_char(&word_clean).is_alphanumeric() == false
        && word_clean.chars().count() > 1
    {
        let word_len = str_get_len(word_clean);
        word_clean = match utf8_slice(word_clean, 1, word_len - 1) {
            Ok(v) => v,
            Err(_) => word_clean,
        }
        .trim();
    }
    while !word_clean.is_empty()
        && str_last_char(word_clean).is_alphanumeric() == false
        && word_clean.chars().count() > 1
    {
        if (word_clean.ends_with("s’")) && !word_clean.ends_with(".’") {
            break;
        }
        let word_len = str_get_len(word_clean);
        word_clean = match utf8_slice(word_clean, 0, word_len - 1) {
            Ok(v) => v,
            Err(_) => word_clean,
        }
        .trim();
    }
    // println!("{:?}", word_clean);
    Ok(word_clean)
}

pub fn word_without_extensions<'a>(word: &'a str) -> Result<&'a str, Box<dyn Error>> {
    if word.len() < 3 {
        return Ok(word);
    }
    let mut word_new: &str = word;
    let word_len: usize = str_get_len(word_new);
    if word_new.ends_with("'s") || word_new.ends_with("’s") {
        word_new = match utf8_slice(word_new, 0, word_len - 2) {
            Ok(v) => v,
            Err(_) => word_new,
        }
        .trim();
        return Ok(word_new);
    }
    if word_new.ends_with("s") && !word_new.ends_with("s") && !word_new.ends_with("is") {
        word_new = match utf8_slice(word_new, 0, word_len - 1) {
            Ok(v) => v,
            Err(_) => word_new,
        }
        .trim();
        return Ok(word_new);
    }
    Ok(word_new)
}
